#!/usr/bin/env python3
# [Day 1: Inverse Captcha](https://adventofcode.com/2017/day/1)

import sys
from pathlib import Path

filename = ("test.txt" if sys.argv[1] == "-t" else sys.argv[1]) if len(sys.argv) > 1 else "input.txt"
data = Path(filename).read_text().strip()

data2 = data[1:] + data[0]
n = sum(int(a) for a, b in zip(data, data2) if a == b)
print(n)

half = len(data) // 2
data2 = data[half:] + data[:half]
n = sum(int(a) for a, b in zip(data, data2) if a == b)
print(n)
