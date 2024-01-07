#!/usr/bin/env python3
# [Day 20: Jurassic Jigsaw](https://adventofcode.com/2020/day/20)

import sys
from collections import Counter
from copy import deepcopy
from functools import reduce
from operator import mul
from pathlib import Path

verbose = "-v" in sys.argv
if verbose:
    sys.argv.remove("-v")
filename = ("test.txt" if sys.argv[1] == "-t" else sys.argv[1]) if len(sys.argv) > 1 else "input.txt"
data = Path(filename).read_text().strip()

tiles = {}

for tile in data.split("\n\n"):
    tile = tile.splitlines()

    assert len(tile) == 1 + 10
    assert all(len(row) == 10 for row in tile)

    # "Tile 3011:"
    id = int(tile[0][5:-1])

    tile = [[c for c in row] for row in tile[1:]]

    t = []

    for _ in range(2):
        for _ in range(4):
            # read the binary signature of the tile (first row)
            b0 = 0
            for bit in range(10):
                # up
                if tile[0][bit] == "#":
                    b0 += 1 << bit
            t.append((b0, deepcopy(tile)))

            # rotate
            tile2 = [[0 for _ in range(10)] for _ in range(10)]
            for x in range(10):
                for y in range(10):
                    tile2[y][x] = tile[9 - x][y]
            tile = tile2

        # flip
        for y in range(10):
            for x in range(5):
                tile[y][x], tile[y][9 - x] = tile[y][9 - x], tile[y][x]

    # { id: [(sig, tile), (sig, tile), ...], }
    tiles[id] = t


# part 1

counts = list()
for k, v in tiles.items():
    for sig, _ in v:
        counts.append(sig)
counts = Counter(counts)
# counts[]==1 <=> tile on map boundaries. if count of 4, it's a corner
corners = list(id for id, v in tiles.items() if sum(1 for k, _ in v if counts[k] == 1) == 4)
print(reduce(mul, corners))


# part 2

N = int(len(tiles) ** 0.5)
big_grid = [["?" for _ in range(N * 10)] for _ in range(N * 10)]
monster_grid = [["?" for _ in range(N * 8)] for _ in range(N * 8)]


def reverse_sig(b):
    """Reverse the binary signature."""
    r = 0
    for i in range(10):
        r *= 2
        if b & 1 == 1:
            r += 1
        b //= 2
    return r


def opposite_edge(id, sig):
    """Get the opposite edge of a tile relative to a sig."""
    t = tiles[id]
    sig = reverse_sig(sig)
    for i, (b, _) in enumerate(t):
        if sig == b:
            return t[((i % 4) + 2) % 4 + 4 * (i // 4)][0]


def adj_edge(id, sig):
    """Get the left edge of a tile relative to a sig."""
    t = tiles[id]

    for i, (b, _) in enumerate(t):
        if sig == b:
            return t[((i % 4) + 3) % 4 + 4 * (i // 4)][0]


def print_tile(id, top_sig, x, y):
    """Print the tile into the map."""
    # print(id, top_sig, bin(top_sig))
    for b, v in tiles[id]:
        if top_sig == b:
            # print("\n".join("".join(c for c in row) for row in v))

            for i in range(10):
                for j in range(10):
                    big_grid[i + y * 10][j + x * 10] = v[i][j]

            for i in range(8):
                for j in range(8):
                    monster_grid[i + y * 8][j + x * 8] = v[i + 1][j + 1]

            break


def next_tile(prev_id, sig):
    """Find the next tile (same sig, but different id)."""
    for id, v in tiles.items():
        if id != prev_id:
            for b, _ in v:
                if b == sig:
                    return id


edges = []
id = corners[0]
for i in range(4):
    sig1 = tiles[id][i][0]
    sig2 = tiles[id][(i + 1) % 4][0]
    if counts[sig1] == 2 and counts[sig2] == 2:
        edges.append(sig1)
        edges.append(sig2)
        break


x, y = 0, 0

id_top = id
bottom = opposite_edge(id, edges[1])
right = edges[0]


# fill the map
while True:
    y = 0
    while id:
        print_tile(id, bottom, x, y)
        y += 1
        bottom = opposite_edge(id, bottom)
        id = next_tile(id, bottom)

    x += 1
    id_top = next_tile(id_top, right)
    if not id_top:
        break
    id = id_top
    bottom = adj_edge(id, right)
    right = opposite_edge(id, right)


if verbose:
    for y, row in enumerate(big_grid):
        s = ""
        for x, c in enumerate(row):
            s += c
            if x % 10 == 9:
                s += " "
        print(s)
        if y % 10 == 9:
            print()


monster_def = """\
                  #
#    ##    ##    ###
 #  #  #  #  #  #
"""

# parse the monster
monster_def = monster_def.splitlines()
MX = max(len(line) for line in monster_def)
MY = len(monster_def)
monster = []
monster_length = 0
for y in range(MY):
    m = []
    for x in range(MX):
        if x < len(monster_def[y]):
            if monster_def[y][x] == "#":
                monster_length += 1
            m.append(monster_def[y][x])
        else:
            m.append(" ")
    monster.append(m)

# look for the monster
found = 0
for _ in range(2):
    for _ in range(4):
        # search
        for y in range(N * 8 - MY):
            for x in range(N * 8 - MX):
                m = 0
                for my in range(MY):
                    for mx in range(MX):
                        if monster_grid[y + my][x + mx] == monster[my][mx] == "#":
                            m += 1
                if monster_length == m:
                    found += 1

        # rotate
        rotate = [[0 for _ in range(N * 8)] for _ in range(N * 8)]
        for x in range(N * 8):
            for y in range(N * 8):
                rotate[y][x] = monster_grid[N * 8 - 1 - x][y]
        monster_grid = rotate

    # flip
    for y in range(N * 8):
        for x in range(N * 8 // 2):
            monster_grid[y][x], monster_grid[y][N * 8 - 1 - x] = monster_grid[y][N * 8 - 1 - x], monster_grid[y][x]

# count the '#' on the map
hashtag = 0
for y in range(N * 8):
    for x in range(N * 8):
        if monster_grid[y][x] == "#":
            hashtag += 1

# at last...
print(hashtag - found * monster_length)
