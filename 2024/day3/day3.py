#!/usr/bin/env python3
# [Day 3: Mull It Over](https://adventofcode.com/2024/day/3)

from pathlib import Path
import sys, re
import re


if verbose := "-v" in sys.argv:
    sys.argv.remove("-v")
if self_tests := "-T" in sys.argv:
    sys.argv.remove("-T")
filename = ("test.txt" if sys.argv[1] == "-t" else sys.argv[1]) if len(sys.argv) > 1 else "input.txt"


def solve(data: str, part1: bool):

    enabled = True
    total_sum = 0
    i = 0

    while i < len(data):

        if data[i : i + 4] == "do()":
            enabled = True
            i += 4

        elif data[i : i + 7] == "don't()":
            enabled = False
            i += 7

        elif data[i : i + 4] == "mul(":
            if m := re.match(r"mul\((\d+),(\d+)\)", data[i:]):
                x, y = map(int, m.groups())
                if enabled or part1:
                    total_sum += x * y
                i += len(m.group(0))
            else:
                i += 4

        else:
            i += 1

    return total_sum


if self_tests:
    assert solve(Path("sample_1.txt").read_text(), True) == 161
    assert solve(Path("sample_2.txt").read_text(), False) == 48

else:
    data = Path(filename).read_text().strip()
    print(solve(data, True))
    print(solve(data, False))
