#!/usr/bin/env python3
# [Day 1: Historian Hysteria](https://adventofcode.com/2024/day/1)

import sys
from collections import Counter
from pathlib import Path

verbose = "-v" in sys.argv
if verbose:
    sys.argv.remove("-v")
self_tests = "-T" in sys.argv
if self_tests:
    sys.argv.remove("-T")
filename = ("test.txt" if sys.argv[1] == "-t" else sys.argv[1]) if len(sys.argv) > 1 else "input.txt"
data = Path(filename).read_text().strip()
lines = data.splitlines()

# read input
left = []
right = []
for line in lines:
    a, b = map(int, line.split())
    left.append(a)
    right.append(b)
left = sorted(left)
right = sorted(right)

# part 1
print(sum(abs(a - b) for a, b in zip(left, right)))

# part 2
right = Counter(right)
print(sum(a * right.get(a, 0) for a in left))
