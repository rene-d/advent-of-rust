#!/usr/bin/env python3
# https://adventofcode.com/2023/day/10

import sys
from collections import deque
from pathlib import Path

from shapely.geometry import Point
from shapely.geometry.polygon import Polygon

verbose = "-v" in sys.argv
if verbose:
    sys.argv.remove("-v")
filename = ("test.txt" if sys.argv[1] == "-t" else sys.argv[1]) if len(sys.argv) > 1 else "input.txt"
data = Path(filename).read_text().strip()
lines = data.splitlines()

_grid = [[c for c in line] for line in lines]

sx = len(_grid[0])
sy = len(_grid)


def grid(x, y):
    if 0 <= x < sx and 0 <= y < sy:
        return _grid[y][x]
    return "."


for y in range(sy):
    for x in range(sx):
        if grid(x, y) == "S":
            starting = (x, y)


visited = set()
q = deque()

q.append(starting)
pt = []
while q:
    x, y = q.pop()

    if (x, y) in visited:
        continue
    visited.add((x, y))

    pt.append(Point(x, y))

    c = grid(x, y)

    if grid(x, y + 1) in "|LJ" and c in "|7FS":
        q.append((x, y + 1))

    if grid(x, y - 1) in "|7F" and c in "|LJS":
        q.append((x, y - 1))

    if grid(x - 1, y) in "-FL" and c in "-J7S":
        q.append((x - 1, y))

    if grid(x + 1, y) in "-7J" and c in "-FLS":
        q.append((x + 1, y))

part1 = len(visited) // 2


interior = set()
po = Polygon(pt)
part2 = 0
for y in range(sy):
    for x in range(sx):
        p = (x, y)
        if p in visited:
            continue
        if po.contains(Point(*p)):
            part2 += 1
            interior.add((x, y))


def prt():
    for y in range(sy):
        s = ""
        for x in range(sx):
            if (x, y) in interior:
                c = "\033[93mI\033[0m"
            elif (x, y) not in visited:
                c = " "
            else:
                c = grid(x, y)

                if c == "S":
                    c = "\033[32mS\033[0m"
                elif c == ".":
                    c = " "
                else:
                    if c == "F":
                        c = "┌"
                    if c == "J":
                        c = "┘"
                    if c == "L":
                        c = "└"
                    if c == "7":
                        c = "┐"
                    if c == "-":
                        c = "─"
                    if c == "|":
                        c = "│"
                    c = f"\033[95m{c}\033[0m"
            s += c
        print(s)


if verbose:
    prt()

print(part1)
print(part2)
