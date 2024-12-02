#!/usr/bin/env python3
# [Day 2: Red-Nosed Reports](https://adventofcode.com/2024/day/2)

from pathlib import Path
import sys

verbose = "-v" in sys.argv
if verbose:
    sys.argv.remove("-v")
self_tests = "-T" in sys.argv
if self_tests:
    sys.argv.remove("-T")
filename = ("test.txt" if sys.argv[1] == "-t" else sys.argv[1]) if len(sys.argv) > 1 else "input.txt"
data = Path(filename).read_text().strip()
lines = data.splitlines()


def is_safe(v):
    return all(1 <= a - b <= 3 for a, b in zip(v, v[1:])) or all(1 <= b - a <= 3 for a, b in zip(v, v[1:]))


safe = 0
for line in lines:
    v = list(map(int, line.split()))
    safe += is_safe(v)
print(safe)

safe = 0
for line in lines:
    v = list(map(int, line.split()))
    safe += is_safe(v) or any(is_safe(v[:i] + v[i + 1 :]) for i in range(len(v)))
print(safe)
