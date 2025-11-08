#!/usr/bin/env python3
# [Day 6: Wait For It](https://adventofcode.com/2023/day/6)

import atexit
import math
import sys
import time
from pathlib import Path

filename = ("test.txt" if sys.argv[1] == "-t" else sys.argv[1]) if len(sys.argv) > 1 else "input.txt"
data = Path(filename).read_text().strip()
if "--elapsed" in sys.argv:
    sys.argv.remove("--elapsed")
    start_time_ns = time.time_ns()
    atexit.register(lambda: print(f"elapsed: {(time.time_ns() - start_time_ns) / 1_000_000}ms"))


def wins(times, distances):
    result = 1

    for t, d in zip(times, distances):
        # win = 0
        # for hold in range(1, t):
        #     if hold * (t - hold) > d:
        #         win += 1

        # hold * (t - hold) > d ⇔ a < hold < b
        a = (t - (t * t - 4 * d) ** 0.5) / 2
        b = (t + (t * t - 4 * d) ** 0.5) / 2

        # 1 ≤ hold < t
        a = max(1, math.floor(a))
        b = min(t, math.ceil(b))

        win = b - a - 1

        result *= win

    return result


# parse input
lines = data.splitlines()

times = list(map(int, lines[0].removeprefix("Time:").split()))
distances = list(map(int, lines[1].removeprefix("Distance:").split()))

# part 1
print(wins(times, distances))

# part 2
times = (int(lines[0].removeprefix("Time:").replace(" ", "")),)
distances = (int(lines[1].removeprefix("Distance:").replace(" ", "")),)
print(wins(times, distances))
