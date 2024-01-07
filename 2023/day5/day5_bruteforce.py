#!/usr/bin/env python3
# [Day 5: If You Give A Seed A Fertilizer](https://adventofcode.com/2023/day/5)

import sys
from pathlib import Path

filename = ("test.txt" if sys.argv[1] == "-t" else sys.argv[1]) if len(sys.argv) > 1 else "input.txt"
data = Path(filename).read_text().strip()
lines = data.splitlines()

maps = {}
for line in lines:
    if line.startswith("seeds: "):
        line = line.removeprefix("seeds: ")
        seeds = list(map(int, line.split()))
    elif line.endswith(" map:"):
        line = line.removesuffix(" map:")
        current_map = []
        maps[line] = current_map
    elif line:
        current_map.append(list(map(int, line.split())))


def convert(map, seed):
    map = maps[map]

    for b, a, n in map:
        if a <= seed < a + n:
            return b - a + seed

    return seed


def converts(seed):
    seed = convert("seed-to-soil", seed)
    seed = convert("soil-to-fertilizer", seed)
    seed = convert("fertilizer-to-water", seed)
    seed = convert("water-to-light", seed)
    seed = convert("light-to-temperature", seed)
    seed = convert("temperature-to-humidity", seed)
    seed = convert("humidity-to-location", seed)
    return seed


min_location = float("inf")
for seed in seeds:
    seed = converts(seed)
    min_location = min(min_location, seed)

print(min_location)


m = float("inf")
for start, n in zip(seeds[::2], seeds[1::2]):
    for seed in range(start, start + n):
        m = min(m, converts(seed))
print(m)
