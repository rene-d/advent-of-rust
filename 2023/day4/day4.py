#!/usr/bin/env python3
# https://adventofcode.com/2023/day/4

import sys
from pathlib import Path

filename = ("test.txt" if sys.argv[1] == "-t" else sys.argv[1]) if len(sys.argv) > 1 else "input.txt"
data = Path(filename).read_text().strip()
lines = data.splitlines()

# parse puzzle input
matching_cards = list()
for line in lines:
    winning, nums = line.split(": ")[1].split(" | ")

    winning = list(map(int, winning.split()))
    nums = list(map(int, nums.split()))

    matching_cards.append(len(set(winning).intersection(set(nums))))

# part 1
print(sum(2 ** (p - 1) for p in matching_cards if p > 0))

# part 2
copies = [0] * len(matching_cards)
for k, m in enumerate(matching_cards):
    copies[k] += 1
    for i in range(k + 1, k + m + 1):
        copies[i] += copies[k]
print(sum(copies))
