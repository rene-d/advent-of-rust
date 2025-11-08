#!/usr/bin/env python3

import atexit
import itertools
import re
import sys
import time
from pathlib import Path

filename = ("test.txt" if sys.argv[1] == "-t" else sys.argv[1]) if len(sys.argv) > 1 else "input.txt"
data = Path(filename).read_text()
if "--elapsed" in sys.argv:
    sys.argv.remove("--elapsed")
    start_time_ns = time.time_ns()
    atexit.register(lambda: print(f"elapsed: {(time.time_ns() - start_time_ns) / 1_000_000}ms"))


happiness = {}
names = set()

for line in data.splitlines():
    m = re.match(r"(.+) would (gain|lose) (\d+) happiness units by sitting next to (.+)\.$", line)
    if not m:
        exit()

    name1 = m.group(1)
    name2 = m.group(4)
    delta = int(m.group(3))
    if m.group(2) == "lose":
        delta = -delta

    happiness[(name1, name2)] = delta

    names.add(name1)
    names.add(name2)


def solve():
    happiness_max = 0
    for x in itertools.permutations(names):
        happiness_sum = 0
        for i in range(len(x)):
            happiness_sum += happiness[(x[i], x[(i + 1) % len(x)])]
            happiness_sum += happiness[(x[(i + 1) % len(x)], x[i])]

        if happiness_sum > happiness_max:
            happiness_max = happiness_sum
    print(happiness_max)


solve()

for name in names:
    happiness[(name, "me")] = 0
    happiness[("me", name)] = 0
names.add("me")
solve()
