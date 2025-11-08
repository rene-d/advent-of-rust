#!/usr/bin/env python3
# [Day 1: Report Repair](https://adventofcode.com/2020/day/1)

import atexit
import sys
import time
from pathlib import Path

filename = ("test.txt" if sys.argv[1] == "-t" else sys.argv[1]) if len(sys.argv) > 1 else "input.txt"
data = Path(filename).read_text().strip()
if "--elapsed" in sys.argv:
    sys.argv.remove("--elapsed")
    start_time_ns = time.time_ns()
    atexit.register(lambda: print(f"elapsed: {(time.time_ns() - start_time_ns) / 1_000_000}ms"))


lines = list(map(int, data.splitlines()))
length = len(lines)


def part1():
    for i in range(length):
        for j in range(length):
            if i != j:
                if lines[i] + lines[j] == 2020:
                    print(lines[i] * lines[j])
                    return


def part2():
    for i in range(length):
        for j in range(length):
            if i == j:
                continue
            p = lines[i] + lines[j]
            if p > 2020:
                continue
            for k in range(length):
                if i != j and i != k and j != k:
                    if p + lines[k] == 2020:
                        print(lines[i] * lines[j] * lines[k])
                        return


part1()
part2()
