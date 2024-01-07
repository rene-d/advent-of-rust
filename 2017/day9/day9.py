#!/usr/bin/env python3
# [Day 9: Stream Processing](https://adventofcode.com/2017/day/9)

import sys
from pathlib import Path

filename = ("test.txt" if sys.argv[1] == "-t" else sys.argv[1]) if len(sys.argv) > 1 else "input.txt"
data = Path(filename).read_text().strip()


def compute(data, part1=True):
    in_group = 0
    in_garbage = False
    score = 0
    garbage = 0

    i = 0
    while i < len(data):
        c = data[i]

        if c == "!":
            i += 2
            continue

        if in_garbage:
            if c == ">":
                in_garbage = False
            else:
                garbage += 1
            i += 1
            continue

        if c == "<":
            in_garbage = True
            i += 1
            continue

        if c == "{":
            in_group += 1

        elif c == "}":
            score += in_group
            in_group -= 1
            assert in_group >= 0

        i += 1

    return score if part1 else garbage


assert compute("{}") == 1
assert compute("{{{}}}") == 6
assert compute("{{},{}}") == 5
assert compute("{{{},{},{{}}}}") == 16
assert compute("{<a>,<a>,<a>,<a>}") == 1
assert compute("{{<ab>},{<ab>},{<ab>},{<ab>}}") == 9
assert compute("{{<!!>},{<!!>},{<!!>},{<!!>}}") == 9
assert compute("{{<a!>},{<a!>},{<a!>},{<ab>}}") == 3

assert compute("<>", False) == 0
assert compute("<random characters>", False) == 17
assert compute("<<<<>", False) == 3
assert compute("<{!>}>", False) == 2
assert compute("<!!>", False) == 0
assert compute("<!!!>>", False) == 0
assert compute("""<{o"i!a,<{i<a>""", False) == 10

print(compute(data, True))
print(compute(data, False))
