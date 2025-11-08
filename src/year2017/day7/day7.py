#!/usr/bin/env python3
# [Day 7: Recursive Circus](https://adventofcode.com/2017/day/7)

import atexit
import re
import sys
import time
from collections import defaultdict
from pathlib import Path

filename = ("test.txt" if sys.argv[1] == "-t" else sys.argv[1]) if len(sys.argv) > 1 else "input.txt"
data = Path(filename).read_text().strip()
if "--elapsed" in sys.argv:
    sys.argv.remove("--elapsed")
    start_time_ns = time.time_ns()
    atexit.register(lambda: print(f"elapsed: {(time.time_ns() - start_time_ns) / 1_000_000}ms"))


lines = data.splitlines()

all_children = set()
children = dict()
nodes = dict()
for line in lines:
    m = re.match(r"^(\w+) \((\d+)\)(?: \-> (.+))?$", line)
    node = m.group(1)
    nodes[node] = int(m.group(2))
    if m.group(3):
        all_children.update(m.group(3).split(", "))
        children[node] = m.group(3).split(", ")
root = set(nodes.keys()).difference(all_children).pop()

print(root)


def traverse(root):
    weight = nodes[root]

    z = defaultdict(list)
    for node in children.get(root, ()):
        child_weight = traverse(node)
        if not child_weight:
            # solution found
            return
        z[child_weight].append(node)
        weight += child_weight  # compute total weight of node 'root'

    if len(set(z.keys())) >= 2:
        for cost, n in z.items():
            if len(n) == 1:
                bad_weight = nodes[n.pop()]
                bad = cost
            else:
                good = cost
        print(bad_weight - bad + good)
        return

    return weight


traverse(root)
