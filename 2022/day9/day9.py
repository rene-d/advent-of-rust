#!/usr/bin/env python3

from pathlib import Path
import sys

filename = "test.txt" if len(sys.argv) > 1 and sys.argv[1] == "-t" else "input.txt"
data = Path(filename).read_text()


def show(rope):

    minx = min(k[0][0] for k in rope)
    maxx = max(k[0][0] for k in rope)
    miny = min(k[0][1] for k in rope)
    maxy = max(k[0][1] for k in rope)

    maxx = max(maxx, 5)
    maxy = max(maxy, 4)

    for y in range(maxy, miny - 1, -1):
        row = ""
        for x in range(minx, maxx + 1):
            for knot in rope:
                if knot[0] == (x, y):
                    row += knot[1]
                    break
            else:
                if (x, y) == (0, 0):
                    row += "s"
                else:
                    row += "."
        print(row)
    print()


def move(dir, H, T):
    hx, hy = H
    tx, ty = T

    if dir == "R":
        mx, my = 1, 0
    elif dir == "L":
        mx, my = -1, 0
    elif dir == "U":
        mx, my = 0, 1
    elif dir == "D":
        mx, my = 0, -1
    else:
        mx, my = 0, 0

    hx += mx
    hy += my

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

    H = (hx, hy)
    T = (tx, ty)

    return H, T


# part 1
start = (0, 0)
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

# rope=[(H, "H"), (T, "T")]
# show(rope)

# part 2
tails = set()
rope = [[(0, 0), "H"]]
for k in range(1, 10):
    rope.append([(0, 0), str(k)])


def move_rope(dir, n):
    for i in range(n):
        for k in range(len(rope) - 1):
            h, _ = rope[k]
            t, _ = rope[k + 1]
            h, t = move(dir if k == 0 else "", h, t)
            rope[k], rope[k + 1] = [h, rope[k][1]], [t, rope[k + 1][1]]
        tails.add(rope[-1][0])


tails = set()
for line in data.splitlines():
    dir, n = line.split()
    move_rope(dir, int(n))
print(len(tails))
