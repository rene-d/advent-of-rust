#!/usr/bin/env python3

import re

part1 = 0
part2 = 0

for line in open("input.txt"):
    m = re.match(r"(\d+)-(\d+),(\d+)-(\d+)", line)
    a, b, c, d = map(int, m.groups())

    if a <= c <= d <= b or c <= a <= b <= d:
        part1 += 1

    if max(0, min(b, d) - max(a, c) + 1) != 0:
        part2 += 1

print(part1)
print(part2)
