#!/usr/bin/env python3
# [Day 4: Ceres Search](https://adventofcode.com/2024/day/4)

from pathlib import Path
from copy import deepcopy
from collections import defaultdict, deque, namedtuple, Counter
import sys, re, math, itertools, time
from functools import reduce
import re
import unittest

verbose = "-v" in sys.argv
if verbose:
    sys.argv.remove("-v")
self_tests = "-T" in sys.argv
if self_tests:
    sys.argv.remove("-T")
filename = ("test.txt" if sys.argv[1] == "-t" else sys.argv[1]) if len(sys.argv) > 1 else "input.txt"
data = Path(filename).read_text().strip()
lines = data.splitlines()

sy = len(lines)
sx = len(lines[0])


def letters(x, y):
    if 0 <= x < sx and 0 <= y < sy:
        return lines[y][x]
    return "."


n = 0
for y in range(sy):
    for x in range(sx):
        if letters(x, y) == "X":
            n += "".join(letters(x, y + i) for i in range(1, 4)) == "MAS"
            n += "".join(letters(x + i, y) for i in range(1, 4)) == "MAS"
            n += "".join(letters(x + i, y + i) for i in range(1, 4)) == "MAS"
            n += "".join(letters(x + i, y - i) for i in range(1, 4)) == "MAS"
            n += "".join(letters(x, y - i) for i in range(1, 4)) == "MAS"
            n += "".join(letters(x - i, y) for i in range(1, 4)) == "MAS"
            n += "".join(letters(x - i, y + i) for i in range(1, 4)) == "MAS"
            n += "".join(letters(x - i, y - i) for i in range(1, 4)) == "MAS"
print(n)


n = 0
for y in range(sy):
    for x in range(sx):
        if letters(x, y) == "A":
            w = letters(x - 1, y - 1) + letters(x + 1, y + 1) + letters(x + 1, y - 1) + letters(x - 1, y + 1)
            n += w in ("MSMS", "MSSM", "SMMS", "SMSM")
print(n)
