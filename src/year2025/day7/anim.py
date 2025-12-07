#!/usr/bin/env python3

import sys
import time
from copy import deepcopy
from pathlib import Path

RED = "\033[31m"
GREEN = "\033[32m"
LIGHTRED = "\033[91m"
LIGHTGREEN = "\033[92m"
YELLOW = "\033[93m"
BLUE = "\033[94m"
RESET = "\033[0m"
CLEAR = "\033[H\033[2J"
DARK = "\033[1;30m"


START = "S"
SPLITTER = "^"
BEAM = "|"
EMPTY = "."


def show(grid, seconds: float = 0.5, footer=None):
    sys.stdout.write(CLEAR)

    for row in grid:
        color = ""

        def pr(new_color: str, cell: str):
            nonlocal color
            if color != new_color:
                color = new_color
                sys.stdout.write(color)
            sys.stdout.write(cell)

        for cell in row:
            if cell == EMPTY:
                pr(DARK, EMPTY)
            elif cell == SPLITTER:
                pr(LIGHTGREEN, SPLITTER)
            elif cell == BEAM:
                pr(YELLOW, BEAM)
            elif cell == START:
                pr(LIGHTRED, START)
            else:
                print(cell)
                exit()

        print(RESET)

    if footer is not None:
        print()
        print(BLUE + f" {footer} ".center(width, "~") + RESET)

    time.sleep(seconds)


grid = list(list(line) for line in Path("test.txt").read_text().splitlines())
width = len(grid[0])
height = len(grid)
start_x = grid[0].index("S")


def part1():
    show(grid)

    beams = set([start_x])
    splits = 0

    for y in range(1, height):
        new_beams = set()

        for x in beams:
            if grid[y][x] == SPLITTER:
                splits += 1
                if x > 0:
                    new_beams.add(x - 1)
                if x < width - 1:
                    new_beams.add(x + 1)
            else:
                new_beams.add(x)

        beams = new_beams

        for x in beams:
            grid[y][x] = BEAM

        show(grid, footer=splits)


def part2(grid, x: int, y: int, count: int):
    if y == height:
        count += 1
        show(grid, 0.25, count)
        return count

    new_grid = deepcopy(grid)

    show(new_grid, 0.005, count)

    if grid[y][x] == SPLITTER:
        if x > 0:
            count = part2(new_grid, x - 1, y, count)
        if x < width - 1:
            count = part2(new_grid, x + 1, y, count)
    else:
        new_grid[y][x] = BEAM
        count = part2(new_grid, x, y + 1, count)

    return count


try:
    if len(sys.argv) < 2:
        show(grid, 0, "hello")
    elif sys.argv[1] == "1":
        part1()
    elif sys.argv[1] == "2":
        part2(grid, start_x, 1, 0)
except KeyboardInterrupt:
    pass
