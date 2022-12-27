#!/usr/bin/env python3

from pathlib import Path
import sys
from collections import defaultdict, deque, namedtuple
import argparse

# opcodes
OPCODES = {
    # opcode: (short_name, arguments, comment),
    99: ("halt", 0, "halt"),  # https://adventofcode.com/2019/day/2
    1: ("add", 3, "addition"),
    2: ("mul", 3, "multiplication"),
    3: ("inp", 1, "input"),  # https://adventofcode.com/2019/day/5
    4: ("outp", 1, "output"),
    5: ("jnz", 2, "jump-if-true"),  # https://adventofcode.com/2019/day/5#part2
    6: ("jz", 2, "jump-if-false"),
    7: ("less", 3, "less than"),
    8: ("equal", 3, "equal"),
    9: ("relbase", 1, "incr/decr relative base offset"),  # https://adventofcode.com/2019/day/9
    0: ("nop", 0, "no operation"),  # custom
}

# address mode
POSITION_MODE = 0  # https://adventofcode.com/2019/day/5
IMMEDIATE_MODE = 1  # https://adventofcode.com/2019/day/5
RELATIVE_MODE = 2  # https://adventofcode.com/2019/day/9

Operand = namedtuple("operand", ["value", "addr"])


class Computer:
    def __init__(self):
        self.load("99")

    def load(self, data):
        if isinstance(data, list):
            self.program = data
        else:
            self.program = []
            for line in data.splitlines():
                # comments
                p = line.find("#")
                if p != -1:
                    line = line[:p]
                p = line.find(";")
                if p != -1:
                    line = line[:p]
                line = line.strip()
                self.program.extend(map(int, filter(lambda x: x, line.strip(",").split(","))))
        self.flush_io()

    def load_from(self, filename):
        self.load(Path(filename).read_text())

    def flush_io(self):
        self.input = deque()
        self.output = deque()

    def disasm(self, debugger=None, source=False):

        if debugger is not None:
            ip = debugger
            memory = self._code
        else:
            ip = 0
            memory = self.program

        while ip < len(memory):
            opcode = memory[ip]
            modes = (opcode // 100) % 10, (opcode // 1000) % 10, (opcode // 10000) % 10
            if opcode > 0:
                opcode %= 100

            if opcode not in OPCODES:
                instruction = f"unknown"
                comment = f"unknown opcode {memory[ip]}"
                operands = f"{memory[ip]}"
                n_args = 0
            else:
                instruction, n_args, comment = OPCODES[opcode]

                args = []
                for i in range(n_args):
                    if ip + 1 + i >= len(memory):
                        args.append("???")

                    elif modes[i] == POSITION_MODE:
                        args.append(f"[{memory[ip+1+i]}]")
                    elif modes[i] == IMMEDIATE_MODE:
                        args.append(f"{memory[ip+1+i]}")
                    elif modes[i] == RELATIVE_MODE:
                        args.append(f"(rel {memory[ip+1+i]})")
                    else:
                        raise ValueError

                match n_args:
                    case 0:
                        operands = ""
                    case 1:
                        operands = args[0]
                    case 2:
                        operands = args[0] + " to " + args[1]
                    case 3:
                        operands = args[0] + "," + args[1] + " to " + args[2]
                    case _:
                        raise ValueError

            if comment:
                comment = f"; {comment}"
            numbers = ",".join(map(str, memory[ip : ip + 1 + n_args])) + ","

            if source:
                line = f"{numbers:<20} ; {instruction:<12}{operands}"
            else:
                line = f"{ip:5d}  {numbers:<20} {instruction:<16}{operands:<20}{comment}"

            print(line)

            ip += 1 + n_args

            if debugger is not None:
                break

    def run(self, **kwargs):
        self.start(**kwargs)
        return self.resume()

    def start(self, output_mode="buffered", debug=False):
        assert output_mode in ["buffered", "direct", "ascii", "yield"]
        self._code = self.program.copy()  # code segment
        self._bss = defaultdict(lambda: 0)  # data segment
        self._ip = 0
        self._relbase = 0
        self._output_mode = output_mode
        self._state = "pause"
        self._debug = debug
        return self._state

    def resume(self):
        assert self._state in ["yield", "read", "pause"]
        self._ip, self._state = self._run()
        return self._state

    def _peek(self, addr):
        if 0 <= addr < len(self._code):
            value = self._code[addr]
        else:
            value = self._bss[addr]
        # print(f"_peek [{addr}] -> {value}")
        return value

    def _poke(self, addr, value):
        # print(f"_poke [{addr}] <- {value}")
        if 0 <= addr < len(self._code):
            self._code[addr] = value
        else:
            self._bss[addr] = value

    def _run(self):
        ip = self._ip

        assert self._state in ["start", "pause", "yield", "read"]
        self._state = ""

        while ip < len(self._code):

            if self._debug:
                print()
                self.disasm(ip)

            opcode = self._code[ip]

            modes = (opcode // 100) % 10, (opcode // 1000) % 10, (opcode // 10000) % 10
            if opcode > 0:
                opcode %= 100

            if opcode not in OPCODES:
                return ip + 1, "bad instruction"

            _, n_args, _ = OPCODES[opcode]

            args = []  # (value, address) of arguments
            for i in range(n_args):
                arg = self._code[ip + 1 + i]

                if modes[i] == POSITION_MODE:
                    args.append(Operand(self._peek(arg), arg))
                elif modes[i] == IMMEDIATE_MODE:
                    args.append(Operand(arg, None))
                elif modes[i] == RELATIVE_MODE:
                    args.append(Operand(self._peek(arg + self._relbase), arg + self._relbase))
                else:
                    raise ValueError

            current_ip = ip
            ip += 1 + n_args

            match opcode:
                case 0:  # nop
                    pass

                case 99:  # halt
                    return ip, "halted"

                case 1:  # addition
                    self._poke(args[2].addr, args[0].value + args[1].value)

                case 2:  # multiplication
                    self._poke(args[2].addr, args[0].value * args[1].value)

                case 3:  # input
                    # if no buffered input, program must suspend
                    if not self.input:
                        return current_ip, "read"
                    self._poke(args[0].addr, self.input.popleft())

                case 4:  # output
                    if self._output_mode == "direct":
                        print(args[0].value)

                    elif self._output_mode == "ascii":
                        if args[0].value < 256:
                            sys.stdout.write(chr(args[0].value))
                        else:
                            sys.stdout.write(f"#{args[0].value};")
                        sys.stdout.flush()

                    else:
                        self.output.append(args[0].value)
                        if self._output_mode == "yield":
                            return ip, "yield"

                case 5:  # jump-if-true
                    if args[0].value != 0:
                        ip = args[1].value

                case 6:  # jump-if-false
                    if args[0].value == 0:
                        ip = args[1].value

                case 7:  # less than
                    if args[0].value < args[1].value:
                        self._poke(args[2].addr, 1)
                    else:
                        self._poke(args[2].addr, 0)
                case 8:  # equal
                    if args[0].value == args[1].value:
                        self._poke(args[2].addr, 1)
                    else:
                        self._poke(args[2].addr, 0)

                case 9:  # relative base offset
                    self._relbase += args[0].value

                case _:
                    raise NotImplemented

            if self._debug:
                for i, arg in enumerate(args):
                    print(f"  args[{i}] =  {arg.value}   @{arg.addr}")
                # input(f"{list(self.output)}> ")

        return ip, "exited"

    def dump(self):
        print(".text")
        for i in range(0, len(self._code)):
            if self.program[i] != self._code[i]:
                print(f"{i:5}   {self.program[i]} -> {self._code[i]}")
        print(".bss")
        for i in sorted(self._bss.keys()):
            print(f"{i:5}   {self._bss[i]}")


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("-D", "--debug", action="store_true")
    parser.add_argument("-d", "--disasm", action="store_true")
    parser.add_argument("-s", "--source", action="store_true")
    parser.add_argument("-a", "--ascii", action="store_true")
    parser.add_argument("-m", "--memory", action="store_true", help="show memory on exit")
    parser.add_argument("filename")
    args = parser.parse_args()

    computer = Computer()
    computer.load_from(args.filename)

    if args.disasm:
        computer.disasm()
    elif args.source:
        computer.disasm(source=True)
    elif args.ascii:
        computer.run(output_mode="ascii")
    else:
        state = computer.run(debug=args.debug)
        while state == "read":
            print(list(computer.output))
            value = input("input> ")
            if value.strip() == "":
                break
            computer.input.extend(map(int, value.split(",")))
            state = computer.resume()
        print(state, list(computer.output))
        if args.memory:
            computer.dump()


if __name__ == "__main__":
    main()
