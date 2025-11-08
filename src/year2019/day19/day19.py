#!/usr/bin/env python3
# [Day 19: Tractor Beam](https://adventofcode.com/2019/day/19)

import atexit
import sys
import time
from collections import namedtuple
from pathlib import Path

filename = sys.argv[1] if len(sys.argv) > 1 else "input.txt"
software = Path(filename).read_text()
if "--elapsed" in sys.argv:
    sys.argv.remove("--elapsed")
    start_time_ns = time.time_ns()
    atexit.register(lambda: print(f"elapsed: {(time.time_ns() - start_time_ns) / 1_000_000}ms"))


sys.path.append(Path(__file__).parent.parent.as_posix())
from intcode.Intcode import Computer  # noqa

drone = Computer()
drone.load(software)


def scan_cell(x, y):
    drone.input.append(x)
    drone.input.append(y)
    assert drone.run() == "halted"
    return drone.output.popleft()


N = 50
last_row = (0, -1)
Row = namedtuple("Row", ("y", "x1", "x2"))


def scan_row(y, XMAX=N) -> Row:
    global last_row

    # follow the edges of the beam
    x1, x2 = last_row

    while scan_cell(x1, y) == 0 and x1 < XMAX:
        x1 += 1

    if x1 == XMAX:
        # nothing found on the line
        return

    if x1 > x2:
        x2 = x1
    while x2 < XMAX and scan_cell(x2, y) == 1:
        x2 += 1

    last_row = (x1, x2 - 1)

    return Row(y, x1, x2 - 1)


# part 1
count = 0
for y in range(N):
    row = scan_row(y, N)
    if row:
        count += row.x2 - row.x1 + 1
    if sys.stdout.isatty():
        print(f"{y:2}", "".join(".#"[scan_cell(x, y)] for x in range(N)), row)
print(count)

SQUARE = 100
grid = []

while y < 10000:
    row = scan_row(y, 10000)
    grid.append(row)
    y += 1

    if y < N + SQUARE:
        continue

    # let's consider the square OABC:
    # ................#################.......
    # .................########O--------A..... <= 100th row before
    # ..................#######|        |#....
    # ...................######|        |###..
    # ....................#####|        |#####
    # .....................####|        |#####
    # .....................####|        |#####
    # ......................###|        |#####
    # .......................##|        |#####
    # ........................#|        |#####
    # .........................C--------B##### <- current row
    # ..........................##############
    # - A should be at the very right of the 100th row before the current one
    # - B should be at the very left of the current row
    # - and, obviously, all points should be into the beam

    upper_row = grid[-SQUARE]

    yB = yC = row.y
    yO = yA = upper_row.y

    # the scan should not miss any row
    assert yB - yO + 1 == SQUARE

    xA = upper_row.x2
    xC = xO = xA - (SQUARE - 1)
    if xO < upper_row.x1:
        continue
    if xC < row.x1:
        continue
    xB = xC + (SQUARE - 1)
    if xB > row.x2:
        continue

    # print the coordinates of O
    print(xO * 10000 + yO)
    break
