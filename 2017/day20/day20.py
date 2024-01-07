#!/usr/bin/env python3
# [Day 20: Particle Swarm](https://adventofcode.com/2017/day/20)

import re
import sys
from collections import defaultdict, namedtuple
from pathlib import Path

verbose = "-v" in sys.argv
if verbose:
    sys.argv.remove("-v")
self_tests = "-T" in sys.argv
if self_tests:
    sys.argv.remove("-T")
filename = ("test.txt" if sys.argv[1] == "-t" else sys.argv[1]) if len(sys.argv) > 1 else "input.txt"
data = Path(filename).read_text().strip()
lines = data.splitlines()

# p=<-4897,3080,2133>, v=<-58,-15,-78>, a=<17,-7,0>

pattern = re.compile(r"^p=<(-?\d+),(-?\d+),(-?\d+)>, v=<(-?\d+),(-?\d+),(-?\d+)>, a=<(-?\d+),(-?\d+),(-?\d+)>$")

Particle = namedtuple("Particle", "px py pz vx vy vz ax ay az")
particles = []

for line in lines:
    m = pattern.match(line)
    p = Particle(*map(int, m.groups()))
    particles.append(p)


# part 1

t = 1000
md, mi = float("inf"), 0
for i, p in enumerate(particles):
    x = p.px + p.vx * t + p.ax * t * (t + 1) // 2
    y = p.py + p.vy * t + p.ay * t * (t + 1) // 2
    z = p.pz + p.vz * t + p.az * t * (t + 1) // 2
    d = abs(x) + abs(y) + abs(z)
    if md > d:
        md = d
        mi = i
print(mi)


# part 2

for t in range(0, 1000):
    collisions = defaultdict(list)

    for i, p in enumerate(particles):
        x = p.px + p.vx * t + p.ax * t * (t + 1) // 2
        y = p.py + p.vy * t + p.ay * t * (t + 1) // 2
        z = p.pz + p.vz * t + p.az * t * (t + 1) // 2
        collisions[x, y, z].append(i)

    np = []
    for v in collisions.values():
        if len(v) == 1:
            np.append(particles[v[0]])
    particles = np

print(len(particles))
