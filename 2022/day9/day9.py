#!/usr/bin/env python3

from pathlib import Path
import sys

filename = "test.txt" if len(sys.argv) > 1 and sys.argv[1] == "-t" else "input.txt"
data = Path(filename).read_text()


nx, ny = 6, 5

H = (0, 0)
T = (0, 0)

grid = [None] * ny
for y in range(ny):
    grid[y] = ["."] * nx

grid[0][0] = "s"


def show():
    for y in range(ny - 1, -1, -1):
        row = ""
        for x in range(nx):
            if H == (x, y):
                row += "H"
            elif T == (x, y):
                row += "T"
            else:
                row += grid[y][x]
        print(row)
    print()


def move(dir, n):
    global H, T
    hx, hy = H
    tx, ty = T

    for _ in range(n):

        mx, my = 0, 0

        if dir == "R":
            mx, my = 1, 0
        elif dir == "L":
            mx, my = -1, 0
        elif dir == "U":
            mx, my = 0, 1
        elif dir == "D":
            mx, my = 0, -1

        hx += mx
        hy += my

        if tx == hx:
            if abs(ty - hy) > 1:
                ty += my
        elif ty == hy:
            if abs(tx - hx) > 1:
                tx += mx
        elif abs(tx - hx) + abs(ty - hy) > 2:

            mx = 1 if hx - tx > 0 else -1
            my = 1 if hy - ty > 0 else -1
            tx += mx
            ty += my

    H = (hx, hy)
    T = (tx, ty)


tails = set()
tails.add(T)
for line in data.splitlines():
    dir, n = line.split()
    for i in range(int(n)):
        move(dir, 1)
        tails.add(T)

print(len(tails))
