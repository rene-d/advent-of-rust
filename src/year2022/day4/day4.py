#!/usr/bin/env python3

import re
import sys

part1 = 0
part2 = 0

filename = ("test.txt" if sys.argv[1] == "-t" else sys.argv[1]) if len(sys.argv) > 1 else "input.txt"

for line in open(filename):
    m = re.match(r"(\d+)-(\d+),(\d+)-(\d+)", line)
    a, b, c, d = map(int, m.groups())

    # [a,b] contained into [c,d] or vice versa
    if a <= c <= d <= b or c <= a <= b <= d:
        part1 += 1

    # [a,b] and [c,d] overlap ?
    if max(0, min(b, d) - max(a, c) + 1) != 0:
        part2 += 1

print(part1)
print(part2)
