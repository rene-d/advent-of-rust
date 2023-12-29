#!/usr/bin/env python3
# https://adventofcode.com/2023/day/9

import sys
from functools import reduce
from pathlib import Path

filename = ("test.txt" if sys.argv[1] == "-t" else sys.argv[1]) if len(sys.argv) > 1 else "input.txt"
data = Path(filename).read_text().strip()
lines = data.splitlines()

part1 = 0
part2 = 0

for line in lines:
    line = list(map(int, line.split()))
    diffs = [line]

    while True:
        history = []
        for a, b in zip(line, line[1:]):
            history.append(b - a)

        if set(history) == set([0]):
            break

        diffs.append(history)
        line = history

    right = sum(d[-1] for d in diffs)
    left = reduce(lambda left, d: d[0] - left, reversed(diffs), 0)

    part1 += right
    part2 += left

print(part1)
print(part2)
