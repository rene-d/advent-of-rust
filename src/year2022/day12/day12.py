#!/usr/bin/env python3
# [Day 12: Hill Climbing Algorithm](https://adventofcode.com/2022/day/12)

import atexit
import sys
import time
from collections import deque
from pathlib import Path

filename = (
    "test.txt" if len(sys.argv) > 1 and sys.argv[1] == "-t" else sys.argv[1] if len(sys.argv) > 1 else "input.txt"
)
data = Path(filename).read_text().strip()
if "--elapsed" in sys.argv:
    sys.argv.remove("--elapsed")
    start_time_ns = time.time_ns()
    atexit.register(lambda: print(f"elapsed: {(time.time_ns() - start_time_ns) / 1_000_000}ms"))


grid = []
for y, line in enumerate(data.splitlines()):
    row = []
    for x, c in enumerate(line):
        if c == "E":
            end_position = (x, y)
            c = 26
        elif c == "S":
            start_position = (x, y)
            c = 1
        else:
            c = ord(c) - 96
        row.append(c)
    grid.append(row)
nx, ny = len(grid[0]), len(grid)


def bfs(part):
    """[Breadth-first search](https://en.wikipedia.org/wiki/Breadth-first_search)"""

    q = deque()
    if part == 1:
        q.append((start_position, 0))
    else:
        for y in range(ny):
            for x in range(nx):
                if grid[y][x] == 1:  # start at 'S' or any 'a'
                    q.append(((x, y), 0))

    visited = set()
    while q:
        (x, y), steps = q.popleft()
        if (x, y) in visited:
            continue
        visited.add((x, y))
        if (x, y) == end_position:
            return steps
        for dx, dy in [(1, 0), (-1, 0), (0, 1), (0, -1)]:
            x2, y2 = x + dx, y + dy
            # move within the grid and « at most one higher »
            if 0 <= x2 < nx and 0 <= y2 < ny and grid[y2][x2] <= 1 + grid[y][x]:
                q.append(((x2, y2), steps + 1))


print(bfs(1))
print(bfs(2))
