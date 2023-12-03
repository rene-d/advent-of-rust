#!/usr/bin/env python3
# https://adventofcode.com/2020/day/3

from pathlib import Path
from copy import deepcopy
from collections import defaultdict, deque, namedtuple
import sys, re, math, itertools, time
from functools import reduce
import re

filename = ("test.txt" if sys.argv[1] == "-t" else sys.argv[1]) if len(sys.argv) > 1 else "input.txt"
data = Path(filename).read_text().strip()
lines = data.splitlines()
size_x = len(lines[0])
size_y = len(lines)


def trees(slope_x, slope_y):
    x, y = 0, 0
    n = 0
    while y < size_y:
        if lines[y][x] == "#":
            n += 1
        x = (x + slope_x) % size_x
        y += slope_y
    return n


print(trees(3, 1))

print(trees(1, 1) * trees(3, 1) * trees(5, 1) * trees(7, 1) * trees(1, 2))
