#!/usr/bin/env python3
# [Day 9: Sensor Boost](https://adventofcode.com/2019/day/9)

import atexit
import sys
import time
from pathlib import Path

filename = ("test.txt" if sys.argv[1] == "-t" else sys.argv[1]) if len(sys.argv) > 1 else "input.txt"
data = Path(filename).read_text()
if "--elapsed" in sys.argv:
    sys.argv.remove("--elapsed")
    start_time_ns = time.time_ns()
    atexit.register(lambda: print(f"elapsed: {(time.time_ns() - start_time_ns) / 1_000_000}ms"))


sys.path.append(Path(__file__).parent.parent.as_posix())
from intcode.Intcode import Computer  # noqa

computer = Computer()
computer.load(data)

computer.input.append(1)
computer.run()
print(computer.output.popleft())

computer.input.append(2)
computer.run()
print(computer.output.popleft())
