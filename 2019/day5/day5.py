#!/usr/bin/env python3
# https://adventofcode.com/2019/day/5

import sys

sys.path.append("..")
from intcode.Intcode import Computer


filename = "test.txt" if len(sys.argv) > 1 and sys.argv[1] == "-t" else "input.txt"


computer = Computer()
computer.load_from(filename)

# part 1
computer.input.append(1)
computer.run()
print(computer.output.pop())

# part 2
computer.input.append(5)
computer.run()
print(computer.output.pop())