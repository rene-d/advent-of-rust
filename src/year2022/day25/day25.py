#!/usr/bin/env python3
# [Day 25: Full of Hot Air](https://adventofcode.com/2022/day/25)

import atexit
import sys
import time
from pathlib import Path

filename = ("test.txt" if sys.argv[1] == "-t" else sys.argv[1]) if len(sys.argv) > 1 else "input.txt"
data = Path(filename).read_text()
if "--elapsed" in sys.argv:
    sys.argv.remove("--elapsed")
    start_time_ns = time.time_ns()
    atexit.register(lambda: print(f"elapsed: {(time.time_ns() - start_time_ns) / 1_000_000}ms"))


lines = data.splitlines()


def from_snafu(s):
    digits = {"2": 2, "1": 1, "0": 0, "-": -1, "=": -2}
    n = 0
    for i, c in enumerate(reversed(s)):
        d = digits[c]
        n = 5**i * d + n
    return n


def to_snafu(n):
    digits = "=-012"
    s = []
    while True:
        c = digits[(n + 2) % 5]
        s.append(c)
        n = (n + 2) // 5
        if n == 0:
            break
    return "".join(reversed(s))


assert from_snafu("2=-01") == 976
assert to_snafu(976) == "2=-01"

print(to_snafu(sum(map(from_snafu, lines))))
