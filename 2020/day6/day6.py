#!/usr/bin/env python3
# https://adventofcode.com/2020/day/6

import sys
from pathlib import Path

verbose = "-v" in sys.argv
if verbose:
    sys.argv.remove("-v")
filename = ("test.txt" if sys.argv[1] == "-t" else sys.argv[1]) if len(sys.argv) > 1 else "input.txt"
data = Path(filename).read_text().strip()
lines = data.splitlines()

result = 0
for group in data.split("\n\n"):
    group = group.replace("\n", "")
    result += len(set(group))
print(result)


result = 0
for group in data.split("\n\n"):
    yes = None
    for person in group.splitlines():
        if yes is None:
            yes = set(person)
        else:
            yes.intersection_update(set(person))
    result += len(set(yes))
print(result)
