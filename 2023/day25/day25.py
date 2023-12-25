#!/usr/bin/env python3
# https://adventofcode.com/2023/day/25

import sys
from pathlib import Path

import networkx as nx

verbose = "-v" in sys.argv
if verbose:
    sys.argv.remove("-v")
filename = ("test.txt" if sys.argv[1] == "-t" else sys.argv[1]) if len(sys.argv) > 1 else "input.txt"
data = Path(filename).read_text().strip()
lines = data.splitlines()

graph = nx.Graph()
for line in lines:
    c, e = line.split(": ")
    for v in e.split():
        graph.add_edge(c, v, capacity=1)

for a in graph.nodes():
    for b in graph.nodes():
        if a != b:
            cut, (left, right) = nx.minimum_cut(graph, a, b)
            if cut == 3:
                print(len(left) * len(right))
                exit()
