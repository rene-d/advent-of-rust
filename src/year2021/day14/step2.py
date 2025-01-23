#!/usr/bin/env python3

from pathlib import Path

data = Path("input.txt").read_text().splitlines()

polymer = "y" + data[0] + "x"

transforms = {}
for line in data[2:]:
    transforms[line[0:2]] = line[-1]

for c in set("".join(transforms.keys())):
    transforms[c + "x"] = "x"
    transforms["y" + c] = "y"

transforms["xx"] = "x"
transforms["yy"] = "y"

pairs = list(transforms.keys())
N = len(pairs)

grows = []
for i, pair in enumerate(pairs):
    x = transforms[pair]
    g = (pair, x, pairs.index(pair[0] + x), pairs.index(x + pair[1]))
    grows.append(g)

nb = [0] * N

for i in range(len(polymer) - 1):
    pair = polymer[i : i + 2]
    j = pairs.index(pair)
    nb[j] += 1


for _ in range(40):
    nb2 = [0] * N
    for i in range(N):
        _, _, a, b = grows[i]
        nb2[a] += nb[i]
        nb2[b] += nb[i]
    nb = nb2


letters = [0] * 26
for i in range(N):
    c = pairs[i][0]
    if c not in "xy":
        letters[ord(c) - ord("A")] += nb[i]
    c = pairs[i][1]
    if c not in "xy":
        letters[ord(c) - ord("A")] += nb[i]

M = max(letters)
m = min(n for n in letters if n != 0)
print((M - m) // 2)
