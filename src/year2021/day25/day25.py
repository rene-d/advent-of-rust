#!/usr/bin/env python3

# Day 25: Sea Cucumber
# https://adventofcode.com/2021/day/25

import atexit
import sys
import time

data = [line.strip() for line in open("input.txt" if len(sys.argv) == 1 else sys.argv[1])]
if "--elapsed" in sys.argv:
    sys.argv.remove("--elapsed")
    start_time_ns = time.time_ns()
    atexit.register(lambda: print(f"elapsed: {(time.time_ns() - start_time_ns) / 1_000_000}ms"))


ny = len(data)
nx = len(data[0])

grid = [["." for _ in range(nx)] for _ in range(ny)]
for y, line in enumerate(data):
    for x, c in enumerate(line):
        grid[y][x] = c


def prt():
    print("\n".join(["".join(line) for line in grid]))
    print()


def move():
    moved = False

    # don't move blocked sea cucumbers
    for y in range(ny):
        for x in range(nx):
            if grid[y][x] == grid[y][(x + 1) % nx] == ">":
                grid[y][x] = "H"
            if grid[y][x] == grid[(y + 1) % ny][x] == "v":
                grid[y][x] = "V"

    # « During a single step, the east-facing herd moves first,
    for y in range(ny):
        for x in range(nx):
            c = grid[y][x]
            if c == ">" and grid[y][(x + 1) % nx] == ".":
                grid[y][(x + 1) % nx] = "H"
                grid[y][x] = "."
                moved = True

    # then the south-facing herd moves. »
    for y in range(ny):
        for x in range(nx):
            c = grid[y][x]
            if c == "v" and grid[(y + 1) % ny][x] == ".":
                grid[(y + 1) % ny][x] = "V"
                grid[y][x] = "."
                moved = True

    # restore blocked and moving sea cucumbers
    for y in range(ny):
        for x in range(nx):
            c = grid[y][x]
            if c == "H":
                grid[y][x] = ">"
            elif c == "V":
                grid[y][x] = "v"

    return moved


step = 1
while move():
    step += 1
print(step)
