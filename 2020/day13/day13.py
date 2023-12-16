#!/usr/bin/env python3
# https://adventofcode.com/2020/day/13

import sys
from functools import reduce
from pathlib import Path

verbose = "-v" in sys.argv
if verbose:
    sys.argv.remove("-v")
filename = ("test.txt" if sys.argv[1] == "-t" else sys.argv[1]) if len(sys.argv) > 1 else "input.txt"
data = Path(filename).read_text().strip()
lines = data.splitlines()

depart = int(lines[0])
bus = lines[1].split(",")

# part 1

m = []
for id in bus:
    if id == "x":
        continue
    id = int(id)
    m.append(((depart // id + 1) * id - depart, id))
min, id = min(m)
print(min * id)


# part 2


def mul_inv(a, b):
    b0 = b
    x0, x1 = 0, 1
    if b == 1:
        return 1
    while a > 1:
        q = a // b
        a, b = b, a % b
        x0, x1 = x1 - q * x0, x0
    if x1 < 0:
        x1 += b0
    return x1


def chinese_remainder(n, a):
    sum = 0
    prod = reduce(lambda a, b: a * b, n)
    for n_i, a_i in zip(n, a):
        p = prod // n_i
        sum += a_i * mul_inv(p, n_i) * p
    return sum % prod


n = []
a = []
for t, id in enumerate(bus):
    if id != "x":
        id = int(id)
        n.append(id)
        a.append(id - t)
print(chinese_remainder(n, a))
