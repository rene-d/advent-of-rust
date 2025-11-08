#!/usr/bin/env python3
# [Day 25: Snowverload](https://adventofcode.com/2023/day/25)

import atexit
import sys
import time
from pathlib import Path

import networkx as nx

verbose = "-v" in sys.argv
if verbose:
    sys.argv.remove("-v")
filename = ("test.txt" if sys.argv[1] == "-t" else sys.argv[1]) if len(sys.argv) > 1 else "input.txt"
data = Path(filename).read_text().strip()
if "--elapsed" in sys.argv:
    sys.argv.remove("--elapsed")
    start_time_ns = time.time_ns()
    atexit.register(lambda: print(f"elapsed: {(time.time_ns() - start_time_ns) / 1_000_000}ms"))


# parse input
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
