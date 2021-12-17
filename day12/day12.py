#!/usr/bin/env python3

# Day 12: Passage Pathing
# https://adventofcode.com/2021/day/12

import sys
from collections import defaultdict

data = open("input.txt" if len(sys.argv) == 1 else sys.argv[1]).read().splitlines()

nodes = defaultdict(list)
for line in data:
    a, b = line.split("-")
    nodes[a].append(b)
    nodes[b].append(a)


def paths(visit_small_twice):
    count = 0
    heap = [("start", set(["start"]), None)]
    while heap:
        node, once, twice = heap.pop()

        if node == "end":
            count += 1
            continue

        for dest in nodes[node]:

            if dest not in once:
                if dest.lower() == dest:
                    new_once = once.copy()
                    new_once.add(dest)
                    heap.append((dest, new_once, twice))
                else:
                    heap.append((dest, once, twice))

            elif visit_small_twice and twice is None and dest in once and dest != "end" and dest != "start":
                heap.append((dest, once, dest))

    return count


print(paths(False))
print(paths(True))
