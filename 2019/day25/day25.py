#!/usr/bin/env python3
# https://adventofcode.com/2019/day/25

import argparse
import re
import sys
from functools import reduce
from pathlib import Path

sys.path.append(Path(__file__).parent.parent.as_posix())
from intcode.Intcode import Computer  # noqa

parser = argparse.ArgumentParser()
parser.add_argument("-v", "--verbose", action="store_true")
parser.add_argument("-m", "--manual", action="store_true", help="play the game")
parser.add_argument("input", nargs="?", default="input.txt")
args = parser.parse_args()

software = Path(args.input).read_text()


computer = Computer()
computer.load(software)
computer.start()


if not args.manual:
    if reduce(lambda a, b: a ^ b, computer.program) != -2251798974787211:
        print("work only for my puzzle input", file=sys.stderr)
        exit(2)

    # explore the spacecraft and take items - works only for my puzzle input
    explore_cmds = (
        "east,east,take semiconductor,north,take planetoid,west,take food ration,west,west,"
        + "take monolith,east,east,north,take space law space brochure,north,north,"
        + "take weather machine,south,south,south,east,north,take antenna,east,north,"
        + "south,north,west,east,south,south,east,north,south,west,east,west,south,east,"
        + "south,south,south,east,inv,west,west,west,north,north,west,north,north,south,"
        + "west,north,east,take jam,west,south,east,south,east,south,south,east,inv"
    ).split(",")

    # try all combinations of items - works only for my puzzle input
    solve_cmds = []
    items = "food ration,weather machine,antenna,space law space brochure,jam,semiconductor,planetoid,monolith".split(
        ","
    )
    for k in range(8):
        solve_cmds.append("drop " + items[k])

    previous = 0
    for i in range(1, 256):
        for k in range(8):
            if i & (1 << k) != 0:
                if previous & (1 << k) == 0:
                    solve_cmds.append("take " + items[k])
            if i & (1 << k) == 0:
                if (previous & (1 << k)) != 0:
                    solve_cmds.append("drop " + items[k])
        solve_cmds.append("inv")
        solve_cmds.append("east")
        previous = i

    # let's go
    for cmd in explore_cmds + solve_cmds:
        if args.verbose:
            print(f"> {cmd}")

        computer.input.extend(map(ord, cmd))
        computer.input.append(10)
        state = computer.resume()

        t = "".join(map(chr, computer.output))
        if args.verbose:
            print(f"\033[2m{t}\033[0m")

        computer.flush_io()

        if state != "read":
            answer = re.search(
                r"Oh, hello! You should be able to get in by typing (\d+) on the keypad at the main airlock.", t
            )
            if answer:
                answer = answer[1]
            break

    print(answer)

else:
    shortcuts = {
        "n": "north",
        "s": "south",
        "e": "east",
        "w": "west",
    }

    log = Path("moves.log").open("w")

    state = computer.run()
    while state == "read":
        t = "".join(map(chr, computer.output))
        print(f"\033[2m{t}\033[0m")
        computer.flush_io()

        # 't' to take the current item
        w = ""
        take = ""
        for line in t.splitlines():
            if line.startswith("Items here:"):
                w = "items"
            elif line.startswith("- ") and w == "items":
                take = "take " + line[2:]
                break
            else:
                w = ""

        while True:
            value = input("input> ")
            if value.strip() == "":
                continue

            if take and value == "t":
                value = take
            else:
                value = shortcuts.get(value, value)

            print(value, file=log)
            log.flush()

            computer.input.extend(map(ord, value))
            computer.input.append(10)
            break

        state = computer.resume()

    t = "".join(map(chr, computer.output))
    print(f"\033[2m{t}\033[0m")
