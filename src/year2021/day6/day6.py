#!/usr/bin/env python3

# Day 6: Lanternfish
# https://adventofcode.com/2021/day/6

import atexit
import sys
import time

filename = ("test.txt" if sys.argv[1] == "-t" else sys.argv[1]) if len(sys.argv) > 1 else "input.txt"
data = open(filename).readlines()
if "--elapsed" in sys.argv:
    sys.argv.remove("--elapsed")
    start_time_ns = time.time_ns()
    atexit.register(lambda: print(f"elapsed: {(time.time_ns() - start_time_ns) / 1_000_000}ms"))


timers = list(map(int, data[0].split(",")))

# step 1 (naive)
for k in range(80):
    n = len(timers)
    for i in range(n):
        timer = timers[i] - 1
        if timer == -1:
            timer = 6
            timers.append(8)
        timers[i] = timer
print(len(timers))

# step 2
timers = [0] * 9
for timer in list(map(int, data[0].split(","))):
    timers[timer] += 1
for day in range(256):
    new = [0] * 9
    for i, timer in enumerate(timers):
        if i == 0:
            new[6] += timer
            new[8] += timer
        else:
            new[i - 1] += timer
    timers = new
print(sum(timers))
