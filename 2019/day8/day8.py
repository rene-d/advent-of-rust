#!/usr/bin/env python3
# https://adventofcode.com/2019/day/8

from pathlib import Path
import sys


sys.path.append("..")
from ocr.ocr import ocr


def chunker(seq, size):
    return (seq[pos : pos + size] for pos in range(0, len(seq), size))


filename = "test.txt" if len(sys.argv) > 1 and sys.argv[1] == "-t" else "input.txt"
data = Path(filename).read_text().strip()


# part 1
layers = []
for i, layer in enumerate(chunker(data, 25 * 6), 1):
    layers.append((layer.count("0"), layer.count("1") * layer.count("2")))
print(sorted(layers)[0][1])

# part 2
w, h = 25, 6
image = [[" "] * w for _ in range(h)]
for y in range(h):
    for x in range(w):
        for i in range(len(data) // (w * h)):
            pixel = data[(x + y * w) + i * w * h]
            if pixel == "2":
                continue
            if pixel == "1":
                image[h - 1 - y][x] = "#"
            if pixel == "0":
                image[h - 1 - y][x] = "."
            break
password = "\n".join("".join(row) for row in reversed(image))
print(ocr(password))
