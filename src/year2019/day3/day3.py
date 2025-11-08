#!/usr/bin/env python3
# [Day 3: Crossed Wires](https://adventofcode.com/2019/day/3)

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


def draw(instr):
    x, y = 0, 0

    line = set()
    # line.add((x, y))  # do not add the central port

    for s in instr.split(","):
        dir, n = s[0], int(s[1:])

        dx, dy = {"R": (1, 0), "L": (-1, 0), "D": (0, -1), "U": (0, 1)}[dir]

        for _ in range(n):
            x, y = x + dx, y + dy
            line.add((x, y))

    return line


def steps(instr, target):
    x, y = 0, 0
    count = 0
    for s in instr.split(","):
        dir, n = s[0], int(s[1:])
        dx, dy = {"R": (1, 0), "L": (-1, 0), "D": (0, -1), "U": (0, 1)}[dir]
        for _ in range(n):
            x, y = x + dx, y + dy
            count += 1
            if (x, y) == target:
                return count
    return 0


def manhattan(a, b):
    return abs(a[0] - b[0]) + abs(a[1] - b[1])


def part1(line1, line2):
    l1 = draw(line1)
    l2 = draw(line2)
    return min(manhattan(x, (0, 0)) for x in l1.intersection(l2))


def part2(line1, line2):
    l1 = draw(line1)
    l2 = draw(line2)
    return min(steps(line1, x) + steps(line2, x) for x in l1.intersection(l2))


assert part1("R8,U5,L5,D3", "U7,R6,D4,L4") == 6
assert part1("R75,D30,R83,U83,L12,D49,R71,U7,L72", "U62,R66,U55,R34,D71,R55,D58,R83") == 159
assert part1("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51", "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7") == 135

assert part2("R8,U5,L5,D3", "U7,R6,D4,L4") == 30
assert part2("R75,D30,R83,U83,L12,D49,R71,U7,L72", "U62,R66,U55,R34,D71,R55,D58,R83") == 610
assert part2("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51", "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7") == 410

print(part1(lines[0], lines[1]))
print(part2(lines[0], lines[1]))
