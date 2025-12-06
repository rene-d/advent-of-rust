#!/usr/bin/env python3
# [Day 6: Trash Compactor](https://adventofcode.com/2025/day/6)


import atexit
import time
from argparse import ArgumentParser
from functools import reduce
from operator import mul
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


# Part 1

table = []
for line in data.splitlines():
    if "+" in line:
        operators = list(line.split())
    else:
        table.append(list(map(int, line.split())))

part1 = 0
for i, op in enumerate(operators):
    if op == "+":
        s = 0
        for row in table:
            s += row[i]
    elif op == "*":
        s = 1
        for row in table:
            s *= row[i]
    part1 += s
print(part1)


# Part 2

table = []
for line in data.splitlines():
    if "+" in line:
        operators = line
    else:
        table.append(line)

part2 = 0
for p, op in enumerate(operators):
    if op in "+*":
        q = p + 1
        while q < len(operators) and operators[q] == " ":
            q += 1
        width = q - p
        if q < len(operators):
            width -= 1

        nums = []
        for i in range(p, p + width):
            vn = 0
            for row in table:
                if row[i] != " ":
                    vn = vn * 10 + int(row[i])
            nums.append(vn)

        if op == "+":
            part2 += sum(nums)
        elif op == "*":
            part2 += reduce(mul, nums)
print(part2)
