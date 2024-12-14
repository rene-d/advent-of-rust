#!/usr/bin/env python3
# [Day 14: Restroom Redoubt](https://adventofcode.com/2024/day/14)

import itertools
import re
import sys
from collections import defaultdict
from dataclasses import dataclass
from pathlib import Path
from random import randint, seed

from imgcat import imgcat
from PIL import Image

filename = sys.argv[1] if len(sys.argv) > 1 else "input.txt"
data = Path(filename).read_text().strip()
lines = data.splitlines()

# to always make the same picture with a given input
seed(sum(map(ord, filename)))


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

for n in itertools.count(5000):

    grid = defaultdict(int)
    for robot in robots:
        px = (robot.px + robot.vx * n) % width
        py = (robot.py + robot.vy * n) % height

        if (px, py) in grid:
            break

        grid[px, py] = 1
    else:
        break

picture = Image.new("RGB", (width, height))
for y in range(height):
    for x in range(width):
        if grid[x, y] != 0:
            h = sum(grid[x + i, y] for i in range(-4, 5))
            v = sum(grid[x, y + i] for i in range(-4, 5))

            if h >= 5 or v >= 5:
                picture.putpixel((x, y), (0, 255, 0))
            else:
                picture.putpixel((x, y), (255, randint(0, 255), 0))

picture.save("christmastree.png")
imgcat(picture)
