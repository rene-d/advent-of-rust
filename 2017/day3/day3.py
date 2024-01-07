#!/usr/bin/env python3
# [Day 3: Spiral Memory](https://adventofcode.com/2017/day/3)

import sys
from pathlib import Path

filename = ("test.txt" if sys.argv[1] == "-t" else sys.argv[1]) if len(sys.argv) > 1 else "input.txt"
data = Path(filename).read_text().strip()
data = int(data)


def spirale1(n):
    x, y = 0, 0
    i = 0
    m = 1
    dx, dy = 1, 0

    while n > 1:
        n -= 1
        i += 1

        if i == (m * 2 + 1) ** 2:
            x += 1
            m += 1
        else:
            x += dx
            y += dy

            if y + dy > m:
                dx, dy = 1, 0
            elif x + dx < -m:
                dx, dy = 0, 1
            elif y + dy < -m:
                dx, dy = -1, 0
            elif x + dx > m:
                dx, dy = 0, -1

    print(abs(x) + abs(y))


def spirale2(n):
    t = [[0] * 11 for i in range(11)]

    half = len(t) // 2

    x, y = 0, 0
    i = 0
    m = 1
    dx, dy = 1, 0

    t[y + half][x + half] = 1

    while True:
        i += 1

        if x == y == 0:
            z = 1
        else:
            z = 0
            for ix, iy in (
                (-1, -1),
                (-1, 0),
                (-1, 1),
                (0, -1),
                (0, 1),
                (1, -1),
                (1, 0),
                (1, 1),
            ):
                z += t[y + iy + half][x + ix + half]

        if z >= n:
            break

        t[y + half][x + half] = z

        if i == (m * 2 + 1) ** 2:
            x += 1
            m += 1
        else:
            x += dx
            y += dy

            if y + dy > m:
                dx, dy = 1, 0
            elif x + dx < -m:
                dx, dy = 0, 1
            elif y + dy < -m:
                dx, dy = -1, 0
            elif x + dx > m:
                dx, dy = 0, -1

    # print(tabulate.tabulate(t, tablefmt="pretty"))
    print(z)


spirale1(data)
spirale2(data)
