#!/usr/bin/env python3

import sys

filename = ("test.txt" if sys.argv[1] == "-t" else sys.argv[1]) if len(sys.argv) > 1 else "input.txt"
data = open(filename).read()

SCORE_WIN = 6
SCORE_DRAW = 3
SCORE_LOOSE = 0

ROCK = 1
PAPER = 2
SCISSORS = 3

NEED_TO_LOOSE = 1
NEED_TO_DRAW = 2
NEED_TO_WIN = 3

part1 = 0
part2 = 0

for i in data.splitlines():
    opponent, me = i.split()

    opponent = ord(opponent) - ord("A") + 1
    me = ord(me) - ord("X") + 1

    # part 1
    if opponent == me:
        part1 += me + SCORE_DRAW

    elif opponent == ROCK and me == SCISSORS:  # the rock breaks the scissors
        part1 += me + SCORE_LOOSE
    elif opponent == SCISSORS and me == PAPER:  # the scissors cut the paper
        part1 += me + SCORE_LOOSE
    elif opponent == PAPER and me == ROCK:  # the paper covers the scissors
        part1 += me + SCORE_LOOSE

    else:
        part1 += me + SCORE_WIN

    # part 2
    if me == NEED_TO_DRAW:
        part2 += opponent + SCORE_DRAW

    elif me == NEED_TO_LOOSE:
        if opponent == ROCK:
            part2 += SCISSORS + SCORE_LOOSE
        elif opponent == PAPER:
            part2 += ROCK + SCORE_LOOSE
        elif opponent == SCISSORS:
            part2 += PAPER + SCORE_LOOSE
    else:
        if opponent == ROCK:
            part2 += PAPER + SCORE_WIN
        elif opponent == PAPER:
            part2 += SCISSORS + SCORE_WIN
        elif opponent == SCISSORS:
            part2 += ROCK + SCORE_WIN

print(part1)
print(part2)
