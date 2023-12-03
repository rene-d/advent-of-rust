#!/usr/bin/env python3
# https://adventofcode.com/2019/day/6

import sys
from pathlib import Path

filename = sys.argv[1] if len(sys.argv) > 1 else "input.txt"
data = Path(filename).read_text()
lines = data.splitlines()

orbits = {}
for line in lines:
    a, b = line.split(")")
    assert b not in orbits
    orbits[b] = a


# part 1

total = 0
for o in orbits.keys():
    # compute the distance to COM for each object
    while o != "COM":
        o = orbits[o]
        total += 1
print(total)


# part 2

# path from YOU to COM
you_orbits = []
o = "YOU"
while o != "COM":
    o = orbits[o]
    you_orbits.append(o)

# path from SAN to COM
san_orbits = []
o = "SAN"
while o != "COM":
    o = orbits[o]
    san_orbits.append(o)

# remove common parts starting at the end
common = 0
while san_orbits[-1 - common] == you_orbits[-1 - common]:
    common += 1

# the remaining is the wanted distance
print(len(you_orbits) + len(san_orbits) - common * 2)
