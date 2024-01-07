#!/usr/bin/env python3
# [Day 5: Binary Boarding](https://adventofcode.com/2020/day/5)

import sys
from pathlib import Path

filename = ("test.txt" if sys.argv[1] == "-t" else sys.argv[1]) if len(sys.argv) > 1 else "input.txt"
data = Path(filename).read_text().strip()
lines = data.splitlines()


def parse(seat):
    a = 0
    b = 127
    for letter in seat[:7]:
        if letter == "F":
            # lower half
            b = (b - a + 1) // 2 - 1 + a
        elif letter == "B":
            # upper half
            a = b + 1 - (b - a + 1) // 2
    row = a

    a = 0
    b = 7
    for letter in seat[7:]:
        if letter == "L":
            # lower half
            b = (b - a + 1) // 2 - 1 + a
        elif letter == "R":
            # upper half
            a = b + 1 - (b - a + 1) // 2
    column = a

    id = row * 8 + column

    return id


# part 1
seats = list(parse(seat) for seat in lines)
print(max(seats))

# part 2
seats = sorted(seats)
for i in range(len(seats) - 1):
    if seats[i + 1] - seats[i] == 2:
        print(seats[i] + 1)
        break
