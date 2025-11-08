#!/usr/bin/env python3
# [Day 1: Inverse Captcha](https://adventofcode.com/2017/day/1)

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


data2 = data[1:] + data[0]
n = sum(int(a) for a, b in zip(data, data2) if a == b)
print(n)

half = len(data) // 2
data2 = data[half:] + data[:half]
n = sum(int(a) for a, b in zip(data, data2) if a == b)
print(n)
