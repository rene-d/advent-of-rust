#!/usr/bin/env python3
# https://adventofcode.com/2017/day/2

import itertools
import sys
from pathlib import Path

filename = ("test.txt" if sys.argv[1] == "-t" else sys.argv[1]) if len(sys.argv) > 1 else "input.txt"
data = Path(filename).read_text().strip()
lines = data.splitlines()


n = 0
for line in lines:
    values = list(map(int, line.split()))
    n += max(values) - min(values)
print(n)


n = 0
for line in lines:
    values = list(map(int, line.split()))
    for a, b in itertools.product(values, values):
        if a > b and a % b == 0:
            n += a // b
            break
print(n)
