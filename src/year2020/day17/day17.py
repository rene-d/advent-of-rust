#!/usr/bin/env python3
# [Day 17: Conway Cubes](https://adventofcode.com/2020/day/17)

import sys
from pathlib import Path

verbose = "-v" in sys.argv
if verbose:
    sys.argv.remove("-v")
filename = ("test.txt" if sys.argv[1] == "-t" else sys.argv[1]) if len(sys.argv) > 1 else "input.txt"
data = Path(filename).read_text().strip()
lines = data.splitlines()


cubes = set()
hypercubes = set()

for y, row in enumerate(lines):
    for x, ch in enumerate(row):
        if ch == "#":
            cubes.add((x, y, 0))
            hypercubes.add((x, y, 0, 0))


def neighbors(cube):
    if len(cube) == 3:
        x, y, z = cube
        for dx in range(-1, 2):
            for dy in range(-1, 2):
                for dz in range(-1, 2):
                    if dx != 0 or dy != 0 or dz != 0:
                        yield x + dx, y + dy, z + dz
    else:
        x, y, z, w = cube
        for dx in range(-1, 2):
            for dy in range(-1, 2):
                for dz in range(-1, 2):
                    for dw in range(-1, 2):
                        if dx != 0 or dy != 0 or dz != 0 or dw != 0:
                            yield x + dx, y + dy, z + dz, w + dw


def cycle(cubes):
    next_cubes = set()
    tested = set()
    for cube in cubes:
        actives = 0

        for c in neighbors(cube):
            if c in cubes:
                actives += 1
            else:
                # c is inactive
                if c not in tested:
                    tested.add(c)
                    if 3 == sum(1 for cc in neighbors(c) if cc in cubes):
                        # c becomes active since it has exactly 3 active neighbors
                        next_cubes.add(c)

        if actives == 2 or actives == 3:
            # active with exactly 2 or 3 active neighbors, the cube remains active
            next_cubes.add(cube)

    return next_cubes


for _ in range(6):
    cubes = cycle(cubes)
print(len(cubes))


for _ in range(6):
    hypercubes = cycle(hypercubes)
print(len(hypercubes))
