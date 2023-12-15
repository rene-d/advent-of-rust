#!/usr/bin/env python3
# https://adventofcode.com/2020/day/2

import re
import sys
from pathlib import Path

filename = ("test.txt" if sys.argv[1] == "-t" else sys.argv[1]) if len(sys.argv) > 1 else "input.txt"
data = Path(filename).read_text().strip()
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
