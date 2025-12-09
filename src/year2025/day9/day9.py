#!/usr/bin/env python3
# [Day 9: Movie Theater](https://adventofcode.com/2025/day/9)

import atexit
import time
from argparse import ArgumentParser
from operator import itemgetter
from pathlib import Path

parser = ArgumentParser()
parser.add_argument("-v", "--verbose", action="store_true")
parser.add_argument("-t", "--test", action="store_true")
parser.add_argument("--elapsed", action="store_true")
parser.add_argument("--draw", action="store_true")
parser.add_argument("filename", nargs="?", type=Path, default="input.txt")
args = parser.parse_args()
if args.test:
    args.filename = Path("test.txt")

data = args.filename.read_text()

if args.elapsed:
    start_time_ns = time.time_ns()
    atexit.register(lambda: print(f"elapsed: {(time.time_ns() - start_time_ns) / 1_000_000}ms"))


def fill_polygon(vertices, width, height, draw_pixel):
    if not vertices:
        return

    min_y = min(y for x, y in vertices)
    max_y = max(y for x, y in vertices)

    min_y = max(0, min_y)
    max_y = min(height - 1, max_y)

    num_vertices = len(vertices)

    for y in range(min_y, max_y + 1):
        intersections = []

        for i in range(num_vertices):
            p1 = vertices[i]
            p2 = vertices[(i + 1) % num_vertices]

            y1, y2 = p1[1], p2[1]
            x1, x2 = p1[0], p2[0]

            if (y1 <= y < y2) or (y2 <= y < y1):
                if y2 != y1:
                    x_intersect = x1 + (y - y1) * (x2 - x1) / (y2 - y1)
                    intersections.append(x_intersect)

        intersections.sort()

        for i in range(0, len(intersections), 2):
            if i + 1 < len(intersections):
                start_x = int(intersections[i])
                end_x = int(intersections[i + 1])

                for x in range(start_x, end_x):
                    if 0 <= x < width and 0 <= y < height:
                        draw_pixel(x, y)


points = list(tuple(map(int, line.split(","))) for line in data.splitlines())

# coordinate compression
coord_x = sorted(map(itemgetter(0), points))
coord_y = sorted(map(itemgetter(1), points))

comp_x = {}
for i, x in enumerate(coord_x, 1):
    comp_x[x] = i
inv_x = dict((i, x) for x, i in comp_x.items())

comp_y = {}
for i, y in enumerate(coord_y, 1):
    comp_y[y] = i
inv_y = dict((i, y) for y, i in comp_y.items())

comp = list((comp_x[x], comp_y[y]) for x, y in points)


grid = [["." for x in range(500)] for y in range(500)]


def fill_grid(x, y):
    grid[y][x] = "X"


fill_polygon(comp, 500, 500, fill_grid)


for i in range(len(comp)):
    p1 = comp[i]
    p2 = comp[(i + 1) % len(comp)]
    if p1[0] == p2[0]:
        x = p2[0]
        y1 = min(p1[1], p2[1])
        y2 = max(p1[1], p2[1])
        for y in range(y1, y2 + 1):
            grid[y][x] = "#"
    elif p1[1] == p2[1]:
        y = p2[1]
        x1 = min(p1[0], p2[0])
        x2 = max(p1[0], p2[0])
        for x in range(x1, x2 + 1):
            grid[y][x] = "#"
    else:
        exit()


max_area1 = 0
max_area2 = 0
rect = None
for i in range(len(comp) - 1):
    for j in range(i + 1, len(comp)):
        p1 = comp[i]
        p2 = comp[j]

        x1 = min(p1[0], p2[0])
        x2 = max(p1[0], p2[0])
        y1 = min(p1[1], p2[1])
        y2 = max(p1[1], p2[1])

        area = (inv_x[x2] - inv_x[x1] + 1) * (inv_y[y2] - inv_y[y1] + 1)

        # part one
        if area > max_area1:
            max_area1 = area

        # part two
        if area > max_area2:
            contained = True
            for x in range(x1, x2 + 1):
                for y in range(y1, y2 + 1):
                    if grid[y][x] == ".":
                        contained = False
                        break
                if not contained:
                    break

            if contained:
                max_area2 = area
                rect = (x1, x2, y1, y2)


print(max_area1)
print(max_area2)


if args.draw:
    from PIL import Image

    im = Image.new("RGB", (500, 500))

    for y in range(500):
        for x in range(500):
            if grid[y][x] == "X":
                im.putpixel((x, y), (30, 220, 30))
            elif grid[y][x] == "#":
                im.putpixel((x, y), (220, 10, 10))

    for x in range(rect[0], rect[1] + 1):
        for y in range(rect[2], rect[3] + 1):
            im.putpixel((x, y), (100, 100, 100))

    for x, y in comp:
        im.putpixel((x, y), (220, 10, 10))

    for i in range(1, 50):
        f = Path(f"im{i:02d}.png")
        if not f.is_file():
            break

    im.save(f)
