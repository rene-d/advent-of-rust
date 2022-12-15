#!/usr/bin/env python3
# https://adventofcode.com/2022/day/15

from pathlib import Path
import sys
import re

filename = "test.txt" if len(sys.argv) > 1 and sys.argv[1] == "-t" else "input.txt"
data = Path(filename).read_text()
lines = data.splitlines()


def manhattan(p1, p2):
    return abs(p1[0] - p2[0]) + abs(p1[1] - p2[1])


sensors = set()
beacons = set()
d_max = 0

for line in lines:
    m = re.match(r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)", line)
    if m:
        sx, sy, bx, by = map(int, m.groups())
        d = manhattan((sx, sy), (bx, by))
        d_max = max(d, d_max)
        sensors.add((sx, sy, d))
        beacons.add((bx, by))

# the search range
bx_min = min(x for x, y in beacons) - d_max
bx_max = max(x for x, y in beacons) + d_max

part1 = 0
y = 10 if filename == "test.txt" else 2000000
for x in range(bx_min, bx_max + 1):
    if (x, y) in beacons:
        continue

    ok = True
    for sx, sy, nearest_beacon in sensors:
        d = manhattan((sx, sy), (x, y))
        if d <= nearest_beacon:
            # the sensors always report the nearest beacon
            # if the distance is less than the distance measured by the sensor,
            # there cannot be a beacon at this position
            ok = False
            break
    if not ok:
        part1 += 1

print(part1)
