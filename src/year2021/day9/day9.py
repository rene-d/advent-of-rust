#!/usr/bin/env python3

# Day 9: Smoke Basin
# https://adventofcode.com/2021/day/9

import sys

data = open("input.txt" if len(sys.argv) == 1 else sys.argv[1]).read().splitlines()

sy = len(data)
sx = len(data[0])
grid = []
for line in data:
    grid.append(list(int(x) for x in line))


def basin(ly, lx):
    heap = [(1, ly, lx)]
    n = 0

    while heap:
        size, y, x = heap.pop()

        if grid[y][x] == 9:
            continue

        grid[y][x] = 9
        n += 1

        size += 1
        if y < sy - 1:
            heap.append((size, y + 1, x))
        if x < sx - 1:
            heap.append((size, y, x + 1))
        if y > 0:
            heap.append((size, y - 1, x))
        if x > 0:
            heap.append((size, y, x - 1))

    return n


basins = []
risk = 0
for y in range(sy):
    for x in range(sx):
        v = grid[y][x]
        if y > 0 and v >= grid[y - 1][x]:
            continue
        if x > 0 and v >= grid[y][x - 1]:
            continue
        if y < sy - 1 and v >= grid[y + 1][x]:
            continue
        if x < sx - 1 and v >= grid[y][x + 1]:
            continue
        risk += v + 1
        basins.append(basin(y, x))

# part 1
print(risk)

# part 2
basins = sorted(basins)
print(basins[-1] * basins[-2] * basins[-3])
