#!/usr/bin/env python3
# https://adventofcode.com/2019/day/7

from pathlib import Path
import sys
from copy import deepcopy
from collections import defaultdict, deque
import re
import itertools

filename = "test.txt" if len(sys.argv) > 1 and sys.argv[1] == "-t" else "input.txt"
data = Path(filename).read_text()
lines = data.splitlines()


class Intcode:
    def __init__(self):
        self.load("99")

    def load(self, data):
        if isinstance(data, list):
            self.program = data
        else:
            self.program = list(map(int, data.split(",")))
        self.reset_io()

    def load_from(self, filename):
        self.load(Path(filename).read_text())

    def reset_io(self):
        self.input = deque()
        self.output = deque()

    def run(self, memory=None, yield_on_output=False, break_on_enter=False):
        self._memory = memory or self.program.copy()
        self._ip = 0
        self._yield_on_output = yield_on_output
        if break_on_enter:
            self._state = "pause"
        else:
            self._state = "start"
            self._run()
        return self._state

    def resume(self):
        assert self._state in ["yield", "read", "pause"]
        self._run()
        return self._state

    def _run(self):
        memory = self._memory
        ip = self._ip

        assert self._state in ["start", "pause", "yield", "read"]
        self._state = ""

        while ip < len(memory):
            opcode = memory[ip]

            mode_3 = (opcode // 10000) % 10
            mode_2 = (opcode // 1000) % 10
            mode_1 = (opcode // 100) % 10

            match opcode % 100:
                case 99:  # halt
                    ip += 1
                    self._state = "halted"
                    self._ip = ip
                    return

                case 1:  # addition
                    length = 4
                    assert ip + 4 <= len(memory)
                    noun, verb, result = memory[ip + 1 : ip + 4]
                    arg1 = noun if mode_1 == 1 else memory[noun]
                    arg2 = verb if mode_2 == 1 else memory[verb]
                    assert mode_3 == 0
                    memory[result] = arg1 + arg2
                    ip += length

                case 2:  # multiplication
                    length = 4
                    assert ip + 4 <= len(memory)
                    assert mode_3 == 0
                    noun, verb, result = memory[ip + 1 : ip + 4]
                    arg1 = noun if mode_1 == 1 else memory[noun]
                    arg2 = verb if mode_2 == 1 else memory[verb]
                    memory[result] = arg1 * arg2
                    ip += length

                case 3:  # input

                    if not self.input:
                        self._state = "read"
                        self._ip = ip
                        return

                    length = 2
                    assert ip + 2 <= len(memory)
                    assert mode_1 == 0
                    noun = memory[ip + 1]
                    memory[noun] = self.input.popleft()
                    ip += length

                case 4:  # output
                    length = 2
                    assert ip + 2 <= len(memory)
                    noun = memory[ip + 1]
                    arg1 = noun if mode_1 == 1 else memory[noun]
                    self.output.append(arg1)
                    ip += length

                    if self._yield_on_output:
                        self._state = "yield"
                        self._ip = ip
                        return

                case 5:  # jump-if-true
                    length = 3
                    assert ip + length <= len(memory)
                    noun, verb = memory[ip + 1 : ip + 3]
                    arg1 = noun if mode_1 == 1 else memory[noun]
                    arg2 = verb if mode_2 == 1 else memory[verb]
                    if arg1 != 0:
                        ip = arg2
                    else:
                        ip += length

                case 6:  # jump-if-false
                    length = 3
                    assert ip + length <= len(memory)
                    noun, verb = memory[ip + 1 : ip + 3]
                    arg1 = noun if mode_1 == 1 else memory[noun]
                    arg2 = verb if mode_2 == 1 else memory[verb]
                    if arg1 == 0:
                        ip = arg2
                    else:
                        ip += length

                case 7:  # less than
                    length = 4
                    assert ip + length <= len(memory)
                    assert mode_3 == 0
                    noun, verb, result = memory[ip + 1 : ip + 4]
                    arg1 = noun if mode_1 == 1 else memory[noun]
                    arg2 = verb if mode_2 == 1 else memory[verb]
                    if arg1 < arg2:
                        memory[result] = 1
                    else:
                        memory[result] = 0
                    ip += length

                case 8:  # equal
                    length = 4
                    assert ip + length <= len(memory)
                    assert mode_3 == 0
                    noun, verb, result = memory[ip + 1 : ip + 4]
                    arg1 = noun if mode_1 == 1 else memory[noun]
                    arg2 = verb if mode_2 == 1 else memory[verb]
                    if arg1 == arg2:
                        memory[result] = 1
                    else:
                        memory[result] = 0
                    ip += length

                case _:
                    raise NotImplemented

        self._state = "exited"
        self._ip = ip


def run_amplifiers(phases):
    input_signal = 0
    for phase in phases:
        amp.input.append(phase)  # phase setting
        amp.input.append(input_signal)  # input signal
        amp.run()
        input_signal = amp.output.popleft()
    return input_signal


amp = Intcode()

# tests from puzzle
amp.load("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0")
assert run_amplifiers([4, 3, 2, 1, 0]) == 43210

amp.load("3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0")
assert run_amplifiers([0, 1, 2, 3, 4]) == 54321

amp.load("3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0")
assert run_amplifiers([1, 0, 4, 3, 2]) == 65210

# part 1
amp.load_from("input.txt")
print(max(run_amplifiers(phases) for phases in itertools.permutations(range(5))))


# part 2
def run_feedback(program, phase):
    amps = []
    for i in range(5):
        amp = Intcode()
        amp.load(program)
        amp.run(yield_on_output=True, break_on_enter=True)
        amps.append(amp)

    input_signal = 0

    for k in range(11):
        for i in range(5):
            if k == 0:
                amps[i].input.append(phase[i])
            amps[i].input.append(input_signal)
            state = amps[i].resume()
            if state == "halted":
                break
            input_signal = amps[i].output.popleft()
            assert len(amps[i].input) == 0
            assert len(amps[i].output) == 0
            # print("loop",k,"amp", i, state, "signal:", input_signal)

        if state == "halted":
            break

    return input_signal


# part2("3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5", phase=[9, 8, 7, 6, 5])

# part2(
#     "3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10",
#     phase=[9, 7, 8, 5, 6],
# )

print(max(run_feedback(amp.program, phases) for phases in itertools.permutations(range(5, 10))))
