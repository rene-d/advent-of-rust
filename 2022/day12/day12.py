#!/usr/bin/env python3
# https://adventofcode.com/2022/day/12

from pathlib import Path
import sys
from collections import deque

filename = "test.txt" if len(sys.argv) > 1 and sys.argv[1] == "-t" else "input.txt"
data = Path(filename).read_text()

grid = []
for y, line in enumerate(data.splitlines()):
    row = []
    for x, c in enumerate(line):
        if c == "E":
            end_position = (x, y)
            c = 26
        elif c == "S":
            start_position = (x, y)
            c = 0
        else:
            c = ord(c) - 96
        row.append(c)
    grid.append(row)
nx, ny = len(grid[0]), len(grid)


def bfs(part):
    """[Breadth-first search](https://en.wikipedia.org/wiki/Breadth-first_search)"""

    q = deque()
    for y in range(ny):
        for x in range(nx):
            if grid[y][x] == part - 1:  # start at 'S' (0) or any 'a' (1)
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
