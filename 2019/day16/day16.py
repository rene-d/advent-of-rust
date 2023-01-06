#!/usr/bin/env python3
# https://adventofcode.com/2019/day/16

from pathlib import Path
import sys
from copy import deepcopy
from collections import defaultdict, deque
import re

filename = ("test.txt" if sys.argv[1] == "-t" else sys.argv[1]) if len(sys.argv) > 1 else "input.txt"
data = Path(filename).read_text().strip()

pattern = [0, 1, 0, -1]


def fft(signal):
    t = [0] * len(signal)
    for n in range(1, 1 + len(signal)):
        s = 0
        for i, b in enumerate(signal, 1):
            p = pattern[(i // n) % 4]
            s += b * p
        t[n - 1] = abs(s) % 10
    return t


def part1(signal):
    signal = list(map(int, signal))

    for _ in range(100):
        signal = fft(signal)

    return "".join(map(str, signal[:8]))


print(part1(data))

# part 2
offset = int(data[:7])
n = len(data) * 10000 - offset
p = [0] * n
t = [0] * n

for i in range(n):
    p[i] = int(data[(i + offset) % len(data)])

for _ in range(100):
    s = 0
    for i in range(n - 1, -1, -1):
        s += p[i]
        s %= 10
        t[i] = s
    p = t
print("".join(map(str, p[:8])))
