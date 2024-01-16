#!/usr/bin/env python3
# [Day 18: Settlers of The North Pole](https://adventofcode.com/2018/day/18)

import sys
import time
from collections import Counter
from copy import deepcopy
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


OPEN_ACRE = 0
TREE = 1
LUMBERYARD = 2

N = len(lines[0])

# read the area
area = bytearray(N * N)
for y, line in enumerate(lines):
    for x, acre in enumerate(line):
        area[y * N + x] = {".": OPEN_ACRE, "|": TREE, "#": LUMBERYARD}[acre]


def collect(area):
    new_area = bytearray(N * N)

    for y in range(N):
        for x in range(N):
            acre = area[y * N + x]

            adjacents = Counter(
                area[(y + dy) * N + x + dx]
                for dx in range(-1, 2)
                for dy in range(-1, 2)
                if 0 <= x + dx < N and 0 <= y + dy < N and (dx, dy) != (0, 0)
            )

            if acre == OPEN_ACRE:
                if adjacents[TREE] >= 3:
                    acre = TREE
            elif acre == TREE:
                if adjacents[LUMBERYARD] >= 3:
                    acre = LUMBERYARD
            elif acre == LUMBERYARD:
                if not (adjacents[TREE] >= 1 and adjacents[LUMBERYARD] >= 1):
                    acre = OPEN_ACRE
            else:
                raise ValueError

            new_area[y * N + x] = acre

    return new_area


def value(area):
    trees = sum(1 for i in range(N * N) if area[i] == TREE)
    lumberyards = sum(1 for i in range(N * N) if area[i] == LUMBERYARD)
    return trees * lumberyards


def show(area):
    ACRE = "\033[38:5:{n}m".format(n=231)
    GREEN = "\033[1;32m"
    BROWN = "\033[38;2;{r};{g};{b}m".format(r=165, g=42, b=42)
    RESET = "\033[0m"
    return (
        f"{RESET}\n".join(
            "".join((f"{ACRE}..", f"{GREEN}||", f"{BROWN}##")[area[y * N + x]] for x in range(N)) for y in range(N)
        )
        + RESET
    )


def animate(area):
    a = deepcopy(area)
    for i in range(500):
        print("\033[H\033[2J", show(a))
        time.sleep(0.010)
        a = collect(a)


if verbose:
    animate(area)


# part 1

a = deepcopy(area)
for _ in range(10):
    a = collect(a)
print(value(a))


# part 2

a = deepcopy(area)
seen = dict()
values = []

for i in range(1000):
    values.append(value(a))
    s = bytes(a)
    if s in seen:
        cycle_start = seen[s]
        cycle_end = i
        break
    seen[s] = i
    a = collect(a)

n = 1000000000
cycle = cycle_end - cycle_start
print(values[cycle_start + (n - cycle_end) % cycle])
