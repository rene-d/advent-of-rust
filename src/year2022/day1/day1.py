#!/usr/bin/env python3
# [Day 1: Calorie Counting](https://adventofcode.com/2022/day/1)

import atexit
import sys
import time

filename = ("test.txt" if sys.argv[1] == "-t" else sys.argv[1]) if len(sys.argv) > 1 else "input.txt"
lines = open(filename).readlines()
if "--elapsed" in sys.argv:
    sys.argv.remove("--elapsed")
    start_time_ns = time.time_ns()
    atexit.register(lambda: print(f"elapsed: {(time.time_ns() - start_time_ns) / 1_000_000}ms"))

lines.append("")


energy = 0
calories = []
for line in lines:
    line = line.strip()
    if not line:
        calories.append(energy)
        energy = 0
    else:
        energy += int(line)

# part one
print(max(calories))

# part two
calories = sorted(calories, reverse=True)
print(sum(calories[0:3]))
