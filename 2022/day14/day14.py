#!/usr/bin/env python3
# https://adventofcode.com/2022/day/14

from pathlib import Path
import sys
import re

filename = ("test.txt" if sys.argv[1] == "-t" else sys.argv[1]) if len(sys.argv) > 1 else "input.txt"
data = Path(filename).read_text()
lines = data.splitlines()

wall = set()
floor = 0

for line in lines:
    points = list(tuple(map(int, re.match("^(\d+),(\d+)$", p).groups())) for p in line.split(" -> "))

    for a, b in zip(points, points[1:]):

        x1, y1 = a
        x2, y2 = b

        floor = max(y1, y2, floor)

        if x1 == x2:
            if y1 > y2:
                y1, y2 = y2, y1
            for y in range(y1, y2 + 1):
                wall.add((x1, y))
        elif y1 == y2:
            if x1 > x2:
                x1, x2 = x2, x1
            for x in range(x1, x2 + 1):
                wall.add((x, y1))
        else:
            raise ValueError

floor += 2


def fall(part2):
    x, y = 500, 0

    while True:

        if y + 1 >= floor:
            if part2:
                break
            else:
                return False

        if (x, y + 1) not in wall:
            y += 1
        elif (x - 1, y + 1) not in wall:
            y += 1
            x -= 1
        elif (x + 1, y + 1) not in wall:
            y += 1
            x += 1
        else:
            break

    wall.add((x, y))

    if (x, y) == (500, 0):
        return False

    return True


for i in range(100000):
    if not fall(False):
        print(i)
        break

for j in range(100000):
    if not fall(True):
        print(i + j + 1)
        break
