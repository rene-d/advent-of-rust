#!/usr/bin/env python3
# https://adventofcode.com/2020/day/12

from pathlib import Path
from copy import deepcopy
from collections import defaultdict, deque, namedtuple
import sys, re, math, itertools, time
from functools import reduce
import re

verbose = "-v" in sys.argv
if verbose:
    sys.argv.remove("-v")
filename = ("test.txt" if sys.argv[1] == "-t" else sys.argv[1]) if len(sys.argv) > 1 else "input.txt"
data = Path(filename).read_text().strip()
lines = data.splitlines()

# part 1

x, y, d = 0, 0, 90
for line in lines:
    c = line[0]
    n = int(line[1:])

    match c:
        case "N":
            y += n
        case "S":
            y -= n
        case "W":
            x -= n
        case "E":
            x += n
        case "L":
            d = (d - n) % 360
        case "R":
            d = (d + n) % 360
        case "F":
            match d:
                case 0:
                    y += n
                case 90:
                    x += n
                case 180:
                    y -= n
                case 270:
                    x -= n
                case _:
                    raise ValueError(f"bad direction {d}")
        case _:
            raise ValueError(f"bad action {line}")

    if verbose:
        print(f"{line:<5}  ship {x:5} {y:5}  direction {d:3}")

print(abs(x) + abs(y))


# part 2

x, y = 0, 0
wx, wy = 10, 1

for line in lines:
    c = line[0]
    n = int(line[1:])

    match c:
        case "N":
            wy += n
        case "S":
            wy -= n
        case "W":
            wx -= n
        case "E":
            wx += n
        case "L":
            match n:
                case 90:
                    wx, wy = -wy, wx
                case 180:
                    wx, wy = -wx, -wy
                case 270:
                    wx, wy = wy, -wx
                case _:
                    raise ValueError(f"bad angle {line}")

        case "R":
            match n:
                case 270:
                    wx, wy = -wy, wx
                case 180:
                    wx, wy = -wx, -wy
                case 90:
                    wx, wy = wy, -wx
                case _:
                    raise ValueError(f"bad angle {line}")
        case "F":
            x += wx * n
            y += wy * n
        case _:
            raise ValueError(f"bad action {c}")

    if verbose:
        print(f"{line:<5}  ship {x:5} {y:5}  waypoint {wx:5} {wy:5}")

print(abs(x) + abs(y))
