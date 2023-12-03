#!/usr/bin/env python3
# https://adventofcode.com/2022/day/17

# Nota: not sure to write it in Rust, nor to make it cleaner and more Pythonic...

import sys
from pathlib import Path

filename = ("test.txt" if sys.argv[1] == "-t" else sys.argv[1]) if len(sys.argv) > 1 else "input.txt"
data = Path(filename).read_text()

rocks = [
    [
        [1, 1, 1, 1],
    ],
    [
        [0, 1, 0],
        [1, 1, 1],
        [0, 1, 0],
    ],
    [
        [1, 1, 1],
        [0, 0, 1],
        [0, 0, 1],
    ],
    [
        [1],
        [1],
        [1],
        [1],
    ],
    [
        [1, 1],
        [1, 1],
    ],
]


jets = data.strip()

cave = [
    [1, 1, 1, 1, 1, 1, 1],  # the bottom
]
bottom = 0

jet_count = 0
rock_count = 0


def cave_height():
    """Returns cave height, including the bottom line."""
    global bottom
    return len(cave) + bottom


def overlap(x, y, rock):
    width = len(rock[0])
    height = len(rock)
    for i in range(height):
        for j in range(width):
            if y + i < cave_height():
                if cave[y + i - bottom][x + j] != 0 and rock[i][j] == 1:
                    return True
    return False


def fall():
    global jet_count, rock_count, bottom

    rock = rocks[rock_count % len(rocks)]
    rock_count += 1

    width = len(rock[0])
    height = len(rock)

    y = cave_height() + 3
    x = 2

    while True:
        # current jet of gas
        gas = jets[jet_count % len(jets)]
        jet_count += 1

        # shift the rock if possible
        if gas == ">" and x + width + 1 <= 7 and not overlap(x + 1, y, rock):
            x += 1
        elif gas == "<" and x > 0 and not overlap(x - 1, y, rock):
            x -= 1

        # rock falls if possible
        if not overlap(x, y - 1, rock):
            y -= 1
        else:
            break

    for i in range(height):
        if y + i >= cave_height():
            cave.append([0, 0, 0, 0, 0, 0, 0])
        for j in range(width):
            if rock[i][j] == 1:
                cave[y + i - bottom][x + j] = 65 + (rock_count - 1) % len(rocks)

    if len(cave) > 200:
        del cave[0:100]
        bottom += 100


def show(rx=None, ry=None, rock=None):
    global bottom

    h = cave_height() + 2
    if rx and ry and rock:
        width = len(rock[0])
        height = len(rock)

        h = max(h, ry + len(rock))

    for y in range(h - 1, bottom - 1, -1):
        s = "|" if y > 0 else "+"

        for x in range(0, 7):
            if rx and ry and rock:
                if rx <= x < rx + width and ry <= y < ry + height:
                    if rock[y - ry][x - rx] == 1:
                        s += "@"
                        continue

            if y < cave_height():
                c = cave[y - bottom][x]
            else:
                c = 0

            if c == 0:
                s += "."
            elif c == 1:
                s += "-"
            else:
                s += chr(c)

        s += "|" if y > 0 else "+"

        if y == cave_height() - 1:
            print(f"{s} <= top ({y})")
        else:
            print(s)

    if bottom != 0:
        print(f"({bottom} rows suppressed)")


def make_key():
    global rock_count, jet_count
    top = []
    mask = 0
    for y in range(len(cave) - 1, -1, -1):
        for i, c in enumerate(cave[y]):
            if c != 0:
                mask |= 1 << i
        top.append(mask)
        if mask == 127:
            break
    return (rock_count % len(rocks), jet_count % len(jets), bytes(top))


keys = {}
heights = [0]
start, end = None, None
part1 = None

for n in range(1, 10000):
    fall()

    if n == 2022:
        part1 = cave_height() - 1

    if not end:
        heights.append(cave_height() - 1)
        key = make_key()
        if key in keys:
            start = keys[key]
            end = n
        keys[key] = n

    if part1 and end:
        break

else:
    print("no solution")
    exit()

# part 1
print(part1)

# part 2
q, r = divmod(1_000_000_000_000 - start, end - start)
part2 = heights[start + r] + q * (heights[end] - heights[start])
print(part2)


if filename == "test.txt":
    assert part1 == 3068
    assert part2 == 1514285714288
