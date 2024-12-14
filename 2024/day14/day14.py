#!/usr/bin/env python3
# [Day 14: Restroom Redoubt](https://adventofcode.com/2024/day/14)

import itertools
import re
import sys
from collections import defaultdict
from dataclasses import dataclass
from functools import reduce
from operator import mul
from pathlib import Path

verbose = "-v" in sys.argv
if verbose:
    sys.argv.remove("-v")
self_tests = "-T" in sys.argv
if self_tests:
    sys.argv.remove("-T")
filename = ("test.txt" if sys.argv[1] == "-t" else sys.argv[1]) if len(sys.argv) > 1 else "input.txt"
data = Path(filename).read_text().strip()
lines = data.splitlines()


@dataclass
class Robot:
    px: int
    py: int
    vx: int
    vy: int


robots = []
for line in lines:
    m = re.match(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)", line)
    px, py, vx, vy = map(int, m.groups())
    robots.append(Robot(px, py, vx, vy))

width = 101
height = 103

if filename == "test.txt":
    width = 11
    height = 7

quadrants = defaultdict(int)
for robot in robots:

    px = (robot.px + robot.vx * 100) % width
    py = (robot.py + robot.vy * 100) % height

    if px == width // 2 or py == height // 2:
        continue
    q = (px * 2) // width, (py * 2) // height
    quadrants[q] += 1

print(reduce(mul, quadrants.values(), 1))

# part 2 requires the real puzzle input
if filename == "test.txt":
    exit()

if verbose:
    # investigation
    for n in itertools.count():

        print(n, end="\r")

        grid = defaultdict(int)
        for robot in robots:
            px = (robot.px + robot.vx * n) % width
            py = (robot.py + robot.vy * n) % height

            grid[px, py] += 1

        horizontal = 0
        for y in range(width):
            for x in range(width - 5):
                if all(grid.get((x + i, y), 0) > 0 for i in range(5)):
                    horizontal += 1
                    break

        if horizontal > 5:
            print("\033[H\033[2J   SECOND: {n}")
            for y in range(height):
                line = "".join(str(grid.get((x, y), ".")) for x in range(width))
                print(line)

    exit()


for n in range(100_000):
    grid = defaultdict(int)
    for robot in robots:
        px = (robot.px + robot.vx * n) % width
        py = (robot.py + robot.vy * n) % height

        grid[px, py] += 1

        if grid[px, py] > 1:
            break

    else:
        # assume there is a christmas tree in the middle of picture
        # when no robot is in the same place
        print(n)
        break
