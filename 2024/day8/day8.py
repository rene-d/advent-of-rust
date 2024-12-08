#!/usr/bin/env python3
# [Day 8: Resonant Collinearity](https://adventofcode.com/2024/day/8)

from pathlib import Path
from collections import defaultdict
import sys, itertools

verbose = "-v" in sys.argv
if verbose:
    sys.argv.remove("-v")
self_tests = "-T" in sys.argv
if self_tests:
    sys.argv.remove("-T")
filename = ("test.txt" if sys.argv[1] == "-t" else sys.argv[1]) if len(sys.argv) > 1 else "input.txt"
data = Path(filename).read_text().strip()

lines = data.splitlines()
width = len(lines[0])
height = len(lines)

antennas = defaultdict(list)
for y, line in enumerate(lines):
    for x, c in enumerate(line):
        if c != ".":
            antennas[c].append((x, y))

uniq = set()
for c in antennas:
    for a, b in itertools.combinations(antennas[c], 2):

        antinode = (a[0] - (b[0] - a[0]), a[1] - (b[1] - a[1]))
        uniq.add(antinode)

        antinode = (b[0] + (b[0] - a[0]), b[1] + (b[1] - a[1]))
        uniq.add(antinode)

print(sum(1 if 0 <= x < width and 0 <= y < height else 0 for x, y in uniq))


uniq = set()
for c in antennas:
    for a, b in itertools.combinations(antennas[c], 2):

        dx = b[0] - a[0]
        dy = b[1] - a[1]

        for n in itertools.count(0):
            x = a[0] - n * dx
            y = a[1] - n * dy
            if not (0 <= x < width and 0 <= y < height):
                break
            uniq.add((x, y))

        for n in itertools.count(0):
            x = b[0] + n * dx
            y = b[1] + n * dy
            if not (0 <= x < width and 0 <= y < height):
                break
            uniq.add((x, y))

print(len(uniq))
