#!/usr/bin/env python3
# https://adventofcode.com/2017/day/11

import sys
from pathlib import Path

filename = ("test.txt" if sys.argv[1] == "-t" else sys.argv[1]) if len(sys.argv) > 1 else "input.txt"
data = Path(filename).read_text().strip()


STEP_X = 1  # 1.5
STEP_Y = 1  # math.sqrt(3) / 2


def steps(x, y):
    x = abs(x)
    y = abs(y)

    s = 0
    while x != 0:
        s += 1
        if x >= y:
            x -= STEP_X
            y -= STEP_Y
        else:
            y -= STEP_Y * 2

    return s


def walk(data):
    m = 0
    x, y = 0, 0
    for d in data.split(","):
        match d:
            case "ne":
                x = x + STEP_X
                y = y + STEP_Y
            case "se":
                x = x + STEP_X
                y = y - STEP_Y
            case "s":
                y = y - STEP_Y * 2
            case "n":
                y = y + STEP_Y * 2
            case "nw":
                x = x - STEP_X
                y = y + STEP_Y
            case "sw":
                x = x - STEP_X
                y = y - STEP_Y
            case _:
                raise ValueError(d)

        m = max(m, steps(x, y))

    print(steps(x, y))
    print(m)


walk(data)
