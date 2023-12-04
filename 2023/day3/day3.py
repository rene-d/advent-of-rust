#!/usr/bin/env python3
# https://adventofcode.com/2023/day/3

import sys
from collections import defaultdict
from pathlib import Path

filename = ("test.txt" if sys.argv[1] == "-t" else sys.argv[1]) if len(sys.argv) > 1 else "input.txt"
data = Path(filename).read_text().strip()
lines = data.splitlines()

sx = len(lines[0])
sy = len(lines)


def has_symbol(x0, x1, y):
    for yy in range(y - 1, y + 2):
        for xx in range(x0 - 1, x1 + 1):
            if yy == y and x0 <= xx < x1:
                continue
            if 0 <= xx < sx and 0 <= yy < sy:
                c = lines[yy][xx]
                if c != "." and not c.isdigit():
                    return c, yy, xx
    return None


gears = defaultdict(list)
part1 = 0
for y in range(sy):
    x = 0
    while x < sx:
        n = 0

        x0 = x
        while x < sx and lines[y][x].isdigit():
            n = n * 10 + int(lines[y][x])
            x += 1

        if n != 0:
            symbol = has_symbol(x0, x, y)
            if symbol:
                part1 += n
                if symbol[0] == "*":
                    gears[symbol].append(n)

        x += 1


print(part1)

part2 = 0
for g in gears.values():
    if len(g) == 2:
        part2 += g[0] * g[1]
print(part2)
