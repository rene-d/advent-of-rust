#!/usr/bin/env python3

import sys
from pathlib import Path

sys.path.append(Path(__file__).parent.parent.as_posix())
from ocr.ocr import ocr  # noqa

filename = ("test.txt" if sys.argv[1] == "-t" else sys.argv[1]) if len(sys.argv) > 1 else "input.txt"
data = Path(filename).read_text()

X = 1
cycles = [X]
for line in data.splitlines():
    if line == "noop":
        cycles.append(X)
    elif line.startswith("addx "):
        cycles.append(X)
        X += int(line[5:])
        cycles.append(X)
    else:
        if line:
            raise ValueError

# part one
signal_strength = 0
for i, x in enumerate(cycles, 1):
    if (i + 20) % 40 == 0:
        signal_strength += i * x
print(signal_strength)

# part two
it_x = iter(cycles)
crt = ""
for _ in range(6):
    crt_line = ""
    for pixel in range(1, 41):
        sprite = next(it_x)
        if sprite <= pixel < sprite + 3:
            crt_line += "#"
        else:
            crt_line += "."
    crt += crt_line + "\n"
print(ocr(crt))
