#!/usr/bin/env python3
# https://adventofcode.com/2019/day/4

from pathlib import Path
import sys
from copy import deepcopy
from collections import defaultdict, deque
import re

filename = "test.txt" if len(sys.argv) > 1 and sys.argv[1] == "-t" else "input.txt"
data = Path(filename).read_text()

a, b = map(int, data.split("-"))

part1 = 0
for n in range(a, b + 1):
    s = str(n)
    ok = 1
    same_adj = 0
    for i, j in zip(s, s[1:]):
        if j < i:
            ok = 0
            break
        if i == j:
            same_adj = 1
    part1 += same_adj * ok
print(part1)


part2 = 0
for n in range(a, b + 1):
    s = str(n)
    for i, j in zip(s, s[1:]):
        if j < i:
            break
    else:
        freq = [0] * 10
        for c in s:
            freq[int(c)] += 1
        if 2 not in freq:
            continue

        part2 += 1
print(part2)
