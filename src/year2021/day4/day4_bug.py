#!/usr/bin/env python3

# Day 4: Giant Squid
# https://adventofcode.com/2021/day/4

import atexit
import sys
import time

filename = ("test.txt" if sys.argv[1] == "-t" else sys.argv[1]) if len(sys.argv) > 1 else "input.txt"
data = open(filename).readlines()
if "--elapsed" in sys.argv:
    sys.argv.remove("--elapsed")
    start_time_ns = time.time_ns()
    atexit.register(lambda: print(f"elapsed: {(time.time_ns() - start_time_ns) / 1_000_000}ms"))


drawn = list(map(int, data[0].split(",")))

grids = []
for line in range(1, len(data), 6):
    grid = []
    for i in range(1, 6):
        grid.extend(list(map(int, data[line + i].split())))
    grids.append(grid)


def win(grid):
    for i in range(5):
        if grid[i] == grid[i + 5] == grid[i + 10] == grid[i + 15] == grid[i + 20] == 0:
            return True
        if grid[i * 5] == grid[i * 5 + 1] == grid[i * 5 + 2] == grid[i * 5 + 3] == grid[i * 5 + 4] == 0:
            return True
    return False


first_win = False
last_draw = None

for draw in drawn:
    for grid in grids:
        if grid[0] == -1:
            continue

        for i in range(25):
            if draw == grid[i]:
                grid[i] = 0
                break

        if win(grid):
            if not first_win:
                print(draw * sum(grid))
                first_win = True

            last_win = draw * sum(grid)
            grid[0] = -1

print(last_win)
