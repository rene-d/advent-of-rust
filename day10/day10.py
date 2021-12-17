#!/usr/bin/env python3


import sys

data = open("input.txt" if len(sys.argv) == 1 else sys.argv[1]).read().splitlines()


def check(line):
    stack = []
    corrupted = {")": 3, "]": 57, "}": 1197, ">": 25137}
    completed = {")": 1, "]": 2, "}": 3, ">": 4}

    for c in line:
        if c == "<":
            stack.append(">")
        elif c == "(":
            stack.append(")")
        elif c == "[":
            stack.append("]")
        elif c == "{":
            stack.append("}")
        else:
            d = stack.pop()
            if c != d:
                return corrupted[c], 0

    score = 0
    while stack:
        score = score * 5 + completed[stack.pop()]

    return 0, score


part1 = 0
part2 = []
for line in data:
    corrupted, completed = check(line)
    part1 += corrupted
    if completed != 0:
        part2.append(completed)
print(part1)
part2 = sorted(part2)
print(part2[len(part2) // 2])
