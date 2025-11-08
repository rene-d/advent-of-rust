#!/usr/bin/env python3

import atexit
import sys
import time

# Day 1: Sonar Sweep
# https://adventofcode.com/2021/day/1

# read the input
filename = ("test.txt" if sys.argv[1] == "-t" else sys.argv[1]) if len(sys.argv) > 1 else "input.txt"
data = open(filename).readlines()
if "--elapsed" in sys.argv:
    sys.argv.remove("--elapsed")
    start_time_ns = time.time_ns()
    atexit.register(lambda: print(f"elapsed: {(time.time_ns() - start_time_ns) / 1_000_000}ms"))


data = list(map(int, data))

# --- Day 1: Sonar Sweep --- part one
n = 0
for i, j in zip(data, data[1:]):
    if i < j:
        n += 1
print(n)

# --- Day 1: Sonar Sweep --- part two
data = list(i + j + k for i, j, k in zip(data, data[1:], data[2:]))
n = 0
for i, j in zip(data, data[1:]):
    if i < j:
        n += 1
print(n)
