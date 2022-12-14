#!/usr/bin/env python3
# https://adventofcode.com/2022/day/13

from pathlib import Path
import sys
from functools import cmp_to_key


def cmp(a, b):

    if isinstance(a, int) and isinstance(b, int):
        if a < b:
            return -1
        elif a == b:
            return 0
        else:
            return 1

    if isinstance(a, int):
        a = [a]

    if isinstance(b, int):
        b = [b]

    i = 0
    while True:
        i = 0
        while i < len(a) and i < len(b):
            c = cmp(a[i], b[i])
            if c != 0:
                return c
            i += 1

        if i == len(a) and i < len(b):
            return -1
        elif i == len(b) and i < len(a):
            return 1
        else:
            return 0


filename = "test.txt" if len(sys.argv) > 1 and sys.argv[1] == "-t" else "input.txt"
data = Path(filename).read_text()
lines = data.splitlines()

# part 1
packets = []
i = 0
part1 = 0
while True:
    if i >= len(lines):
        break
    left = eval(lines[i])
    right = eval(lines[i + 1])
    packets.append(left)
    packets.append(right)
    i += 3
    if cmp(left, right) == -1:
        part1 += i // 3
print(part1)

# part 2
packets.append([[2]])
packets.append([[6]])
packets = sorted(packets, key=cmp_to_key(lambda a, b: cmp(a, b)))
part2 = 1
for i, packet in enumerate(packets, 1):
    if packet == [[2]] or packet == [[6]]:
        part2 *= i
print(part2)
