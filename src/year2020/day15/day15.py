#!/usr/bin/env python3
# [Day 15: Rambunctious Recitation](https://adventofcode.com/2020/day/15)

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


nums = list(map(int, data.split(",")))

last_spoken = {}
last_last_spoken = {}

for turn, n in enumerate(nums, 1):
    first_spoken = n not in last_spoken
    if n in last_spoken:
        last_last_spoken[n] = last_spoken[n]
    last_spoken[n] = turn
    last = n

turn = len(nums) + 1
while True:
    if first_spoken:
        n = 0
    else:
        n = last_spoken[last] - last_last_spoken[n]

    if turn == 2020:
        print(n)  # part 1

    if turn == 30_000_000:
        print(n)  # part 2
        break

    first_spoken = n not in last_spoken
    if n in last_spoken:
        last_last_spoken[n] = last_spoken[n]
    last_spoken[n] = turn
    turn += 1
    last = n
