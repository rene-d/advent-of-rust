#!/usr/bin/env python3
# [Day 17: Pyroclastic Flow](https://adventofcode.com/2022/day/17)

# Nota: not sure to write it in Rust, nor to make it cleaner and more Pythonic...

import argparse
import time
from pathlib import Path

ROCKS = [
    [
        [1, 1, 1, 1],
    ],
    [
        [0, 1, 0],
        [1, 1, 1],
        [0, 1, 0],
    ],
    [
        [1, 1, 1],
        [0, 0, 1],
        [0, 0, 1],
    ],
    [
        [1],
        [1],
        [1],
        [1],
    ],
    [
        [1, 1],
        [1, 1],
    ],
]


class Tetris:
    def __init__(self, jets):

        self.jets = jets.strip()

        self.cave = [
            [1, 1, 1, 1, 1, 1, 1],  # the bottom
        ]
        self.bottom = 0

        self.jet_count = 0
        self.rock_count = 0

    def cave_height(self):
        """Returns cave height, including the bottom line."""

        return len(self.cave) + self.bottom

    def overlap(self, x, y, rock):
        width = len(rock[0])
        height = len(rock)
        for i in range(height):
            for j in range(width):
                if y + i < self.cave_height():
                    if self.cave[y + i - self.bottom][x + j] != 0 and rock[i][j] == 1:
                        return True
        return False

    def fall(self):

        rock = ROCKS[self.rock_count % len(ROCKS)]
        self.rock_count += 1

        width = len(rock[0])
        height = len(rock)

        y = self.cave_height() + 3
        x = 2

        while True:
            # current jet of gas
            gas = self.jets[self.jet_count % len(self.jets)]
            self.jet_count += 1

            # shift the rock if possible
            if gas == ">" and x + width + 1 <= 7 and not self.overlap(x + 1, y, rock):
                x += 1
            elif gas == "<" and x > 0 and not self.overlap(x - 1, y, rock):
                x -= 1

            # rock falls if possible
            if not self.overlap(x, y - 1, rock):
                y -= 1
            else:
                break

        for i in range(height):
            if y + i >= self.cave_height():
                self.cave.append([0, 0, 0, 0, 0, 0, 0])
            for j in range(width):
                if rock[i][j] == 1:
                    self.cave[y + i - self.bottom][x + j] = 65 + (self.rock_count - 1) % len(ROCKS)

        if len(self.cave) > 200:
            del self.cave[0:100]
            self.bottom += 100

    def show(self, gameover: bool):

        max_rows = 25
        print(end="\033[H\033[2J")

        h = self.cave_height() + 2

        for y in range(h - 1, self.bottom - 1, -1):
            s = "|" if y > 0 else "+"

            for x in range(0, 7):

                if y < self.cave_height():
                    c = self.cave[y - self.bottom][x]
                else:
                    c = 0

                if c == 0:
                    s += "  "
                elif c == 1:
                    s += "--"
                else:
                    s += "\033[" + str(c - 65 + 31) + "m██\033[0m"
                    # s += chr(c)█

            s += "|" if y > 0 else "+"

            if y == self.cave_height() - 1:
                print(f"{s} <= top ({y})")
            else:
                print(s)

            max_rows -= 1
            if max_rows == 0:
                time.sleep(0.1)
                break

        if self.bottom != 0:
            print(f"({self.bottom} rows suppressed)")

    def make_key(self):
        top = []
        mask = 0
        for y in range(len(self.cave) - 1, -1, -1):
            for i, c in enumerate(self.cave[y]):
                if c != 0:
                    mask |= 1 << i
            top.append(mask)
            if mask == 127:  # found a rock on all the 7 columns
                break
        return (self.rock_count % len(ROCKS), self.jet_count % len(self.jets), bytes(top))

    def solve(self, show=False):

        keys = {}
        heights = [0]
        start, end = None, None
        part1 = None

        for n in range(1, 10000):
            self.fall()

            if show:
                self.show(False)

            if n == 2022:
                part1 = self.cave_height() - 1

            if not end:
                heights.append(self.cave_height() - 1)
                key = self.make_key()
                if n > 2000 and key in keys:
                    start = keys[key]
                    end = n
                keys[key] = n

            if part1 and end:
                if show:
                    self.show(True)
                break

        else:
            print("no solution")
            exit()

        # part 1

        # part 2
        q, r = divmod(1_000_000_000_000 - start, end - start)
        part2 = heights[start + r] + q * (heights[end] - heights[start])

        return part1, part2


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("-v", "--verbose", action="store_true")
    parser.add_argument("-t", "--test", action="store_true")
    parser.add_argument("filename", nargs="?", type=Path, default="input.txt")
    args = parser.parse_args()
    if args.test:
        args.filename = Path("test.txt")

    data = args.filename.read_text().strip()

    tetris = Tetris(data)
    part1, part2 = tetris.solve(args.verbose)
    print(part1)
    print(part2)


if __name__ == "__main__":
    main()
