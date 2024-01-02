#!/usr/bin/env python3
# https://adventofcode.com/2017/day/19

import sys
from pathlib import Path
from string import ascii_letters

verbose = "-v" in sys.argv
if verbose:
    sys.argv.remove("-v")
self_tests = "-T" in sys.argv
if self_tests:
    sys.argv.remove("-T")
filename = ("test.txt" if sys.argv[1] == "-t" else sys.argv[1]) if len(sys.argv) > 1 else "input.txt"
data = Path(filename).read_text()

grid = data.splitlines()

nx = max(len(row) for row in grid)
ny = len(grid)

delta = ((0, 1), (-1, 0), (0, -1), (1, 0))  # S W N E

y = 0
x = grid[0].index("|")
d = 0

path = ""
steps = 1

while True:
    dx, dy = delta[d]

    x += dx
    y += dy

    if not (0 <= x < nx and 0 <= y < ny) or grid[y][x] == " ":
        break

    # no change of direction
    if grid[y][x] in "-|":
        steps += 1
        continue

    # letter
    if grid[y][x].isalpha():
        path += grid[y][x]
        steps += 1
        continue

    assert grid[y][x] == "+"

    dx, dy = delta[(d + 1) % 4]
    if 0 <= x + dx < nx and 0 <= y + dy < ny and grid[y + dy][x + dx] in "-|" + ascii_letters:
        d = (d + 1) % 4
        steps += 1
        continue

    dx, dy = delta[(d + 3) % 4]
    if 0 <= x + dx < nx and 0 <= y + dy < ny and grid[y + dy][x + dx] in "-|" + ascii_letters:
        d = (d + 3) % 4
        steps += 1
        continue

    assert False

print(path)
print(steps)

if filename == "test":
    assert path == "ABCDEF"
    assert steps == 38
