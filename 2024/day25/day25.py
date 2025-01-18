#!/usr/bin/env python3
# [Day 25: Code Chronicle](https://adventofcode.com/2024/day/25)


import atexit
import time
from argparse import ArgumentParser
from pathlib import Path

parser = ArgumentParser()
parser.add_argument("-v", "--verbose", action="store_true")
parser.add_argument("-t", "--test", action="store_true")
parser.add_argument("--elapsed", action="store_true")
parser.add_argument("filename", nargs="?", type=Path, default="input.txt")
args = parser.parse_args()
if args.test:
    args.filename = Path("test.txt")

data = args.filename.read_text()

if args.elapsed:
    start_time_ns = time.time_ns()
    atexit.register(lambda: print(f"elapsed: {(time.time_ns() - start_time_ns) / 1_000_000}ms"))


keys = []
locks = []

for schematics in data.strip().split("\n\n"):

    schematics = schematics.splitlines()
    assert len(schematics) == 7

    heights = [-1, -1, -1, -1, -1]
    for line in schematics:
        assert len(line) == 5
        for x, c in enumerate(line):
            if c == "#":
                heights[x] += 1

    if schematics[0] == "#####":
        locks.append(heights)
    elif schematics[6] == "#####":
        keys.append(heights)
    else:
        assert False


print(sum(all(a + b <= 5 for (a, b) in zip(key, lock)) for lock in locks for key in keys))
