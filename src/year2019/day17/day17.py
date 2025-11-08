#!/usr/bin/env python3
# [Day 17: Set and Forget](https://adventofcode.com/2019/day/17)

import atexit
import sys
import time
from pathlib import Path

verbose = "-v" in sys.argv
if verbose:
    sys.argv.remove("-v")
filename = sys.argv[1] if len(sys.argv) > 1 else "input.txt"
data = Path(filename).read_text()
if "--elapsed" in sys.argv:
    sys.argv.remove("--elapsed")
    start_time_ns = time.time_ns()
    atexit.register(lambda: print(f"elapsed: {(time.time_ns() - start_time_ns) / 1_000_000}ms"))

#################################################################


sys.path.append(Path(__file__).parent.parent.as_posix())
from intcode.Intcode import Computer  # noqa


# part 1
def part1():
    aft = Computer()
    aft.load(data)

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

    if verbose:
        print("\n".join("".join(map(chr, row)) for row in grid))

    print(part1)


#################################################################


def part2():
    aft = Computer()
    aft.load(data)
    aft.run()

    # parse the output
    scaffold = set()
    x, y = 0, 0
    while aft.output:
        c = chr(aft.output.pop())
        match c:
            case "#":
                scaffold.add((x, y))
            case ".":
                pass
            case "\n":
                y += 1
                x = 0
                continue
            case "^" | "v" | "<" | ">":
                direction = {"^": 0, ">": 1, "v": 2, "<": 3}[c]
                position = (x, y)
        x += 1

    # build the scaffolding
    dxy = ((0, 1), (1, 0), (0, -1), (-1, 0))
    path = ""
    x, y = position
    while True:
        for d in (-1, 1):
            dx, dy = dxy[(direction + d) % 4]
            if (x + dx, y + dy) in scaffold:
                break
        else:
            break

        length = 0
        while (x + dx, y + dy) in scaffold:
            length += 1
            x, y = x + dx, y + dy

        direction = (direction + d) % 4

        d = {1: "L", -1: "R"}[d]
        path += f"{d},{length},"

    # compress the path
    routine = []
    functions = ["" for _ in range(3)]

    def compress(path: str) -> bool:
        # nonlocal: routine, functions

        if len(path) == 0:
            return False

        if len(routine) > 10:
            # 10 function calls + comma at most
            return True

        for i, name in enumerate("ABC"):
            routine.append(name)

            if needle := functions[i]:
                if path.startswith(needle):
                    if not compress(path.removeprefix(needle)):
                        return False
            else:
                p = 0
                while True:
                    p = path.find(",", p + 1)
                    p = path.find(",", p + 1)
                    if p == -1:
                        break

                    needle, remaining = path[: p + 1], path[p + 1 :]

                    if len(needle) > 21:
                        break

                    functions[i] = needle
                    if not compress(remaining):
                        return False
                    functions[i] = ""

            routine.pop()

        return True

    compress(path)

    if verbose:
        print(",".join(routine))
        print("\n".join(functions))

    aft.flush_io()
    aft.start()
    aft._poke(0, 2)  # wake up the robot

    # convert to robot instructions
    aft.input.extend(map(ord, ",".join(routine) + "\n"))
    for i in range(3):
        aft.input.extend(map(ord, functions[i].rstrip(",") + "\n"))
    aft.input.extend(map(ord, "n\n"))

    assert aft.resume() == "halted"
    print(aft.output.pop())


part1()
part2()
