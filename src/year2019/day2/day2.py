#!/usr/bin/env python3
# [Day 2: 1202 Program Alarm](https://adventofcode.com/2019/day/2)

import atexit
import sys
import time
from pathlib import Path


class Puzzle:
    def __init__(self, filename):
        data = Path(filename).read_text()
        self.program = list(map(int, data.split(",")))

    def run(self, memory=None):
        memory = memory or self.program.copy()
        ip = 0
        while True:
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
    filename = ("test.txt" if sys.argv[1] == "-t" else sys.argv[1]) if len(sys.argv) > 1 else "input.txt"

    if "--elapsed" in sys.argv:
        sys.argv.remove("--elapsed")
        start_time_ns = time.time_ns()
        atexit.register(lambda: print(f"elapsed: {(time.time_ns() - start_time_ns) / 1_000_000}ms"))

    puzzle = Puzzle(filename)
    print(puzzle.part1())
    print(puzzle.part2())


if __name__ == "__main__":
    main()
