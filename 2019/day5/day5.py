#!/usr/bin/env python3
# https://adventofcode.com/2019/day/5

from pathlib import Path
import sys
from copy import deepcopy
from collections import defaultdict, deque
import re


class Puzzle:
    def __init__(self, filename):
        data = Path(filename).read_text()
        self.program = list(map(int, data.split(",")))

        self.reset_io()

    def reset_io(self):
        self.input = deque()
        self.output = deque()

    def run(self, memory=None, dump=False):
        memory = memory or self.program.copy()
        ip = 0

        while True:
            opcode = memory[ip]

            mode_3 = (opcode // 10000) % 10
            mode_2 = (opcode // 1000) % 10
            mode_1 = (opcode // 100) % 10

            match opcode % 100:
                case 99:  # halt
                    ip += 1
                    break

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

    def part1(self):
        self.reset_io()
        self.input.append(1)
        self.run()
        return self.output.pop()

    def part2(self):
        self.reset_io()
        self.input.append(5)
        self.run()
        return self.output.pop()


def main():
    filename = "test.txt" if len(sys.argv) > 1 and sys.argv[1] == "-t" else "input.txt"

    puzzle = Puzzle(filename)
    print(puzzle.part1())
    print(puzzle.part2())


if __name__ == "__main__":
    main()
