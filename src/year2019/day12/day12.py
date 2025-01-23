#!/usr/bin/env python3
# [Day 12: The N-Body Problem](https://adventofcode.com/2019/day/12)

import re
import sys
from collections import namedtuple
from copy import deepcopy
from math import lcm
from pathlib import Path

filename = ("test.txt" if sys.argv[1] == "-t" else sys.argv[1]) if len(sys.argv) > 1 else "input.txt"
data = Path(filename).read_text()
lines = data.splitlines()


Coord = namedtuple("Coord", ("x", "y", "z"))

moons = []
for line in lines:
    m = re.match(r"<x=(-?\d+), y=(-?\d+), z=(-?\d+)>", line)
    moon = list(map(int, m.groups()))
    moons.append(moon)

velocities = [[0, 0, 0] for _ in moons]
initial = deepcopy(moons)


def gravity():
    for i, a in enumerate(moons):
        for j, b in enumerate(moons):
            if i < j:
                for k in range(3):
                    if a[k] > b[k]:
                        velocities[i][k] -= 1
                        velocities[j][k] += 1
                    elif a[k] < b[k]:
                        velocities[i][k] += 1
                        velocities[j][k] -= 1

    for i in range(len(moons)):
        for k in range(3):
            moons[i][k] += velocities[i][k]


def energy(v):
    return sum(map(abs, v))


def system_energy():
    return sum(energy(p) * energy(v) for p, v in zip(moons, velocities))


# part 1
for i in range(1000):
    gravity()
print(system_energy())

# part2
moons = deepcopy(initial)
velocities = [[0, 0, 0] for _ in moons]
step = 0
steps = [0] * 3
remaining = 3
while remaining != 0:
    gravity()
    step += 1
    for k in range(3):
        if steps[k] == 0:
            # look for cycles for each coordinate since they are independent
            if all(velocities[j][k] == 0 and moons[j][k] == initial[j][k] for j in range(4)):
                steps[k] = step
                remaining -= 1
                if remaining == 0:
                    break
print(lcm(*steps))
