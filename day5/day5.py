#!/usr/bin/env python3

import re

data = open("input.txt").read().splitlines()

SIZEX = 1000
SIZEY = 1000

# step 1
grid = [0] * (SIZEX * SIZEY)
for line in data:
    x1, y1, x2, y2 = map(int, re.findall(r"\d+", line))

    if x1 == x2:
        if y1 > y2:
            y1, y2 = y2, y1
        for y in range(y1, y2 + 1):
            grid[SIZEX * y + x1] += 1

    elif y1 == y2:
        if x1 > x2:
            x1, x2 = x2, x1
        for x in range(x1, x2 + 1):
            grid[SIZEX * y1 + x] += 1

print(sum(1 for p in grid if p >= 2))

# step 2
grid = [0] * (SIZEX * SIZEY)
for line in data:
    x1, y1, x2, y2 = map(int, re.findall(r"\d+", line))

    if x1 == x2:
        if y1 > y2:
            y1, y2 = y2, y1
        for y in range(y1, y2 + 1):
            grid[SIZEX * y + x1] += 1

    elif y1 == y2:
        if x1 > x2:
            x1, x2 = x2, x1
        for x in range(x1, x2 + 1):
            grid[SIZEX * y1 + x] += 1

    else:
        if x1 > x2:
            x1, x2 = x2, x1
            y1, y2 = y2, y1
        assert x2 - x1 == abs(y2 - y1)
        if y1 < y2:
            for i in range(x2 - x1 + 1):
                grid[SIZEX * (y1 + i) + x1 + i] += 1
        else:
            for i in range(x2 - x1 + 1):
                grid[SIZEX * (y1 - i) + x1 + i] += 1

print(sum(1 for p in grid if p >= 2))
