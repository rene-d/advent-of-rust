#!/usr/bin/env python3
# https://adventofcode.com/2019/day/1

from pathlib import Path
import sys

filename = "test.txt" if len(sys.argv) > 1 and sys.argv[1] == "-t" else "input.txt"
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
