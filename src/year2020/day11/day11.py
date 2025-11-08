#!/usr/bin/env python3
# [Day 11: Seating System](https://adventofcode.com/2020/day/11)

import atexit
import sys
import time
from pathlib import Path

verbose = "-v" in sys.argv
if verbose:
    sys.argv.remove("-v")
filename = ("test.txt" if sys.argv[1] == "-t" else sys.argv[1]) if len(sys.argv) > 1 else "input.txt"
data = Path(filename).read_text().strip()
if "--elapsed" in sys.argv:
    sys.argv.remove("--elapsed")
    start_time_ns = time.time_ns()
    atexit.register(lambda: print(f"elapsed: {(time.time_ns() - start_time_ns) / 1_000_000}ms"))


lines = data.splitlines()


def solve(part2):
    seats = {}
    for y, line in enumerate(lines):
        for x, seat in enumerate(line):
            if seat == "L":
                seats[(x, y)] = "L"

    if part2:
        N = max(max(x for x, _ in seats.keys()), max(x for _, y in seats.keys())) + 1
        TOLERANCE = 5
    else:
        N = 1
        TOLERANCE = 4

    while True:
        change = False
        new_seats = {}
        for (x, y), seat in seats.items():
            occupied = 0

            for dx, dy in ((1, -1), (1, 0), (1, 1), (0, -1), (0, 1), (-1, -1), (-1, 0), (-1, 1)):
                for n in range(1, N + 1):
                    c = seats.get((x + n * dx, y + n * dy), ".")
                    if c == ".":
                        continue
                    if c == "#":
                        occupied += 1
                    break

            if seat == "L" and occupied == 0:
                new_seat = "#"
                change = True
            elif seat == "#" and occupied >= TOLERANCE:
                new_seat = "L"
                change = True
            else:
                new_seat = seat

            new_seats[(x, y)] = new_seat

        if not change:
            break
        seats = new_seats

    print(sum(1 for c in seats.values() if c == "#"))


solve(False)
solve(True)
