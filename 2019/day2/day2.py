#!/usr/bin/env python3
# https://adventofcode.com/2019/day/2

from pathlib import Path
import sys
from copy import deepcopy
from collections import defaultdict, deque
import re


class Puzzle:
    def __init__(self, filename):
        data = Path(filename).read_text()
        self.program = list(map(int, data.split(",")))

    def run(self, memory=None, dump=False):
        memory = memory or self.program.copy()
        ip = 0
        while True:
            if dump:
                self.dump(memory, ip, True)

            if memory[ip] == 99:
                break

            assert ip + 4 <= len(memory)
            opcode, noun, verb, result = memory[ip : ip + 4]
            match opcode:
                case 1:  # addition
                    memory[result] = memory[noun] + memory[verb]
                case 2:  # multiplication
                    memory[result] = memory[noun] * memory[verb]
                case _:
                    assert False
            ip += 4

    def dump(self, memory=None, ip=0, single=False):
        memory = memory or self.program
        while True:
            opcode = memory[ip]
            match opcode:
                case 99:
                    n = 1
                    instr = "halt"
                    comment = ""
                case 1:
                    noun, verb, result = memory[ip + 1 : ip + 4]
                    n = 4
                    instr = f"add [{noun}],[{verb}] to [{result}]"
                    comment = f"{memory[noun]} + {memory[verb]} = {memory[noun] + memory[verb]}"
                case 2:
                    noun, verb, result = memory[ip + 1 : ip + 4]
                    n = 4
                    instr = f"mul [{noun}],[{verb}] to [{result}]"
                    comment = f"{memory[noun]} * {memory[verb]} = {memory[noun] * memory[verb]}"
                case _:
                    instr = f"unknown opcode"
                    n = 1

            x = ",".join(map(str, memory[ip : ip + n])) + ","
            y=f"[{ip}]"
            print(f"{y:>5}  {x:<15} {instr:<30} ; {comment}")
            ip += n

            if single:
                break

    def part1(self):
        memory = self.program.copy()

        memory[1] = 12
        memory[2] = 2

        self.run(memory)

        return memory[0]

    def part2(self):
        for noun in range(100):
            for verb in range(100):
                memory = self.program.copy()
                memory[1] = noun
                memory[2] = verb
                self.run(memory)

                if memory[0] == 1969_07_20:
                    return noun * 100 + verb

        pass


def main():
    filename = "test.txt" if len(sys.argv) > 1 and sys.argv[1] == "-t" else "input.txt"

    if filename=="test.txt":
        puzzle = Puzzle(filename)
        puzzle.run(dump=True)
    else:
        puzzle = Puzzle(filename)
        print(puzzle.part1())
        print(puzzle.part2())



if __name__ == "__main__":
    main()
