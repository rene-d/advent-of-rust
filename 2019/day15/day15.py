#!/usr/bin/env python3
# https://adventofcode.com/2019/day/15

import sys
from collections import deque
from pathlib import Path

sys.path.append(Path(__file__).parent.parent.as_posix())
from intcode.Intcode import Computer  # noqa

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

MOVES = [
    None,
    (0, 1),  # north
    (0, -1),  # south
    (-1, 0),  # west
    (1, 0),  # east
]


def bfs(droid, show):
    positions = {}
    x, y = 0, 0
    positions[(x, y)] = 4  # for S
    direction = EAST

    q = deque()
    q.append((0, 0, 0))
    seen = set()

    droids = {}
    droids[(0, 0)] = droid.clone()

    oxygen = None
    max_steps = 0

    # need to clone the droid state to avoid testing all directions and walk back to them
    # when there is a T or X junction
    while q:
        x, y, steps = q.popleft()

        max_steps = max(max_steps, steps)

        droid = droids[(x, y)]

        for direction in (NORTH, SOUTH, WEST, EAST):
            dx, dy = MOVES[direction]
            mx, my = x + dx, y + dy

            if (mx, my) in seen:
                continue
            seen.add((x, y))

            new_droid = droid.clone()
            new_droid.input.append(direction)
            state = new_droid.resume()
            assert state == "yield"
            status = new_droid.output.popleft()

            assert 0 <= status <= 2

            positions[(mx, my)] = status

            if status == OXYGEN:
                oxygen = (new_droid, steps + 1)
                q.clear()  # stop the bfs
                break

            if status != WALL:
                droids[(mx, my)] = new_droid
                q.append((mx, my, steps + 1))

    droids.clear()
    seen.clear()

    if show:
        inf = int(1e6)
        minx, maxx, miny, maxy = inf, -inf, inf, -inf
        for (x, y), _ in positions.items():
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
        print("\n".join(lines))

    return oxygen, max_steps


(oxygen, steps), _ = bfs(droid, False)
print(steps)

_, distance = bfs(oxygen, False)
print(distance)
