#!/usr/bin/env python3
# [Day 15: Beacon Exclusion Zone](https://adventofcode.com/2022/day/15)

import atexit
import re
import sys
import time
from pathlib import Path

filename = ("test.txt" if sys.argv[1] == "-t" else sys.argv[1]) if len(sys.argv) > 1 else "input.txt"
data = Path(filename).read_text()
if "--elapsed" in sys.argv:
    sys.argv.remove("--elapsed")
    start_time_ns = time.time_ns()
    atexit.register(lambda: print(f"elapsed: {(time.time_ns() - start_time_ns) / 1_000_000}ms"))


lines = data.splitlines()


def manhattan(p1, p2):
    return abs(p1[0] - p2[0]) + abs(p1[1] - p2[1])


sensors = list()
beacons = set()
d_max = 0

for line in lines:
    m = re.match(r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)", line)
    if m:
        sx, sy, bx, by = map(int, m.groups())
        d = manhattan((sx, sy), (bx, by))
        d_max = max(d, d_max)
        sensors.append((sx, sy, d))
        beacons.add((bx, by))


######################

part1 = 0
y = 10 if filename == "test.txt" else 2_000_000

bad_pos = set()
for sx, sy, d in sensors:
    for x in range(sx - d, sx + d + 1):
        if sy - (d - abs(sx - x)) <= y <= sy + (d - abs(sx - x)):
            if (x, y) not in beacons:
                bad_pos.add((x, y))

print(len(bad_pos))

######################


class Segment:
    def __repr__(self):
        return f"({self.start},{self.end})"

    def __init__(self, start, end):
        self.start = start
        self.end = end


max_y = 20 if filename == "test.txt" else 4_000_000
for y in range(0, max_y + 1):
    # each sensor defines a zone where there is only one beacon
    # this zone is all points at a distance less than or equal to the Manhattan distance to its beacon
    # (i.e. a disk for this distance, not the Euclidian one)

    # computes the intersection of the blank zone of each sensor and the row y
    segments = []
    for sx, sy, sd in sensors:
        if abs(sy - y) <= sd:
            delta = sd - abs(sy - y)
            x1 = sx - delta
            x2 = sx + delta
            # the intersection is [sₓ-δ, sₓ+δ]
            # or [sₓ-δ, sₓ+δ+1[ since the coordinates are integer
            segments.append(Segment(x1, x2 + 1))

    # the union of all intersecions: it should overlap the entire row [0, 4000000]
    # except for one row: a point should be not covered and this is the solution
    # in this case, the intersection is two disjointed segments
    segments.sort(key=lambda a: a.start)
    union = [segments[0]]
    for seg in segments[1:]:
        if union[-1].end < seg.start:
            union.append(seg)
        elif union[-1].end < seg.end:
            union[-1].end = seg.end
        else:
            pass

    # assert union[0].start <= 0
    # assert union[-1].end >= max_y

    if len(union) > 1:
        # assert len(union) == 2
        # assert union[0].end + 1 == union[1].start
        x = union[0].end
        part2 = x * 4_000_000 + y
        break

print(part2)
