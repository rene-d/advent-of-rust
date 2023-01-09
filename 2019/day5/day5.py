#!/usr/bin/env python3
# https://adventofcode.com/2019/day/5

import sys
from pathlib import Path

sys.path.append(Path(__file__).parent.parent.as_posix())
from intcode.Intcode import Computer


filename = ("test.txt" if sys.argv[1] == "-t" else sys.argv[1]) if len(sys.argv) > 1 else "input.txt"


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
