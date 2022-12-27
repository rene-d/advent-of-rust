#!/usr/bin/env python3
# https://adventofcode.com/2019/day/9

from pathlib import Path
import sys

sys.path.append("..")
from intcode.Intcode import Computer

filename = "test.txt" if len(sys.argv) > 1 and sys.argv[1] == "-t" else "input.txt"
data = Path(filename).read_text()

computer = Computer()
computer.load(data)

computer.input.append(1)
computer.run()
print(computer.output.popleft())

computer.input.append(2)
computer.run()
print(computer.output.popleft())
