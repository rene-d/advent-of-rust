#!/usr/bin/env python3
# https://adventofcode.com/2019/day/19

from pathlib import Path
import sys
from collections import namedtuple

sys.path.append(Path(__file__).parent.parent.as_posix())
from intcode.Intcode import Computer


filename = sys.argv[1] if len(sys.argv) > 1 else "input.txt"
software = Path(filename).read_text()

drone = Computer()
drone.load(software)


def scan_cell(x, y):
    drone.input.append(x)
    drone.input.append(y)
    assert drone.run() == "halted"
    return drone.output.popleft()


N = 50
scan_range = (0, N)
Row = namedtuple("Row", ("y", "x1", "x2"))


def scan_row(y, XMAX=N) -> Row:
    global scan_range

    # follow the edges of the beam
    # the next row starts and finishs on the same columns of the previous one
    x1, x2 = XMAX, 0
    x = scan_range[0]
    while x < scan_range[1]:
        if scan_cell(x, y) == 1:
            x1 = min(x1, x)
            if 4 < scan_range[1] < XMAX and x < scan_range[1] - 2:
                x = scan_range[1] - 2
            x2 = max(x2, x)
        x += 1

    if x1 > x2:
        # nothing found on the line
        scan_range = (scan_range[0], scan_range[1] + 1)
    else:
        scan_range = (x1, x2 + 2)
        return Row(y, x1, x2)


# part 1
count = 0
for y in range(N):
    row = scan_row(y, N)
    if row:
        count += row.x2 - row.x1 + 1
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
