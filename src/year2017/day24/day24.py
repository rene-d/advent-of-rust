#!/usr/bin/env python3
# [Day 24: Electromagnetic Moat](https://adventofcode.com/2017/day/24)

import atexit
import sys
import time
from collections import deque
from pathlib import Path

verbose = "-v" in sys.argv
if verbose:
    sys.argv.remove("-v")
self_tests = "-T" in sys.argv
if self_tests:
    sys.argv.remove("-T")
filename = ("test.txt" if sys.argv[1] == "-t" else sys.argv[1]) if len(sys.argv) > 1 else "input.txt"
data = Path(filename).read_text().strip()
if "--elapsed" in sys.argv:
    sys.argv.remove("--elapsed")
    start_time_ns = time.time_ns()
    atexit.register(lambda: print(f"elapsed: {(time.time_ns() - start_time_ns) / 1_000_000}ms"))


ports = [tuple(map(int, line.split("/"))) for line in data.splitlines()]


max_strength = 0
max_length = 0
max_length_strength = 0
q = deque()
q.append((0, 0, 1, ports))
while q:
    pin, strength, length, ports = q.popleft()

    max_strength = max(max_strength, strength)

    if length > max_length:
        max_length = length
        max_length_strength = strength
    elif length == max_length:
        max_length_strength = max(max_length_strength, strength)

    for i, (a, b) in enumerate(ports):
        if a == pin:
            c = b
        elif b == pin:
            c = a
        else:
            c = None
        if c is not None:
            np = list(ports)
            np.pop(i)
            q.append((c, strength + a + b, length + 1, np))

# part 1
print(max_strength)

# part 2
print(max_length_strength)

# tests
if filename == "test.txt":
    assert max_strength == 31
    assert max_length_strength == 19
