#!/usr/bin/env python3
# [Day 24: Lobby Layout](https://adventofcode.com/2020/day/24)

import sys
from collections import defaultdict
from pathlib import Path

verbose = "-v" in sys.argv
if verbose:
    sys.argv.remove("-v")
filename = ("test.txt" if sys.argv[1] == "-t" else sys.argv[1]) if len(sys.argv) > 1 else "input.txt"
data = Path(filename).read_text().strip()
lines = data.splitlines()


# part 1


def walk(data, tiles):
    x, y = 0, 0
    i = 0
    while i < len(data):
        if data[i : i + 2] == "ne":
            i += 2
            x = x + 1
            y = y + 1

        elif data[i : i + 2] == "se":
            i += 2
            x = x + 1
            y = y - 1

        elif data[i : i + 1] == "w":
            i += 1
            x = x - 2

        elif data[i : i + 1] == "e":
            i += 1
            x = x + 2

        elif data[i : i + 2] == "nw":
            i += 2
            x = x - 1
            y = y + 1

        elif data[i : i + 2] == "sw":
            i += 2
            x = x - 1
            y = y - 1

        else:
            raise ValueError(data[i:])

    tiles[(x, y)] = not tiles[(x, y)]


tiles = defaultdict(lambda: False)
for line in lines:
    walk(line, tiles)

# get only the black tiles (=True, white tiles=False)
# we need them for part 2
tiles = set((x, y) for (x, y), face in tiles.items() if face is True)

print(len(tiles))


# part 2

# the six neighbors of a tile
NEIGHBORS = ((1, 1), (1, -1), (-2, 0), (2, 0), (-1, 1), (-1, -1))

# Hexagonal Conway's game of life

for day in range(100):
    new_tiles = set()
    seen = set()

    # for every visited tiles
    for black_x, black_y in tiles:
        # test the tile and its six neighbors
        for dx, dy in ((0, 0),) + NEIGHBORS:
            x, y = black_x + dx, black_y + dy

            # but only once
            if (x, y) in seen:
                continue
            seen.add((x, y))

            is_black = (x, y) in tiles

            black_neighbors = sum(1 for nx, ny in NEIGHBORS if (x + nx, y + ny) in tiles)

            if is_black and (black_neighbors == 0 or black_neighbors > 2):
                new_is_black = False
            elif not is_black and (black_neighbors == 2):
                new_is_black = True
            else:
                new_is_black = is_black

            if new_is_black:
                new_tiles.add((x, y))

    tiles = new_tiles

print(len(tiles))
