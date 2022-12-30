#!/usr/bin/env python3
# https://adventofcode.com/2019/day/15

# Nota: this algorithm does not find the shortest path to the oxygen system
# kept for reference

from pathlib import Path
import sys


sys.path.append("..")
from intcode.Intcode import Computer


filename = sys.argv[1] if len(sys.argv) > 1 else "input.txt"
software = Path(filename).read_text()


droid = Computer()
droid.load(software)
droid.start(output_mode="yield")

NORTH = 1
SOUTH = 2
WEST = 3
EAST = 4

WALL = 0
EMPTY = 1
OXYGEN = 2

positions = {}

moves = [
    None,
    (0, 1),  # north
    (0, -1),  # south
    (-1, 0),  # west
    (1, 0),  # east
]

x, y = 0, 0
positions[(x, y)] = 4
direction = EAST

follow_hand = (NORTH, EAST, SOUTH, WEST)
hand = 0
count = 0
found = False
for _ in range(1000):

    for _ in range(len(follow_hand)):
        direction = follow_hand[hand]

        dx, dy = moves[direction]
        mx, my = x + dx, y + dy

        droid.input.append(direction)
        state = droid.resume()
        assert state == "yield"
        status = droid.output.popleft()

        assert 0 <= status <= 2

        positions[(mx, my)] = status

        if status == OXYGEN:
            print("found", count)
            found = True
            break

        if status == EMPTY:
            print(f"droid is at {x},{y} . droid move to {mx},{my}")
            x, y = mx, my
            moved = True
            count += 1
            hand = (hand - 1) % len(follow_hand)

            break

        print(f"droid is at {x},{y} . cannot move {direction}: wall at {mx},{my}")
        hand = (hand + 1) % len(follow_hand)
    else:
        print("bloqué")
        break
    if found:
        break


def show():
    inf = int(1e6)
    minx, maxx, miny, maxy = inf, -inf, inf, -inf
    for (x, y), color in positions.items():
        minx = min(minx, x)
        maxx = max(maxx, x)
        miny = min(miny, y)
        maxy = max(maxy, y)

    lines = []
    for y in range(maxy, miny - 1, -1):
        row = ""
        for x in range(minx, maxx + 1):
            status = positions.get((x, y), 3)
            row += "# O?S"[status]
        lines.append(row)
    return "\n".join(lines)


print(show())
print("found", count)
