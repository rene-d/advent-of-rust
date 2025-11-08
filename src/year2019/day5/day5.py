#!/usr/bin/env python3
# [Day 5: Sunny with a Chance of Asteroids](https://adventofcode.com/2019/day/5)

import atexit
import sys
import time
from pathlib import Path

filename = ("test.txt" if sys.argv[1] == "-t" else sys.argv[1]) if len(sys.argv) > 1 else "input.txt"
if "--elapsed" in sys.argv:
    sys.argv.remove("--elapsed")
    start_time_ns = time.time_ns()
    atexit.register(lambda: print(f"elapsed: {(time.time_ns() - start_time_ns) / 1_000_000}ms"))


sys.path.append(Path(__file__).parent.parent.as_posix())
from intcode.Intcode import Computer  # noqa

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
