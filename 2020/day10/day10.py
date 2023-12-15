#!/usr/bin/env python3
# https://adventofcode.com/2020/day/10

import sys
from collections import defaultdict
from pathlib import Path

verbose = "-v" in sys.argv
if verbose:
    sys.argv.remove("-v")
filename = ("test.txt" if sys.argv[1] == "-t" else sys.argv[1]) if len(sys.argv) > 1 else "input.txt"
data = Path(filename).read_text().strip()


# part 1

adapters = sorted(map(int, data.splitlines()))

diffs = defaultdict(lambda: 0)
for a, b in zip(adapters, adapters[1:]):
    diffs[b - a] += 1

assert len(diffs) == 2
assert (1 in diffs) and (3 in diffs)

diffs[adapters[0] - 0] += 1  # charging outlet has an effective rating of 0 jolts
diffs[3] += 1  # device's built-in adapter is always 3 higher

print(diffs[1] * diffs[3])


# part 2

adapters.insert(0, 0)  # add the charging outlet

n1 = 0
n2 = 0
n3 = 1

for a, b in zip(adapters, adapters[1:]):
    match b - a:
        case 1:
            n1, n2, n3 = n2, n3, n1 + n2 + n3
        case 2:
            n1, n2, n3 = n3, 0, n2 + n3
        case 3:
            n1, n2, n3 = 0, 0, n3

print(n3)
