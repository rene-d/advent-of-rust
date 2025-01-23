#!/usr/bin/env python3
# [Day 10: Pipe Maze](https://adventofcode.com/2023/day/10)

import argparse
from collections import deque
from pathlib import Path

from shapely.geometry import Point
from shapely.geometry.polygon import Polygon

parser = argparse.ArgumentParser()
parser.add_argument("--elapsed", action="store_true")
parser.add_argument("--png", action="store_true")
parser.add_argument("-o", "--output", type=Path, default="pipemaze.png")
parser.add_argument("-s", "--scale", type=int, default=5)
parser.add_argument("-v", "--verbose", action="store_true")
parser.add_argument("filename", type=Path, nargs="?", default="input.txt")
args = parser.parse_args()
data = args.filename.read_text().strip()
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


def png():
    from PIL import Image

    S = args.scale

    im = Image.new("RGB", (sx * S, sy * S))

    def square(x, y, color):
        for k in range((S - 2) * (S - 2)):
            im.putpixel((x * S + 1 + k % (S - 2), y * S + 1 + k // (S - 2)), color)

        # for k in range(S * S):
        #     im.putpixel((x * S + k % S, y * S + k // S), color)

    def line(x, y, d):
        im.putpixel((x * S + S // 2, y * S + S // 2), (255, 0, 255))
        for c in d:
            for k in range(S // 2):
                match c:
                    case ">":
                        im.putpixel((x * S + S - 1 - k, y * S + S // 2), (255, 0, 255))
                    case "<":
                        im.putpixel((x * S + k, y * S + S // 2), (255, 0, 255))
                    case "v":
                        im.putpixel((x * S + S // 2, y * S + S - 1 - k), (255, 0, 255))
                    case "^":
                        im.putpixel((x * S + S // 2, y * S + k), (255, 0, 255))

    for y in range(sy):
        for x in range(sx):
            if (x, y) in interior:
                square(x, y, (255, 255, 0))

            elif (x, y) not in visited:
                pass

            else:
                c = grid(x, y)

                if c == "S":
                    # background square to mark the starting position
                    square(x, y, (40, 130, 40))

                    # guess the missing pipe shape
                    d = ""
                    if grid(x, y - 1) in "F7|":
                        d += "^"
                    if grid(x, y + 1) in "LJ|":
                        d += "v"
                    if grid(x - 1, y) in "FL-":
                        d += "<"
                    if grid(x + 1, y) in "7J-":
                        d += ">"
                    line(x, y, d)

                elif c == ".":
                    pass

                else:
                    d = {"F": "v>", "J": "<^", "L": ">^", "7": "<v", "-": "<>", "|": "^v"}[c]
                    line(x, y, d)

    im.save(args.output)


if args.png:
    png()

if args.verbose:
    prt()

print(part1)
print(part2)
