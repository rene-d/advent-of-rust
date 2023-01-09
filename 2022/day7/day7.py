#!/usr/bin/env python3

from pathlib import Path
import sys

filename = ("test.txt" if sys.argv[1] == "-t" else sys.argv[1]) if len(sys.argv) > 1 else "input.txt"

path = Path("/")
dirs = set()
files = {}

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

            files[path / file] = size
    dirs.add(path)

dirs_size=[]
total=0

# part one (and computing for part two)
part1 = 0
for dir in dirs:
    s = 0
    for file_path, file_size in files.items():
        if file_path.as_posix().startswith(dir.as_posix()):
            s += file_size
    if s <= 100000:
        part1 += s

    dirs_size.append(s)
    if dir.as_posix() == "/":
        total = s

print(part1)

# part two
for part2 in sorted(dirs_size):
    if total - part2 + 30000000 <= 70000000:
        print(part2)
        break

