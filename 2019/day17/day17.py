#!/usr/bin/env python3
# https://adventofcode.com/2019/day/17

import sys
from pathlib import Path

sys.path.append(Path(__file__).parent.parent.as_posix())
from intcode.Intcode import Computer  # noqa

filename = sys.argv[1] if len(sys.argv) > 1 else "input.txt"
data = Path(filename).read_text()

aft = Computer()
aft.load(data)

# part 1
aft.run()
o = list(aft.output)
aft.flush_io()
w = o.index(10)
h = len(o) // (w + 1)
grid = []
for i in range(0, (w + 1) * h, w + 1):
    assert o[i + w] == 10
    grid.append(o[i : i + w])
w, h = len(grid[0]), len(grid)
part1 = 0
for y in range(1, h - 1):
    for x in range(1, w - 1):
        if (
            grid[y][x] == 35
            and grid[y - 1][x] == 35
            and grid[y + 1][x] == 35
            and grid[y][x - 1] == 35
            and grid[y][x + 1] == 35
        ):
            part1 += x * y
            grid[y][x] = ord("O")
# print("\n".join("".join(map(chr, row)) for row in grid))
print(part1)

# part 2
if sum(aft.program) != 2170216:
    print("part2: find your own solution", file=sys.stderr)
    print("???")
    exit(0)
aft.flush_io()
aft.start()
aft._poke(0, 2)  # wake up the robot

# by hand and eyes
aft.input.extend(map(ord, "A,B,A,B,C,C,B,A,B,C\n"))
aft.input.extend(map(ord, "L,12,L,10,R,8,L,12\n"))
aft.input.extend(map(ord, "R,8,R,10,R,12\n"))
aft.input.extend(map(ord, "L,10,R,12,R,8\n"))
aft.input.extend(map(ord, "n\n"))

assert aft.resume() == "halted"
print(aft.output.pop())
