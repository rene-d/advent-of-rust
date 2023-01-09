#!/usr/bin/env python3

import re
from copy import deepcopy
import sys

filename = ("test.txt" if sys.argv[1] == "-t" else sys.argv[1]) if len(sys.argv) > 1 else "input.txt"

moves = []
stacks = []

state = "stacks"
for line in open(filename):
    line = line.rstrip("\n")

    if line == "":
        state = "moves"

    elif state == "stacks":
        p = 0
        crates = []
        p = 0
        while 1 + p * 4 < len(line):
            crate = line[1 + p * 4]

            if "A" <= crate <= "Z":
                while len(stacks) < p + 1:
                    stacks.append([])
                stacks[p].append(crate)

            p += 1

    else:
        m = re.match(r"move (\d+) from (\d+) to (\d+)", line)
        moves.append(list(map(int, m.groups())))


part1 = deepcopy(stacks)
for (count, source, dest) in moves:
    for i in range(count):
        crate = part1[source - 1].pop(0)
        part1[dest - 1].insert(0, crate)
print("".join(stack[0] for stack in part1))


part2 = deepcopy(stacks)
for (count, source, dest) in moves:
    for i in range(count):
        crate = part2[source - 1].pop(count - i - 1)
        part2[dest - 1].insert(0, crate)
print("".join(stack[0] for stack in part2))
