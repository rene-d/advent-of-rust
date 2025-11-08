#!/usr/bin/env python3
# [Day 25: Cryostasis](https://adventofcode.com/2019/day/25)

import argparse
import atexit
import re
import sys
import time
import typing as t
from collections import defaultdict
from pathlib import Path

sys.path.append(Path(__file__).parent.parent.as_posix())
from intcode.Intcode import Computer  # noqa


def run(computer, command=None):

    computer.flush_io()

    if command:
        computer.input.extend(map(ord, command))
        computer.input.append(10)

    computer.resume()

    return "".join(map(chr, computer.output))


def parse(out: str) -> t.Tuple[str, t.List, t.List]:

    room = None
    dirs = []
    items = []

    for line in out.splitlines():
        if line.startswith("== ") and line.endswith(" =="):
            if not room:
                room = line[3:-3]
        if line.startswith("- "):
            line = line.removeprefix("- ")
            if line in ("north", "east", "south", "west"):
                dirs.append(line)
            else:
                items.append(line)

    return room, dirs, items


def reverse(dir: str) -> str:
    match dir:
        case "north":
            return "south"
        case "south":
            return "north"
        case "east":
            return "west"
        case "west":
            return "east"

    raise ValueError(dir)


def explore(computer, map, out):

    room, dirs, items = parse(out)

    for dir in dirs:
        if dir in map[room]:
            continue

        # go next room
        out = run(computer, dir)

        newroom, _, _ = parse(out)
        known = newroom in map

        map[room][dir] = newroom
        map[newroom][reverse(dir)] = room

        if "A loud" in out:
            continue

        if not known:
            explore(computer, map, out)

        # go back
        out = run(computer, reverse(dir))

    for item in items:

        cmd = f"take {item}"

        temp_computer = computer.clone()
        temp_computer.max_iterations = 10000  # prevent infinite loop
        run(temp_computer, cmd)

        if temp_computer._state == "read":
            out = run(temp_computer, "inv")
            if "Items in your inventory" in out:
                run(computer, cmd)


def find_path(map, current, target, seen=set()) -> t.List[str]:

    seen.add(current)
    if current == target:
        return []

    for dir, cell in map[current].items():
        if cell not in seen:
            if (path := find_path(map, cell, target, seen)) is not None:
                return [dir] + path


def find_weight(computer, inventory, checkpoint_dir) -> str:

    prev_code = 0
    for cod in range(1 << len(inventory)):

        gray_code = cod ^ (cod >> 1)

        diff = gray_code - prev_code

        if diff != 0:
            cmd = "drop" if diff > 0 else "take"
            diff = abs(diff)
            item = 0
            while diff & 1 == 0:
                diff >>= 1
                item += 1
            run(computer, f"{cmd} {inventory[item]}")

        out = run(computer, checkpoint_dir)
        answer = re.search(r"You should be able to get in by typing (\d+) on the keypad at the main airlock.", out)
        if answer:
            return answer[1]

        prev_code = gray_code


def solve(software):

    computer = Computer()
    computer.load(software)
    computer.start()

    map = defaultdict(dict)

    # start up
    out = run(computer)
    start_room, _, _ = parse(out)

    # visit all rooms and take items
    explore(computer, map, out)

    # find path to the detection room
    path = find_path(map, start_room, "Pressure-Sensitive Floor")

    # the direction to get to Security Checkpoint from Pressure-Sensitive Floor
    checkpoint_dir = [k for k, v in map["Security Checkpoint"].items() if v == "Pressure-Sensitive Floor"][0]

    # go to the Pressure-Sensitive Floor
    for step in path:
        run(computer, step)

    # the inventory
    out = run(computer, "inv")
    inventory = [s.removeprefix("- ") for s in out.splitlines() if s.startswith("- ")]

    # get the unlock code
    unlock = find_weight(computer, inventory, checkpoint_dir)

    print(unlock)


def main():

    parser = argparse.ArgumentParser()
    parser.add_argument("-v", "--verbose", action="store_true")
    parser.add_argument("--elapsed", action="store_true")
    parser.add_argument("input", nargs="?", default="input.txt")
    args = parser.parse_args()

    software = Path(args.input).read_text()

    if args.elapsed:
        start_time_ns = time.time_ns()
        atexit.register(lambda: print(f"elapsed: {(time.time_ns() - start_time_ns) / 1_000_000}ms"))

    solve(software)


if __name__ == "__main__":
    main()
