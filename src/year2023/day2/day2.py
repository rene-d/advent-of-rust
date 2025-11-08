#!/usr/bin/env python3
# [Day 2: Cube Conundrum](https://adventofcode.com/2023/day/2)

import atexit
import re
import sys
import time
from pathlib import Path

filename = ("test.txt" if sys.argv[1] == "-t" else sys.argv[1]) if len(sys.argv) > 1 else "input.txt"
data = Path(filename).read_text().strip()
if "--elapsed" in sys.argv:
    sys.argv.remove("--elapsed")
    start_time_ns = time.time_ns()
    atexit.register(lambda: print(f"elapsed: {(time.time_ns() - start_time_ns) / 1_000_000}ms"))


# parse input
lines = data.splitlines()

result = 0
for line in lines:
    id, sets = re.match(r"^Game (\d+): (.+)$", line).groups()
    ko = False
    for subset in sets.split(";"):
        for cube in subset.split(","):
            cube, color = cube.strip().split(maxsplit=1)
            cube = int(cube)
            if color == "red" and cube > 12:
                ko = True
            elif color == "green" and cube > 13:
                ko = True
            elif color == "blue" and cube > 14:
                ko = True
            if ko:
                break
        if ko:
            break
    if not ko:
        result += int(id)
print(result)


result = 0
for line in lines:
    id, sets = re.match(r"^Game (\d+): (.+)$", line).groups()

    red, green, blue = 0, 0, 0
    for subset in sets.split(";"):
        for cube in subset.split(","):
            cube, color = cube.strip().split(maxsplit=1)
            cube = int(cube)
            if color == "red":
                red = max(red, cube)
            elif color == "green":
                green = max(green, cube)
            elif color == "blue":
                blue = max(blue, cube)
    result += red * green * blue
print(result)
