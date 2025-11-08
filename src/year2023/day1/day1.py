#!/usr/bin/env python3
# [Day 1: Trebuchet?!](https://adventofcode.com/2023/day/1)

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


# parse input
lines = data.splitlines()


# part 1
a = 0
for line in lines:
    line = line.strip()
    line = [c for c in line if c >= "1" and c <= "9"]
    a += int(line[0]) * 10 + int(line[-1])
print(a)


# part 2
a = 0
for line in lines:
    line = line.strip()

    i = 0
    first = None
    while i < len(line):
        if line[i:].startswith("one"):
            first = 1
            break
        elif line[i:].startswith("two"):
            first = 2
            break
        elif line[i:].startswith("three"):
            first = 3
            break
        elif line[i:].startswith("four"):
            first = 4
            break
        elif line[i:].startswith("five"):
            first = 5
            break
        elif line[i:].startswith("six"):
            first = 6
            break
        elif line[i:].startswith("seven"):
            first = 7
            break
        elif line[i:].startswith("eight"):
            first = 8
            break
        elif line[i:].startswith("nine"):
            first = 9
            break
        elif "1" <= line[i] <= "9":
            first = int(line[i])
            break
        else:
            i += 1

    n = first

    first = None
    i = len(line) - 1
    while i >= 0:
        if line[i:].startswith("one"):
            first = 1
            break
        elif line[i:].startswith("two"):
            first = 2
            break
        elif line[i:].startswith("three"):
            first = 3
            break
        elif line[i:].startswith("four"):
            first = 4
            break
        elif line[i:].startswith("five"):
            first = 5
            break
        elif line[i:].startswith("six"):
            first = 6
            break
        elif line[i:].startswith("seven"):
            first = 7
            break
        elif line[i:].startswith("eight"):
            first = 8
            break
        elif line[i:].startswith("nine"):
            first = 9
            break
        elif "1" <= line[i] <= "9":
            first = int(line[i])
            break
        else:
            i -= 1

    n = n * 10 + first

    a += n

print(a)
