#!/usr/bin/env python3

from pathlib import Path
import sys

filename = "test.txt" if len(sys.argv) > 1 and sys.argv[1] == "-t" else "input.txt"
data = Path(filename).read_text()

data = data.splitlines()
ny = len(data)
nx = len(data[0])
trees = [None] * ny
for y, line in enumerate(data):
    trees[y] = [None] * nx
    for x in range(nx):
        trees[y][x] = int(line[x])

# part 1
visible = nx * 4 - 4
for y in range(1, ny - 1):
    for x in range(1, nx - 1):
        tree = trees[y][x]
        if all(tree > trees[i][x] for i in range(y + 1, ny)):
            visible += 1
            continue
        if all(tree > trees[i][x] for i in range(0, y)):
            visible += 1
            continue
        if all(tree > trees[y][i] for i in range(x + 1, nx)):
            visible += 1
            continue
        if all(tree > trees[y][i] for i in range(0, x)):
            visible += 1
            continue
print(visible)

# part 2
max_scene = 0
for y in range(1, ny - 1):
    for x in range(1, nx - 1):
        tree = trees[y][x]
        scene = 1

        visi = 0
        i = x + 1
        while i < nx:
            visi += 1
            if tree <= trees[y][i]:
                break
            i += 1
        scene *= visi

        visi = 0
        i = x - 1
        while i >= 0:
            visi += 1
            if tree <= trees[y][i]:
                break
            i -= 1
        scene *= visi

        visi = 0
        i = y + 1
        while i < ny:
            visi += 1
            if tree <= trees[i][x]:
                break
            i += 1
        scene *= visi

        visi = 0
        i = y - 1
        while i >= 0:
            visi += 1
            if tree <= trees[i][x]:
                break
            i -= 1
        scene *= visi

        if scene > max_scene:
            max_scene = scene

print(max_scene)
