#!/usr/bin/env python3
# [Day 2: Gift Shop](https://adventofcode.com/2025/day/2)

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


part1 = 0
part2 = 0
for a_b in data.strip().split(","):
    a, b = map(int, a_b.split("-"))
    for i in range(a, b + 1):
        s = str(i)
        n = len(s)

        p = s[0 : n // 2]
        t = p * 2
        if s == t:
            part1 += i

        for k in range(1, n // 2 + 1):
            p = s[0:k]
            t = p * (n // k)
            if s == t:
                part2 += i
                break

print(part1)
print(part2)
