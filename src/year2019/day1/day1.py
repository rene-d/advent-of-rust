#!/usr/bin/env python3
# [Day 1: The Tyranny of the Rocket Equation](https://adventofcode.com/2019/day/1)

import sys
from pathlib import Path

filename = ("test.txt" if sys.argv[1] == "-t" else sys.argv[1]) if len(sys.argv) > 1 else "input.txt"
data = Path(filename).read_text()
lines = data.splitlines()

print(sum(int(mass) // 3 - 2 for mass in lines))

part2 = 0
for mass in lines:
    fuel = int(mass)
    while True:
        fuel = fuel // 3 - 2
        if fuel <= 0:
            break
        part2 += fuel
print(part2)
