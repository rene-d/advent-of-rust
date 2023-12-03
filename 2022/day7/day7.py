#!/usr/bin/env python3

import sys
from collections import defaultdict
from pathlib import Path

filename = ("test.txt" if sys.argv[1] == "-t" else sys.argv[1]) if len(sys.argv) > 1 else "input.txt"

path = Path("/")
dirs = set()
dir_size = defaultdict(lambda: 0)

for line in open(filename):
    line = line.strip()

    if line == "$ cd /":
        path = Path("/")
    elif line == "$ cd ..":
        path = path.parent
    elif line.startswith("$ cd "):
        assert "/" not in line[5:]
        assert len(line) > 5
        path /= line[5:]
    elif line == "$ ls":
        pass
    else:
        if line.startswith("dir "):
            pass
        else:
            size, file = line.split()
            size = int(size)
            dir_size[path] += size
    dirs.add(path)

total_dir_size = []
total = 0

# part one (and computing for part two)
part1 = 0
for dir in dirs:
    dir = dir.as_posix()
    s = 0
    for file_path, file_size in dir_size.items():
        file_path = file_path.as_posix()
        if file_path.startswith(dir):
            s += file_size
    total_dir_size.append(s)
    if s <= 100_000:
        part1 += s

    if dir == "/":
        total = s

print(part1)

# part two
for part2 in sorted(total_dir_size):
    if total - part2 + 30000000 <= 70000000:
        print(part2)
        break
