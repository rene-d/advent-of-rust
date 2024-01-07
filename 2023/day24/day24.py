#!/usr/bin/env python3
# [Day 24: Never Tell Me The Odds](https://adventofcode.com/2023/day/24)

import sys
from collections import Counter, namedtuple

# from fractions import Fraction
from pathlib import Path

import sympy


verbose = "-v" in sys.argv
if verbose:
    sys.argv.remove("-v")
filename = ("test.txt" if sys.argv[1] == "-t" else sys.argv[1]) if len(sys.argv) > 1 else "input.txt"
data = Path(filename).read_text().strip()
lines = data.splitlines()

Hailstone = namedtuple("Hailstone", "x y z vx vy vz")

hailstones = [Hailstone(*map(int, line.replace("@", ",").split(","))) for line in lines]
n = len(hailstones)


# part 1

a_min, a_max = (7, 27) if filename == "test.txt" else (200000000000000, 400000000000000)

result = 0
for i in range(n):
    for j in range(i + 1, n):
        a = hailstones[i]
        b = hailstones[j]

        determinant = b.vy * a.vx - a.vy * b.vx

        if determinant != 0:
            # point of intersection
            y = ((b.x - a.x) + a.y * a.vx / a.vy - b.y * b.vx / b.vy) / (a.vx / a.vy - b.vx / b.vy)
            x = (y - a.y) * a.vx / a.vy + a.x

            # oriented intersection
            intersect_a = (x > a.x) == (a.vx > 0)
            intersect_b = (x > b.x) == (b.vx > 0)

            if a_min <= x <= a_max and a_min <= y <= a_max and intersect_a and intersect_b:
                result += 1

print(result)


# part 2

x, y, z, vx, vy, vz = sympy.symbols("x,y,z,vx,vy,vz")

equations = []

# 9 equations, 6+3=9 variables
for i, hail in enumerate(hailstones[:3]):
    t = sympy.var(f"t{i}")

    equations.append(sympy.Eq(x + vx * t, hail.x + hail.vx * t))
    equations.append(sympy.Eq(y + vy * t, hail.y + hail.vy * t))
    equations.append(sympy.Eq(z + vz * t, hail.z + hail.vz * t))

sol = sympy.solve(equations)[0]
print(sol[x] + sol[y] + sol[z])
