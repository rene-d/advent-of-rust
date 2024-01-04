#!/usr/bin/env python3
# https://adventofcode.com/2017/day/21

import sys
import typing as t
from copy import deepcopy
from pathlib import Path

verbose = "-v" in sys.argv
if verbose:
    sys.argv.remove("-v")
self_tests = "-T" in sys.argv
if self_tests:
    sys.argv.remove("-T")
filename = ("test.txt" if sys.argv[1] == "-t" else sys.argv[1]) if len(sys.argv) > 1 else "input.txt"
data = Path(filename).read_text().strip()
lines = data.splitlines()


Square = t.List[t.List]


def subsquare(grid: Square, x: int, y: int, n: int) -> Square:
    return list(list(grid[y + j][x + i] for i in range(n)) for j in range(n))


def rotate(square: Square) -> None:
    n = len(square)
    orig = deepcopy(square)
    for x in range(n):
        for y in range(n):
            square[y][x] = orig[n - 1 - x][y]


def flip(square: Square) -> None:
    n = len(square)
    for y in range(n):
        for x in range(n // 2):
            square[y][x], square[y][n - 1 - x] = square[y][n - 1 - x], square[y][x]


def iter_pos(grid: Square, x: int, y: int, n: int):
    square = subsquare(grid, x, y, n)
    for _ in range(4):
        rotate(square)
        yield square

    flip(square)
    yield square

    for _ in range(3):
        rotate(square)
        yield square


def pattern(square: Square) -> str:
    return "/".join("".join(row) for row in square)


def show(grid) -> None:
    print("\n".join("".join(row) for row in grid))
    print()


def enhance(rules: t.Dict[str, Square], grid: Square) -> Square:
    n = len(grid)

    if n % 2 == 0:
        m = 2
    elif n % 3 == 0:
        m = 3
    else:
        assert False

    nn = (n // m) * (m + 1)
    ng = [[None for _ in range(nn)] for _ in range(nn)]

    for y in range(n // m):
        for x in range(n // m):
            square = subsquare(grid, m * x, m * y, m)
            enhancement = rules[pattern(square)]
            for j in range(m + 1):
                for i in range(m + 1):
                    ng[y * (m + 1) + j][x * (m + 1) + i] = enhancement[j][i]

    return ng


def pixels(grid: Square) -> int:
    return sum(sum(1 for c in row if c == "#") for row in grid)


# parse the rules and build a dictionaru for all rotates/flipped rules

rules = {}

for line in lines:
    a, b = line.split(" => ", maxsplit=1)

    a = list(a.split("/"))
    b = list(b.split("/"))

    for e in iter_pos(a, 0, 0, len(a)):
        rules[pattern(e)] = b


# set the initial grid

grid = [list(".#."), list("..#"), list("###")]


# enhance the grid according to the rules

if filename == "test.txt":
    for _ in range(2):
        grid = enhance(rules, grid)
    print(pixels(grid))
    show(grid)

    assert pixels(grid) == 12

else:
    # part 1
    for _ in range(5):
        grid = enhance(rules, grid)
    print(pixels(grid))

    # part 2
    for _ in range(13):
        grid = enhance(rules, grid)
    print(pixels(grid))
