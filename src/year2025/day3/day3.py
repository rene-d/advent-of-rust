#!/usr/bin/env python3
# [Day 3: Lobby](https://adventofcode.com/2025/day/3)

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


def largest_joltage(bank: str) -> int:
    """Part 1."""
    return max(int(bank[i]) * 10 + int(max(bank[i + 1 :])) for i in range(len(bank) - 1))


def largest_joltage_n(bank: str, n: int = 12) -> str:
    """Part 2."""
    batteries = 0
    remove = len(bank) - n

    for battery in bank:
        while remove > 0 and batteries > 0 and batteries % 10 < int(battery):
            batteries //= 10
            remove -= 1
        batteries = 10 * batteries + int(battery)

    while remove > 0:
        batteries //= 10
        remove -= 1

    return batteries


print(sum(largest_joltage(line) for line in data.splitlines()))
print(sum(largest_joltage_n(line) for line in data.splitlines()))
