#!/usr/bin/env python3

from pathlib import Path
import sys

filename = "test.txt" if len(sys.argv) > 1 and sys.argv[1] == "-t" else "input.txt"
data = Path(filename).read_text()

X = 1
cycles = []
for line in data.splitlines():
    if line == "noop":
        cycles.append(X)
    elif line.startswith("addx "):
        cycles.append(X)
        X += int(line[5:])
        cycles.append(X)
    else:
        raise ValueError

# part one
signal_strength = 0
for i, x in enumerate(cycles, 2):
    if (i + 20) % 40 == 0:
        signal_strength += i * x
print(signal_strength)

# part two
sprite = 1
it_x = iter(cycles)
for _ in range(6):
    crt = ""
    for pixel in range(1,41):
        if sprite <= pixel < sprite + 3:
            crt += "#"
        else:
            crt += "."
        sprite = next(it_x)
    print(crt)
