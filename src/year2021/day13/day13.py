#!/usr/bin/env python3

# Day 13: Transparent Origami
# https://adventofcode.com/2021/day/13

import atexit
import sys
import time
from pathlib import Path

data = open("input.txt" if len(sys.argv) == 1 else sys.argv[1]).read().splitlines()
if "--elapsed" in sys.argv:
    sys.argv.remove("--elapsed")
    start_time_ns = time.time_ns()
    atexit.register(lambda: print(f"elapsed: {(time.time_ns() - start_time_ns) / 1_000_000}ms"))


sys.path.append(Path(__file__).parent.parent.as_posix())
from ocr.ocr import ocr  # noqa

n = 2000
grid = [["." for _ in range(n)] for _ in range(n)]

for line in data:
    if not line or line.startswith("fold"):
        break
    x, y = map(int, line.split(","))
    grid[y][x] = "#"

# for y, row in enumerate(grid):
#     print("%3d" % y, "".join(str(x) for x in row))

part1 = False

for line in data:
    if not line.startswith("fold"):
        continue

    if line.startswith("fold along x="):
        fold = int(line.split("=")[1])
        for y in range(len(grid)):
            for x in range(fold):
                if grid[y][fold + 1 + x] == "#":
                    grid[y][fold - 1 - x] = "#"
            del grid[y][fold:]

    elif line.startswith("fold along y="):
        fold = int(line.split("=")[1])
        for y in range(fold):
            for x in range(len(grid[0])):
                if grid[fold + 1 + y][x] == "#":
                    grid[fold - 1 - y][x] = "#"

        del grid[fold:]

    if not part1:
        print(sum(1 for row in grid for cell in row if cell == "#"))
        part1 = True

crt = "\n".join("".join(str(x) for x in row) for row in grid)
print(ocr(crt))
