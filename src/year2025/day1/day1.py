#!/usr/bin/env python3
# [Day 1: Secret Entrance](https://adventofcode.com/2025/day/1)

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


def part1(ops):
    pos = 50
    count_zero = 0

    for d, n in ops:
        if d == "L":
            pos = (pos - n) % 100
        else:  # R
            pos = (pos + n) % 100

        if pos == 0:
            count_zero += 1

    return count_zero


def part2(ops):
    pos = 50
    count_zero = 0

    for d, n in ops:
        step = -1 if d == "L" else 1

        for _ in range(n):
            pos = (pos + step) % 100
            if pos == 0:
                count_zero += 1

    return count_zero


ops = [(line[0], int(line[1:])) for line in data.splitlines()]
print(part1(ops))
print(part2(ops))
