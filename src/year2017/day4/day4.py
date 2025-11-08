#!/usr/bin/env python3
# [Day 4: High-Entropy Passphrases](https://adventofcode.com/2017/day/4)

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

n = 0
for line in lines:
    a = line.split()
    if len(a) == len(set(a)):
        n += 1
print(n)

n = 0
for line in lines:
    a = list(map(lambda x: "".join(sorted(x)), line.split()))
    if len(a) == len(set(a)):
        n += 1
print(n)
