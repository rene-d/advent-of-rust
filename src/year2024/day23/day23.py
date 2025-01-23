#!/usr/bin/env python3
# [Day 23: LAN Party](https://adventofcode.com/2024/day/23)

from argparse import ArgumentParser
from pathlib import Path

import networkx as nx

parser = ArgumentParser()
parser.add_argument("-v", "--verbose", action="store_true")
parser.add_argument("-t", "--test", action="store_true")
parser.add_argument("--elapsed", action="store_true")
parser.add_argument("filename", nargs="?", type=Path, default="input.txt")
args = parser.parse_args()
if args.test:
    args.filename = Path("test.txt")

data = args.filename.read_text().strip()
lines = data.splitlines()

# init
connections = []
for line in lines:
    connections.append(tuple(line.strip().split("-")))

G = nx.Graph()
G.add_edges_from(connections)

# part 1
triangles = [list(triangle) for triangle in nx.enumerate_all_cliques(G) if len(triangle) == 3]
print(sum(1 for triangle in triangles if any(node.startswith("t") for node in triangle)))

# part 2
largest_clique = max(nx.find_cliques(G), key=len)
password = ",".join(sorted(largest_clique))
print(password)
