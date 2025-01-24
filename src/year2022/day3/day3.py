#!/usr/bin/env python3

import string
import sys


def priority(items):
    p = 0
    for item in items:
        if item == "\n":
            continue

        p = string.ascii_lowercase.find(item)
        if p >= 0:
            p += 1
        else:
            p = string.ascii_uppercase.find(item)
            assert p >= 0
            p += 27
    return p


filename = ("test.txt" if sys.argv[1] == "-t" else sys.argv[1]) if len(sys.argv) > 1 else "input.txt"
data = open(filename).readlines()

part1 = 0
for rucksack in data:
    n = len(rucksack) // 2
    compartment1 = set(rucksack[0:n])
    compartment2 = set(rucksack[n:])
    part1 += priority(compartment1.intersection(compartment2))
print(part1)

part2 = 0
for i in range(0, len(data), 3):
    s = set(data[i]).intersection(set(data[i + 1])).intersection(set(data[i + 2]))
    part2 += priority(s)
print(part2)
