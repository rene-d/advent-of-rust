#!/usr/bin/env python3

from pathlib import Path

data = Path("input.txt").read_text().splitlines()

polymer = data[0]


transforms = {}
for line in data[2:]:
    transforms[line[0:2]] = line[-1]

for _ in range(10):
    new = ""

    for i in range(len(polymer) - 1):
        pair = polymer[i : i + 2]
        new += pair[0] + transforms[pair]

    polymer = new + pair[1]

counts = [0] * 26
for c in polymer:
    counts[ord(c) - ord("A")] += 1
M = max(counts)
m = min(c for c in counts if c > 0)
print(M - m)
