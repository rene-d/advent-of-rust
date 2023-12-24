#!/usr/bin/env python3
# https://adventofcode.com/2023/day/24

import sys
from collections import Counter

# from fractions import Fraction
from pathlib import Path

from scipy.optimize import fsolve

verbose = "-v" in sys.argv
if verbose:
    sys.argv.remove("-v")
filename = ("test.txt" if sys.argv[1] == "-t" else sys.argv[1]) if len(sys.argv) > 1 else "input.txt"
data = Path(filename).read_text().strip()
lines = data.splitlines()

hailstones = [list(map(int, line.replace("@", ",").split(","))) for line in lines]
n = len(hailstones)


# part 1

a_min, a_max = (7, 27) if filename == "test.txt" else (200000000000000, 400000000000000)

result = 0
for i in range(n):
    for j in range(i + 1, n):
        intersect_a = hailstones[i]
        intersect_b = hailstones[j]

        x0, y0, _, vx, vy, _ = intersect_a  # map(Fraction, a)
        x1, y1, _, wx, wy, _ = intersect_b  # map(Fraction, b)

        determinant = wy * vx - vy * wx

        if determinant != 0:
            # point of intersection
            y = ((x1 - x0) + y0 * vx / vy - y1 * wx / wy) / (vx / vy - wx / wy)
            x = (y - y0) * vx / vy + x0

            # oriented intersection
            intersect_a = (x > x0) == (vx > 0)
            intersect_b = (x > x1) == (wx > 0)

            if a_min <= x <= a_max and a_min <= y <= a_max and intersect_a and intersect_b:
                result += 1

print(result)


# part 2

sols = []
for k in range(1, n - 3):

    def equations(p):
        x, y, z, vx, vy, vz = p
        res = []
        for hailstone in hailstones[k : k + 3]:
            x1, y1, z1, wx, wy, wz = hailstone
            res.append((x - x1) * (vy - wy) - (y - y1) * (vx - wx))
            res.append((x - x1) * (vz - wz) - (z - z1) * (vx - wx))
        return res

    x, y, z, vx, vy, vz = fsolve(equations, hailstones[0])
    sol = round(x) + round(y) + round(z)
    sols.append(sol)


# Nota
# k=0 works nicely (and lovely) with my input and the sample, but not for all other inputs.
# So, I had to try other combinations and retain the most frequent root.
# Ugly.

sols = Counter(sols).items()
sol = sorted(sols, key=lambda v: v[1], reverse=True)[0][0]

print(sol)
