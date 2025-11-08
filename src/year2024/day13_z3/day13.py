#!/usr/bin/env python3
# [Day 13: Claw Contraption](https://adventofcode.com/2024/day/13)

import atexit
import re
import sys
import time
from dataclasses import dataclass
from pathlib import Path

import z3

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


@dataclass
class ClawMachine:
    a_x: int
    a_y: int
    b_x: int
    b_y: int
    p_x: int
    p_y: int

    def price(self, p_offset=0):
        a = z3.Int("a")
        b = z3.Int("b")
        s = z3.Solver()
        s.add(a * self.a_x + b * self.b_x == self.p_x + p_offset)
        s.add(a * self.a_y + b * self.b_y == self.p_y + p_offset)
        s.add(a >= 0)
        s.add(b >= 0)
        if s.check() == z3.sat:
            m = s.model()
            return 3 * m[a].as_long() + m[b].as_long()
        return 0


machines = []
for i in data.split("\n\n"):
    claw = ClawMachine(0, 0, 0, 0, 0, 0)

    claw.a_x, claw.a_y = map(int, re.search(r"Button A: X\+(\d+), Y\+(\d+)", i).groups())
    claw.b_x, claw.b_y = map(int, re.search(r"Button B: X\+(\d+), Y\+(\d+)", i).groups())
    claw.p_x, claw.p_y = map(int, re.search(r"Prize: X=(\d+), Y=(\d+)", i).groups())

    machines.append(claw)


print(sum(machine.price() for machine in machines))

print(sum(machine.price(10000000000000) for machine in machines))
