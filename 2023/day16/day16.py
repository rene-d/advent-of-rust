#!/usr/bin/env python3
# https://adventofcode.com/2023/day/16

from pathlib import Path
from copy import deepcopy
from collections import defaultdict, deque, namedtuple
import sys, re, math, itertools, time
from functools import reduce
import re

verbose = "-v" in sys.argv
if verbose:
    sys.argv.remove("-v")
filename = ("test.txt" if sys.argv[1] == "-t" else sys.argv[1]) if len(sys.argv) > 1 else "input.txt"
data = Path(filename).read_text().strip()
lines = data.splitlines()

grid = [[c for c in line] for line in lines]

sx = len(grid[0])
sy = len(grid)

beams = None


def reset_beams():
    global beams
    beams = [[0 for _ in range(sx)] for _ in range(sy)]


def show(show_beams=True):
    n = math.ceil(math.log10(sx))
    for i in range(n):
        print("     " + "".join(f"{x:>{n}}"[i] for x in range(sx)))
    print()

    for y in range(sy):
        s = f"{y:>3}  "
        for x in range(sx):
            e = grid[y][x]
            if show_beams:
                if e == ".":
                    e = beams[y][x]
                    e = ".^v2<222>2222222"[e]
                    e = "\033[1;33m" + e + "\033[0m"
                else:
                    if beams[y][x] != 0:
                        e = "\033[1;33m" + e + "\033[0m"
                    else:
                        e = "\033[1;34m" + e + "\033[0m"
            s += e
        print(s)
    print()


sys.setrecursionlimit(100_000)


def beam(x, y, d):
    if x < 0 or y < 0 or x >= sx or y >= sy:
        return

    id = 1 << "^v<>".index(d)
    if (beams[y][x] & id) == id:
        return

    beams[y][x] = beams[y][x] + id

    if grid[y][x] == ".":
        if d == ">":
            beam(x + 1, y, d)
        elif d == "<":
            beam(x - 1, y, d)
        elif d == "v":
            beam(x, y + 1, d)
        elif d == "^":
            beam(x, y - 1, d)
        else:
            raise ValueError

    elif grid[y][x] == "\\":
        if d == ">":
            beam(x, y + 1, "v")
        elif d == "<":
            beam(x, y - 1, "^")
        elif d == "v":
            beam(x + 1, y, ">")
        elif d == "^":
            beam(x - 1, y, "<")
        else:
            raise ValueError

    elif grid[y][x] == "/":
        if d == ">":
            beam(x, y - 1, "^")
        elif d == "<":
            beam(x, y + 1, "v")
        elif d == "v":
            beam(x - 1, y, "<")
        elif d == "^":
            beam(x + 1, y, ">")
        else:
            raise ValueError

    elif grid[y][x] == "-":
        if d == ">":
            beam(x + 1, y, d)
        elif d == "<":
            beam(x - 1, y, d)
        elif d == "v" or d == "^":
            beam(x - 1, y, "<")
            beam(x + 1, y, ">")
        else:
            raise ValueError

    elif grid[y][x] == "|":
        if d == ">" or d == "<":
            beam(x, y - 1, "^")
            beam(x, y + 1, "v")
        elif d == "v":
            beam(x, y + 1, d)
        elif d == "^":
            beam(x, y - 1, d)
        else:
            raise ValueError
    else:
        raise ValueError


def energized():
    return sum(1 for x in range(sx) for y in range(sy) if beams[y][x] != 0)


# part 1
reset_beams()
beam(0, 0, ">")

if verbose:
    show()
    print(energized())
    exit()

print(energized())

# part 2
m = 0
for x in range(sx):
    reset_beams()
    beam(x, 0, "v")
    m = max(m, energized())
    reset_beams()
    beam(x, sy - 1, "^")
    m = max(m, energized())
for y in range(sy):
    reset_beams()
    beam(0, y, ">")
    m = max(m, energized())
    reset_beams()
    beam(sx - 1, y, "<")
    m = max(m, energized())
print(m)
