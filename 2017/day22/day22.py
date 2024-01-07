#!/usr/bin/env python3
# [Day 22: Sporifica Virus](https://adventofcode.com/2017/day/22)

import sys
from collections import defaultdict
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

nx = len(lines[0])
ny = len(lines)


MOVES = ((0, -1), (1, 0), (0, 1), (-1, 0))  # up/right/down/left


# part 1

infected = set()
for y, line in enumerate(lines):
    for x, c in enumerate(line):
        if c == "#":
            infected.add((x, y))

x, y = nx // 2, ny // 2  # middle of the map
d = 0  # facing up

infections1 = 0
for _ in range(10000):
    if (x, y) in infected:
        d = (d + 1) % 4  # turn right
        infected.discard((x, y))  # infect the node
    else:
        d = (d + 3) % 4  # turn left
        infected.add((x, y))  # infect the node
        infections1 += 1

    dx, dy = MOVES[d]  # move
    x, y = x + dx, y + dy

print(infections1)


# part 2

CLEAN = 0
WEAKENED = 1
INFECTED = 2
FLAGGED = 3

nodes = defaultdict(lambda: CLEAN)
for y, line in enumerate(lines):
    for x, c in enumerate(line):
        if c == "#":
            nodes[x, y] = INFECTED

x, y = nx // 2, ny // 2  # middle of the map
d = 0  # facing up

infections2 = 0
for _ in range(10_000_000):
    node = nodes[x, y]

    if node == CLEAN:
        d = (d + 3) % 4  # turn left
        node = WEAKENED

    elif node == WEAKENED:
        node = INFECTED
        # do not turn
        infections2 += 1

    elif node == INFECTED:
        d = (d + 1) % 4  # turn right
        node = FLAGGED

    elif node == FLAGGED:
        node = CLEAN
        d = (d + 2) % 4  # go back

    else:
        assert False

    nodes[x, y] = node

    dx, dy = MOVES[d]  # move
    x, y = x + dx, y + dy

print(infections2)


# test
if filename == "test":
    assert infections1 == 5587
    assert infections2 == 2511944
