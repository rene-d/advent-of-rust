#!/usr/bin/env python3
# https://adventofcode.com/2017/day/7

from pathlib import Path
from copy import deepcopy
from collections import defaultdict, deque, namedtuple
import sys, re, math, itertools, time
from functools import reduce
import re

filename = ("test.txt" if sys.argv[1] == "-t" else sys.argv[1]) if len(sys.argv) > 1 else "input.txt"
data = Path(filename).read_text().strip()
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
        weight += child_weight # compute total weight of node 'root'

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
