#!/usr/bin/env python3

import atexit
import re
import sys
import time
from pathlib import Path

filename = ("test.txt" if sys.argv[1] == "-t" else sys.argv[1]) if len(sys.argv) > 1 else "input.txt"
if "--elapsed" in sys.argv:
    sys.argv.remove("--elapsed")
    start_time_ns = time.time_ns()
    atexit.register(lambda: print(f"elapsed: {(time.time_ns() - start_time_ns) / 1_000_000}ms"))
data = Path(filename).read_text()


aunts = {}

for line in data.splitlines():
    m = re.match(r"Sue (\d+): (\w+): (\d+), (\w+): (\d+), (\w+): (\d+)", line)
    aunts[m.group(1)] = {m.group(2): int(m.group(3)), m.group(4): int(m.group(5)), m.group(6): int(m.group(7))}

for sue, aunt in aunts.items():
    if aunt.get("children", 3) != 3:
        continue
    if aunt.get("cats", 7) != 7:  # should be greater than
        continue
    if aunt.get("samoyeds", 2) != 2:
        continue
    if aunt.get("pomeranians", 3) != 3:  # should be fewer than
        continue
    if aunt.get("akitas", 0) != 0:
        continue
    if aunt.get("vizslas", 0) != 0:
        continue
    if aunt.get("goldfish", 5) != 5:  # should be fewer than
        continue
    if aunt.get("trees", 3) != 3:  # should be greater than
        continue
    if aunt.get("cars", 2) != 2:
        continue
    if aunt.get("perfumes", 1) != 1:
        continue

    print(sue)


for sue, aunt in aunts.items():
    if aunt.get("children", 3) != 3:
        continue
    if aunt.get("cats", 7 + 1) <= 7:  # should be greater than
        continue
    if aunt.get("samoyeds", 2) != 2:
        continue
    if aunt.get("pomeranians", 3 - 1) >= 3:  # should be fewer than
        continue
    if aunt.get("akitas", 0) != 0:
        continue
    if aunt.get("vizslas", 0) != 0:
        continue
    if aunt.get("goldfish", 5 - 1) >= 5:  # should be fewer than
        continue
    if aunt.get("trees", 3 + 1) <= 3:  # should be greater than
        continue
    if aunt.get("cars", 2) != 2:
        continue
    if aunt.get("perfumes", 1) != 1:
        continue

    print(sue)
