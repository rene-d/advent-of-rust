#!/usr/bin/env python3
# [Day 12: Digital Plumber](https://adventofcode.com/2017/day/12)

import atexit
import sys
import time
from collections import deque
from pathlib import Path

filename = ("test.txt" if sys.argv[1] == "-t" else sys.argv[1]) if len(sys.argv) > 1 else "input.txt"
data = Path(filename).read_text().strip()
if "--elapsed" in sys.argv:
    sys.argv.remove("--elapsed")
    start_time_ns = time.time_ns()
    atexit.register(lambda: print(f"elapsed: {(time.time_ns() - start_time_ns) / 1_000_000}ms"))


lines = data.splitlines()


links = dict()
for line in lines:
    src, dests = line.split(" <-> ", maxsplit=1)
    src = int(src)
    dests = list(map(int, dests.split(",")))
    links[src] = dests


def walk(id):
    seen = set()
    q = deque()
    q.append(id)

    while q:
        id = q.pop()
        seen.add(id)
        for i in links[id]:
            if i not in seen:
                q.append(i)

    return seen


# part 1
print(len(walk(0)))

# part 2
programs = set(links.keys())
groups = 0
while programs:
    groups += 1
    id = programs.pop()
    connected = walk(id)
    programs = programs.difference(connected)
print(groups)
