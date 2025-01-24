#!/usr/bin/env python3
# [Day 20: Donut Maze](https://adventofcode.com/2019/day/20)

import sys
import unittest
from collections import defaultdict, deque
from pathlib import Path

verbose = "-v" in sys.argv
if verbose:
    sys.argv.remove("-v")
self_tests = "-T" in sys.argv
if self_tests:
    sys.argv.remove("-T")
filename = ("test.txt" if sys.argv[1] == "-t" else sys.argv[1]) if len(sys.argv) > 1 else "input.txt"
data = Path(filename).read_text()


class Puzzle:
    maze: set
    width: int
    height: int
    portals: dict
    portals_pos: dict

    def load(self, data):
        lines = data.splitlines()

        self.width, self.height = len(lines[0]), len(lines)

        maze = set()
        letters = dict()

        for y, line in enumerate(lines):
            for x, c in enumerate(line):
                if c == ".":
                    maze.add((x, y))
                elif c.isalpha():
                    letters[(x, y)] = c

        # map of coord/name
        portals_pos = dict()
        for x, y in letters:
            if (x + 1, y) in letters and (x + 2, y) in maze:
                portals_pos[x + 2, y] = letters[x, y] + letters[x + 1, y]

            if (x - 1, y) in letters and (x - 2, y) in maze:
                portals_pos[x - 2, y] = letters[x - 1, y] + letters[x, y]

            if (x, y + 1) in letters and (x, y + 2) in maze:
                portals_pos[x, y + 2] = letters[x, y] + letters[x, y + 1]

            if (x, y - 1) in letters and (x, y - 2) in maze:
                portals_pos[x, y - 2] = letters[x, y - 1] + letters[x, y]

        # map name/list of coords
        portals = defaultdict(list)
        for k, v in portals_pos.items():
            portals[v].append(k)

        self.maze = maze
        self.portals_pos = portals_pos
        self.portals = portals

        self.start = portals["AA"][0]
        self.end = portals["ZZ"][0]

    def part1(self):
        # map of teleports
        teleports = dict()
        for k, v in self.portals.items():
            if len(v) == 2:
                teleports[v[0]] = v[1]
                teleports[v[1]] = v[0]
            else:
                assert len(v) == 1

        # bfs
        seen = set()
        q = deque([(self.start, 0)])
        while q:
            (x, y), n = q.popleft()

            if (x, y) == self.end:
                return n

            for dx, dy in ((-1, 0), (1, 0), (0, -1), (0, 1)):
                nx, ny = x + dx, y + dy
                if (nx, ny) not in seen and (nx, ny) in self.maze:
                    seen.add((nx, ny))
                    q.append(((nx, ny), n + 1))

            t = teleports.get((x, y))
            if t and t not in seen:
                seen.add(t)
                q.append((t, n + 1))

    def part2(self):
        # init inner and outer portals
        inner_portals = {}
        outer_portals = {}
        for (x, y), v in self.portals_pos.items():
            if v == "AA" or v == "ZZ":
                continue
            if y == 2 or x == 2 or x == self.width - 3 or y == self.height - 3:
                outer_portals[x, y] = v
                outer_portals[v] = (x, y)
            else:
                inner_portals[x, y] = v
                inner_portals[v] = (x, y)

        # bfs
        seen = set()
        q = deque([(self.start, 0, 0, None)])
        while q:
            (x, y), n, level, portal = q.popleft()

            if (x, y, level) in seen:
                continue
            seen.add((x, y, level))

            if (x, y) == self.end and level == 0:
                return n

            for dx, dy in ((-1, 0), (1, 0), (0, -1), (0, 1)):
                nx, ny = x + dx, y + dy

                if level > 0 and ((nx, ny) == self.start or (nx, ny) == self.end):
                    continue

                if (nx, ny) in self.maze:
                    q.append(((nx, ny), n + 1, level, portal))

            if (x, y) in inner_portals:
                p = inner_portals[x, y]
                if p != portal:
                    t = outer_portals[p]
                    q.append((t, n + 1, level + 1, p))

            if (x, y) in outer_portals and level > 0:
                p = outer_portals[x, y]
                if p != portal:
                    t = inner_portals[p]
                    q.append((t, n + 1, level - 1, p))

        return 0


class Tests(unittest.TestCase):
    def test1(self):
        test_file = Path("sample_1.txt")
        self.assertTrue(test_file.is_file())

        p = Puzzle()
        p.load(test_file.read_text())
        self.assertEqual(p.part1(), 23)
        self.assertEqual(p.part2(), 26)

    def test2(self):
        test_file = Path("sample_2.txt")
        self.assertTrue(test_file.is_file())

        p = Puzzle()
        p.load(test_file.read_text())
        self.assertEqual(p.part1(), 58)

    def test3(self):
        test_file = Path("sample_3.txt")
        self.assertTrue(test_file.is_file())

        p = Puzzle()
        p.load(test_file.read_text())
        self.assertEqual(p.part2(), 396)


if self_tests:
    unittest.main(verbosity=2, exit=True)
else:
    p = Puzzle()
    p.load(data)

    print(p.part1())
    print(p.part2())
