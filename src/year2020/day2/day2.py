#!/usr/bin/env python3
# [Day 2: Password Philosophy](https://adventofcode.com/2020/day/2)

import atexit
import re
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

# part 1
valid = 0
for line in lines:
    min, max, letter, password = re.match(r"(\d+)\-(\d+) (\w): (\w+)", line).groups()

    min, max = int(min), int(max)
    if min <= password.count(letter) <= max:
        valid += 1
print(valid)

# part 2
valid = 0
for line in lines:
    a, b, letter, password = re.match(r"(\d+)\-(\d+) (\w): (\w+)", line).groups()
    a = int(a)
    b = int(b)
    if (password[a - 1] == letter and password[b - 1] != letter) or (
        password[a - 1] != letter and password[b - 1] == letter
    ):
        valid += 1
print(valid)
