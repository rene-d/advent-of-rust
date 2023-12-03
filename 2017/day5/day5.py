#!/usr/bin/env python3
# https://adventofcode.com/2017/day/5

import sys
from pathlib import Path

filename = ("test.txt" if sys.argv[1] == "-t" else sys.argv[1]) if len(sys.argv) > 1 else "input.txt"
data = Path(filename).read_text().strip()

maze = list(map(int, data.splitlines()))
i = 0
n = 0
while 0 <= i < len(maze):
    offset = maze[i]
    maze[i] += 1
    i += offset
    n += 1
print(n)

maze = list(map(int, data.splitlines()))
i = 0
n = 0
while 0 <= i < len(maze):
    offset = maze[i]
    if offset >= 3:
        maze[i] -= 1
    else:
        maze[i] += 1
    i += offset
    n += 1
print(n)
