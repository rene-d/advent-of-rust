#!/usr/bin/env python3
# [Day 6: Memory Reallocation](https://adventofcode.com/2017/day/6)

import atexit
import sys
import time
from pathlib import Path

filename = ("test.txt" if sys.argv[1] == "-t" else sys.argv[1]) if len(sys.argv) > 1 else "input.txt"
data = Path(filename).read_text().strip()
if "--elapsed" in sys.argv:
    sys.argv.remove("--elapsed")
    start_time_ns = time.time_ns()
    atexit.register(lambda: print(f"elapsed: {(time.time_ns() - start_time_ns) / 1_000_000}ms"))


banks = list(map(int, data.split()))
size = len(banks)

seen = set()
iterations = 0

part1 = 0
part2 = 0
loop = None

while True:
    state = ",".join(map(str, banks))

    if state in seen:
        # we have detected a loop
        if part1 == 0:
            part1 = iterations
            loop = state

        # count iterations within the first loop
        elif loop == state:
            part2 = iterations - part1
            break

    seen.add(state)

    # find the max
    blocks_max = -1
    index_max = -1
    for i, blocks in enumerate(banks):
        if blocks_max < blocks:
            index_max = i
            blocks_max = blocks

    assert blocks_max >= 1

    # redistribute blocks
    banks[index_max] = 0
    realloc = max(1, blocks_max // size)
    while blocks_max > 0:
        index_max += 1
        banks[index_max % size] += realloc
        blocks_max -= realloc

    iterations += 1


print(part1)
print(part2)
