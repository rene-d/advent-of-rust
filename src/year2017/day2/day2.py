#!/usr/bin/env python3
# [Day 2: Corruption Checksum](https://adventofcode.com/2017/day/2)

import atexit
import itertools
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


n = 0
for line in lines:
    values = list(map(int, line.split()))
    n += max(values) - min(values)
print(n)


n = 0
for line in lines:
    values = list(map(int, line.split()))
    for a, b in itertools.product(values, values):
        if a > b and a % b == 0:
            n += a // b
            break
print(n)
