#!/usr/bin/env python3
# [Day 24: Planet of Discord](https://adventofcode.com/2019/day/24)

import sys
import unittest
from collections import Counter
from copy import deepcopy
from pathlib import Path

verbose = "-v" in sys.argv
if verbose:
    sys.argv.remove("-v")
self_tests = "-T" in sys.argv
if self_tests:
    sys.argv.remove("-T")
filename = ("test.txt" if sys.argv[1] == "-t" else sys.argv[1]) if len(sys.argv) > 1 else "input.txt"
data = Path(filename).read_text().strip()


class Part1:
    def __init__(self, data):
        self.bugs = [[0 for _ in range(5)] for _ in range(5)]
        for y, row in enumerate(data.splitlines()):
            for x, c in enumerate(row):
                self.bugs[y][x] = c

    def evolve(self):
        bugs2 = deepcopy(self.bugs)
        for y in range(5):
            for x in range(5):
                adj = 0
                for dx, dy in ((0, 1), (0, -1), (1, 0), (-1, 0)):
                    if 0 <= x + dx < 5 and 0 <= y + dy < 5:
                        if self.bugs[y + dy][x + dx] == "#":
                            adj += 1

                if self.bugs[y][x] == "#":
                    # die unless there is exactly one bug
                    if adj != 1:
                        bugs2[y][x] = "."
                else:
                    # becomes infested with a bug if exactly one or two bugs
                    if adj == 1 or adj == 2:
                        bugs2[y][x] = "#"
        self.bugs = bugs2

    def biodiversity_rating(self):
        rating = 0
        for y in range(5):
            for x in range(5):
                if self.bugs[y][x] == "#":
                    rating = rating | 1 << (y * 5 + x)
        return rating

    def solve(self):
        seen = set()
        while True:
            r = self.biodiversity_rating()
            if r in seen:
                return r
            seen.add(r)
            self.evolve()


class Part2:
    def __init__(self, data):
        self.bugs = set()
        for y, row in enumerate(data.splitlines()):
            for x, c in enumerate(row):
                if c == "#":
                    self.bugs.add((x, y, 0))

    def neighbors(self, bug):
        x, y, level = bug
        n = []

        # same level
        for dx, dy in ((0, 1), (0, -1), (1, 0), (-1, 0)):
            if 0 <= x + dx < 5 and 0 <= y + dy < 5 and not (x + dx == 2 and y + dy == 2):
                n.append((x + dx, y + dy, level))

        # inner
        if (x, y) == (2, 1):
            n.extend((xx, 0, level + 1) for xx in range(5))
        elif (x, y) == (2, 3):
            n.extend((xx, 4, level + 1) for xx in range(5))
        elif (x, y) == (1, 2):
            n.extend((0, yy, level + 1) for yy in range(5))
        elif (x, y) == (3, 2):
            n.extend((4, yy, level + 1) for yy in range(5))

        else:
            # outer
            if x == 0:
                n.append((1, 2, level - 1))
            elif x == 4:
                n.append((3, 2, level - 1))
            if y == 0:
                n.append((2, 1, level - 1))
            elif y == 4:
                n.append((2, 3, level - 1))

        return n

    def evolve(self):
        adjs = Counter(adj for bug in self.bugs for adj in self.neighbors(bug))

        bugs2 = set()
        for a, n in adjs.items():
            if a in self.bugs:
                if n == 1:
                    bugs2.add(a)
            else:
                if n == 1 or n == 2:
                    bugs2.add(a)

        self.bugs = bugs2

    def solve(self, iterations=200):
        for _ in range(iterations):
            self.evolve()
        return len(self.bugs)


class Tests(unittest.TestCase):
    def test1(self):
        test_file = Path("test.txt")
        self.assertTrue(test_file.is_file())
        p = Part1(test_file.read_text())
        self.assertEqual(p.solve(), 2129920)

    def test2(self):
        test_file = Path("test.txt")
        self.assertTrue(test_file.is_file())
        p = Part2(test_file.read_text())
        self.assertEqual(p.solve(10), 99)


if self_tests:
    unittest.main(verbosity=2, exit=True)

print(Part1(data).solve())
print(Part2(data).solve())
