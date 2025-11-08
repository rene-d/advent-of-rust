#!/usr/bin/env python3
# [Day 3: Toboggan Trajectory](https://adventofcode.com/2020/day/3)

import atexit
import sys
import time
from pathlib import Path

filename = ("test.txt" if sys.argv[1] == "-t" else sys.argv[1]) if len(sys.argv) > 1 else "input.txt"
data = Path(filename).read_text().strip()
if "--elapsed" in sys.argv:
    sys.argv.remove("--elapsed")
    start_time_ns = time.time_ns()
    atexit.register(lambda: print(f"elapsed: {(time.time_ns() - start_time_ns) / 1_000_000}ms"))


lines = data.splitlines()
size_x = len(lines[0])
size_y = len(lines)


def trees(slope_x, slope_y):
    x, y = 0, 0
    n = 0
    while y < size_y:
        if lines[y][x] == "#":
            n += 1
        x = (x + slope_x) % size_x
        y += slope_y
    return n


print(trees(3, 1))

print(trees(1, 1) * trees(3, 1) * trees(5, 1) * trees(7, 1) * trees(1, 2))
