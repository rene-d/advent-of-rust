#!/usr/bin/env python3
# [Day 10: Monitoring Station](https://adventofcode.com/2019/day/10)

import sys
from collections import namedtuple
from math import atan2, gcd, pi
from pathlib import Path
from typing import Tuple

filename = sys.argv[1] if len(sys.argv) > 1 else "input.txt"
data = Path(filename).read_text()
lines = data.splitlines()

# coordinates
Coord = namedtuple("Coord", ("x", "y"))

# target of the giant rotating laser
Target = namedtuple("Target", ("angle", "distance", "coord"))

asteroids = list()

for dy, line in enumerate(lines):
    for dx, c in enumerate(line):
        if c != ".":
            asteroids.append(Coord(dx, dy))


def insight_vector(asteroid: Coord, other: Coord) -> Tuple[int, int]:
    """
    Compute the direction vector between two asteroids.
    Asteroids are considred aligned if their coords are multiple of the same vector.
    So, only one asteroid can be detected for this 'irreductible' vector (i.e. with gcd(x,y)=1).
    """
    dx = other.x - asteroid.x
    dy = other.y - asteroid.y
    d = gcd(dx, dy)
    return (dx // d, dy // d)


def d_square(asteroid: Coord, other: Coord) -> int:
    """Return the square of the distance between two asteroids."""
    return (other.x - asteroid.x) ** 2 + (other.y - asteroid.y) ** 2


def angle(asteroid: Coord, other: Coord) -> float:
    """Returns the angle in radians between two asteroids, 0 is north."""
    return atan2(other.x - asteroid.x, asteroid.y - other.y) % (2 * pi)


# part 1

detected = []
for asteroid in asteroids:
    in_sight = set()
    for other in asteroids:
        if other != asteroid:
            in_sight.add(insight_vector(asteroid, other))
    detected.append((len(in_sight), asteroid))

part1, station = max(detected)
print(part1)


# part 2

vaporized = 0

while len(asteroids) > 1:
    targets = {}

    for asteroid in asteroids:
        if asteroid != station:
            v = insight_vector(station, asteroid)
            d = d_square(station, asteroid)
            a = angle(station, asteroid)

            # save or update the nearest target to the station
            t = targets.get(v)
            if not t or t.distance > d:
                targets[v] = Target(a, d, asteroid)

    # sort targets in order of their vaporization (low angle first)
    targets = sorted(targets.values())

    for target in targets:
        vaporized += 1
        if vaporized == 200:
            t = target.coord
            print(t.x * 100 + t.y)
            break
        else:
            asteroids.remove(target.coord)
