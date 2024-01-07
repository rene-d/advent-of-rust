#!/usr/bin/env python3
# https://adventofcode.com/2019/day/18

import itertools
import sys
from collections import deque
from pathlib import Path

verbose = "-v" in sys.argv
if verbose:
    sys.argv.remove("-v")
self_tests = "-T" in sys.argv
if self_tests:
    sys.argv.remove("-T")
filename = ("test.txt" if sys.argv[1] == "-t" else sys.argv[1]) if len(sys.argv) > 1 else "input.txt"
data = Path(filename).read_text().strip()


maze = [list(row) for row in data.splitlines()]
sx = len(maze[0])
sy = len(maze)


def part1():
    entrance = None
    doors = {}
    keys = {}

    for x, y in itertools.product(range(sx), range(sy)):
        c = maze[y][x]
        if c == "@":
            entrance = x, y
        elif c.isupper():
            doors[c] = (x, y)
        elif c.islower():
            keys[c] = (x, y)
        else:
            assert c == "#" or c == "."

    keys_count = len(keys)

    def bfs(start):
        q = deque()
        q.append((start, set(), 0))

        seen = set()
        while q:
            (x, y), keys, steps = q.popleft()

            state = (x, y, tuple(sorted(keys)))
            if state in seen:
                continue
            seen.add(state)

            c = maze[y][x]

            # a door to which we don't have the key
            if c.isupper() and c.lower() not in keys:
                continue

            # a key
            if c.islower():
                # collect the key
                keys.add(c)
                if len(keys) == keys_count:
                    return steps

            # move one step
            for dx, dy in ((0, 1), (1, 0), (0, -1), (-1, 0)):
                nx, ny = x + dx, y + dy
                # within the limits of maze and not through the walls
                if 0 <= nx < sx and 0 <= ny < sy and maze[ny][nx] != "#":
                    q.append(((nx, ny), set(keys), steps + 1))

        return 0

    print(bfs(entrance))


def part2():
    for x, y in itertools.product(range(sx), range(sy)):
        c = maze[y][x]
        if c == "@":
            ex, ey = x, y
            for i in range(-1, 2):
                maze[y + i][x] = "#"
                maze[y][x + i] = "#"
            break

    part2 = 0

    for dx, dy in ((-1, -1), (-1, 1), (1, -1), (1, 1)):
        max_dist = 0
        prev_dist = 0
        last_key_dist = 0
        visited = 0

        stack = [((ex + dx, ey + dy), 0, 1)]
        while stack:
            (x, y), d, dist = stack.pop()

            if dist - 1 != prev_dist:
                visited -= prev_dist - max(last_key_dist, dist - 1)
                last_key_dist = min(last_key_dist, dist - 1)

            prev_dist = dist
            visited += 1

            if maze[y][x].islower():
                max_dist = max(max_dist, dist)
                last_key_dist = dist

            r = range(-1, 2) if dist > 1 else range(4)
            for i in r:
                nd = (d + i) % 4
                match nd:
                    case 0:
                        nx, ny = x, y - 1
                    case 1:
                        nx, ny = x + 1, y
                    case 2:
                        nx, ny = x, y + 1
                    case 3:
                        nx, ny = x - 1, y

                if 0 <= nx < sx and 0 <= ny < sy and maze[ny][nx] != "#":
                    stack.append(((nx, ny), nd, dist + 1))

        visited -= prev_dist - last_key_dist
        part2 += visited * 2 - max_dist - 1

    print(part2)


part1()

print("???")  # part 2 is buggy
