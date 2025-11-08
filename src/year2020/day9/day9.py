#!/usr/bin/env python3
# [Day 9: Encoding Error](https://adventofcode.com/2020/day/9)

import atexit
import sys
import time
from pathlib import Path

verbose = "-v" in sys.argv
if verbose:
    sys.argv.remove("-v")
filename = ("test.txt" if sys.argv[1] == "-t" else sys.argv[1]) if len(sys.argv) > 1 else "input.txt"
data = Path(filename).read_text().strip()
if "--elapsed" in sys.argv:
    sys.argv.remove("--elapsed")
    start_time_ns = time.time_ns()
    atexit.register(lambda: print(f"elapsed: {(time.time_ns() - start_time_ns) / 1_000_000}ms"))


numbers = list(map(int, data.splitlines()))

# part 1
for i in range(25, len(numbers)):
    invalid = numbers[i]
    found = False
    for a in range(i - 25, i):
        for b in range(i - 25, i):
            if numbers[a] + numbers[b] == invalid:
                found = True
                break
        if found:
            break
    if not found:
        break
print(invalid)

# part 2
weakness = 0
for i in range(len(numbers)):
    acc = 0
    for j in range(i, len(numbers)):
        acc += numbers[j]
        if acc == invalid:
            weakness = min(numbers[i : j + 1]) + max(numbers[i : j + 1])
            break
        if acc > invalid:
            break
    if weakness:
        break
print(weakness)
