#!/usr/bin/env python3
# [Day 4: Printing Department](https://adventofcode.com/2025/day/4)

import atexit
import time
from argparse import ArgumentParser
from pathlib import Path

parser = ArgumentParser()
parser.add_argument("-v", "--verbose", action="store_true")
parser.add_argument("-t", "--test", action="store_true")
parser.add_argument("--elapsed", action="store_true")
parser.add_argument("filename", nargs="?", type=Path, default="input.txt")
args = parser.parse_args()
if args.test:
    args.filename = Path("test.txt")

data = args.filename.read_text()

if args.elapsed:
    start_time_ns = time.time_ns()
    atexit.register(lambda: print(f"elapsed: {(time.time_ns() - start_time_ns) / 1_000_000}ms"))


grid = [list(row) for row in data.splitlines()]
sx = len(grid[0])
sy = len(grid)
PAPER_ROLL = "@"
EMPTY = "."

part1 = 0
part2 = 0

while True:
    accessible = set()

    for x in range(sx):
        for y in range(sx):
            if grid[y][x] == PAPER_ROLL:
                rolls = 0
                for dx, dy in ((-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)):
                    if 0 <= x + dx < sx and 0 <= y + dy < sy:
                        if grid[y + dy][x + dx] == PAPER_ROLL:
                            rolls += 1
                if rolls < 4:
                    accessible.add((x, y))

    if not accessible:
        break

    if part1 == 0:
        part1 = len(accessible)

    part2 += len(accessible)

    for x, y in accessible:
        grid[y][x] = EMPTY

print(part1)
print(part2)
