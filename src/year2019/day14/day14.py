#!/usr/bin/env python3
# [Day 14: Space Stoichiometry](https://adventofcode.com/2019/day/14)

import sys
from collections import defaultdict
from pathlib import Path

filename = ("test.txt" if sys.argv[1] == "-t" else sys.argv[1]) if len(sys.argv) > 1 else "input.txt"

data = Path(filename).read_text()
lines = data.splitlines()

reactions = {}
for line in lines:
    a, b = line.split(" => ")

    qp, p = b.split()

    r = []
    for qr in a.split(", "):
        q, n = qr.split()
        r.append((int(q), n))

    reactions[p] = (int(qp), r)


surplus = defaultdict(lambda: 0)


def calc(demande, chemical):
    if chemical == "FUEL":
        surplus.clear()

    if chemical == "ORE":
        return demande

    if demande <= surplus[chemical]:
        surplus[chemical] -= demande
        return 0

    demande -= surplus[chemical]
    surplus[chemical] = 0

    q_produite, reactifs = reactions[chemical]
    nb_reactions = (q_produite - 1 + demande) // q_produite

    surplus[chemical] += nb_reactions * q_produite - demande

    s = 0
    for i in reactifs:
        s += calc(nb_reactions * i[0], i[1])

    return s


# part 1
print(calc(1, "FUEL"))


# part 2
ore = 1_000_000_000_000

a = 1
b = ore
while b - a > 1:
    m = (a + b) // 2
    c = calc(m, "FUEL")
    if c == ore:
        break
    if c > ore:
        b = m
    else:
        a = m

print(a)
