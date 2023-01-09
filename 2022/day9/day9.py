#!/usr/bin/env python3

from pathlib import Path
import sys

filename = ("test.txt" if sys.argv[1] == "-t" else sys.argv[1]) if len(sys.argv) > 1 else "input.txt"
data = Path(filename).read_text()


def show(rope):

    minx = min(k[0] for k in rope)
    maxx = max(k[0] for k in rope)
    miny = min(k[1] for k in rope)
    maxy = max(k[1] for k in rope)

    minx = min(minx, 0)
    miny = min(miny, 0)
    maxx = max(maxx, 5)
    maxy = max(maxy, 4)

    for y in range(maxy, miny - 1, -1):
        row = ""
        for x in range(minx, maxx + 1):
            for i, knot in enumerate(rope):
                if knot == (x, y):
                    label = "H" if i == 0 else "T" if i == len(rope) - 1 else str(i)
                    row += label
                    break
            else:
                if (x, y) == (0, 0):
                    row += "s"
                else:
                    row += "."
        print(row)
    print()


def move(dir, head, tail):
    hx, hy = head
    tx, ty = tail

    # head movement
    if dir == "R":
        dx, dy = 1, 0
    elif dir == "L":
        dx, dy = -1, 0
    elif dir == "U":
        dx, dy = 0, 1
    elif dir == "D":
        dx, dy = 0, -1
    else:
        dx, dy = 0, 0

    hx += dx
    hy += dy

    # tail movement
    dx = 1 if hx - tx > 0 else -1
    dy = 1 if hy - ty > 0 else -1

    if tx == hx:
        if abs(ty - hy) > 1:
            ty += dy
    elif ty == hy:
        if abs(tx - hx) > 1:
            tx += dx
    elif abs(tx - hx) + abs(ty - hy) > 2:
        tx += dx
        ty += dy

    head = (hx, hy)
    tail = (tx, ty)

    return head, tail


def move_rope(dir, rope):
    for k in range(len(rope) - 1):
        rope[k], rope[k + 1] = move(dir if k == 0 else "", rope[k], rope[k + 1])


# part 1
H = (0, 0)
T = (0, 0)
tails = set()
tails.add(T)
for line in data.splitlines():
    dir, n = line.split()
    for i in range(int(n)):
        H, T = move(dir, H, T)
        tails.add(T)
print(len(tails))
# show([H, T])


# part 2
rope = [(0, 0)] * 10
tails = set()
tails.add(rope[-1])
for line in data.splitlines():
    dir, n = line.split()
    for _ in range(int(n)):
        move_rope(dir, rope)
        tails.add(rope[-1])
print(len(tails))
# show(rope)
