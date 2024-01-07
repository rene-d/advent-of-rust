#!/usr/bin/env python3
# [Day 14: Docking Data](https://adventofcode.com/2020/day/14)

import re
import sys
from pathlib import Path

verbose = "-v" in sys.argv
if verbose:
    sys.argv.remove("-v")
filename = ("test.txt" if sys.argv[1] == "-t" else sys.argv[1]) if len(sys.argv) > 1 else "input.txt"
data = Path(filename).read_text().strip()
lines = data.splitlines()


# part 1

mem = {}
for line in lines:
    if line.startswith("mask = "):
        mask = line[7:]

        or_mask = int(mask.replace("X", "0"), 2)
        and_mask = int(mask.replace("X", "1"), 2)
    else:
        addr, value = map(int, re.match(r"mem\[(\d+)] = (\d+)", line).groups())

        mem[addr] = (value & and_mask) | or_mask

print(sum(mem.values()))


# part 2

mem = {}
for line in lines:
    if line.startswith("mask = "):
        mask = line[7:]

        # mask to cancel X bits
        and_mask = int(mask.replace("0", "1").replace("X", "0"), 2)

    else:
        addr, value = map(int, re.match(r"mem\[(\d+)] = (\d+)", line).groups())

        n = 1 << mask.count("X")
        for i in range(n):
            or_mask = 0
            for bit, digit in enumerate(mask):
                if digit == "X":
                    digit = i % 2
                    i //= 2
                else:
                    digit = int(digit)
                or_mask = (or_mask << 1) + digit

            mem[or_mask | (addr & and_mask)] = value

print(sum(mem.values()))
