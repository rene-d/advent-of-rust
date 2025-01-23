#!/usr/bin/env python3
# [Day 4: Repose Record](https://adventofcode.com/2018/day/4)

import re
import sys
from collections import defaultdict
from pathlib import Path

verbose = "-v" in sys.argv
if verbose:
    sys.argv.remove("-v")
self_tests = "-T" in sys.argv
if self_tests:
    sys.argv.remove("-T")
filename = ("test.txt" if sys.argv[1] == "-t" else sys.argv[1]) if len(sys.argv) > 1 else "input.txt"
data = Path(filename).read_text().strip()
lines = data.splitlines()

sleeping = defaultdict(lambda: [0] * 60)
asleep = None
guard = None
for line in sorted(lines):
    _, _, _, _, minute, event = re.match(r"^\[(\d+)-(\d+)-(\d+) (\d+):(\d+)\] (.+)$", line).groups()
    minute = int(minute)
    if event.startswith("Guard #"):
        guard = int(event.removeprefix("Guard #").removesuffix(" begins shift"))
    else:
        if event == "falls asleep":
            asleep = minute
        elif event == "wakes up":
            for i in range(asleep, minute):
                sleeping[guard][i] += 1

# part 1
guard = max((sum(v), guard) for guard, v in sleeping.items())[1]
n = max((m, n) for (n, m) in enumerate(sleeping[guard]))[1]
part1 = guard * n
print(part1)

# part 2
_, guard, minute = max(max((v[m], guard, m) for guard, v in sleeping.items()) for m in range(60))
part2 = guard * minute
print(part2)


if filename == "test.txt":
    assert part1 == 240
    assert part2 == 4455
