#!/usr/bin/env python3

# --- Day 3: Binary Diagnostic ---
# https://adventofcode.com/2021/day/3

import atexit
import sys
import time

filename = ("test.txt" if sys.argv[1] == "-t" else sys.argv[1]) if len(sys.argv) > 1 else "input.txt"
data = list(map(str.strip, open(filename).readlines()))
if "--elapsed" in sys.argv:
    sys.argv.remove("--elapsed")
    start_time_ns = time.time_ns()
    atexit.register(lambda: print(f"elapsed: {(time.time_ns() - start_time_ns) / 1_000_000}ms"))


should_start_with = ""
for i in range(len(data[0])):
    one = 0
    nb = 0
    for value in data:
        if value.startswith(should_start_with):
            nb += 1
            if value[i] == "1":
                one += 1
                rate = value
    if one >= nb - one:
        should_start_with += "1"
    else:
        should_start_with += "0"
print("O2 ", rate, int(rate, 2))
o2 = int(rate, 2)


should_start_with = ""
for i in range(len(data[0])):
    one = 0
    nb = 0
    for value in data:
        if value.startswith(should_start_with):
            nb += 1
            if value[i] == "1":
                one += 1
            else:
                rate = value
    if one >= nb - one:
        should_start_with += "0"
    else:
        should_start_with += "1"
print("CO2", rate, int(rate, 2))
co2 = int(rate, 2)

print(o2 * co2)
