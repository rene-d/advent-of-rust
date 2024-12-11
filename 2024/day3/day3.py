#!/usr/bin/env python3
# [Day 3: Mull It Over](https://adventofcode.com/2024/day/3)

import re
import sys
from pathlib import Path

if verbose := "-v" in sys.argv:
    sys.argv.remove("-v")
if self_tests := "-T" in sys.argv:
    sys.argv.remove("-T")
filename = ("test.txt" if sys.argv[1] == "-t" else sys.argv[1]) if len(sys.argv) > 1 else "input.txt"


def part1(data: str) -> int:
    return sum(int(m[1]) * int(m[2]) for m in re.finditer(r"mul\((\d+),(\d+)\)", data))


def part2(data: str) -> int:
    enabled = True
    total_sum = 0
    for m in re.finditer(r"mul\((\d+),(\d+)\)|don't\(\)|do\(\)", data):
        if m[0] == "do()":
            enabled = True
        elif m[0] == "don't()":
            enabled = False
        elif enabled:
            total_sum += int(m[1]) * int(m[2])

    return total_sum


if self_tests:
    assert part1(Path("sample_1.txt").read_text()) == 161
    assert part2(Path("sample_2.txt").read_text()) == 48

else:
    data = Path(filename).read_text().strip()
    print(part1(data))
    print(part2(data))
