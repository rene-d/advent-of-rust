#!/usr/bin/env python3

# Day 8: Seven Segment Search
# https://adventofcode.com/2021/day/8

from collections import defaultdict
import sys

filename = ("test.txt" if sys.argv[1] == "-t" else sys.argv[1]) if len(sys.argv) > 1 else "input.txt"
data = open(filename).readlines()


# Part 1
digit_one = 0
digit_four = 0
digit_seven = 0
digit_eight = 0

for line in data:
    x = line.split("|", maxsplit=1)[1].strip()
    for c in x.split():
        if len(c) == 2:
            digit_one += 1
        elif len(c) == 3:
            digit_seven += 1
        elif len(c) == 4:
            digit_four += 1
        if len(c) == 7:
            digit_eight += 1

print(digit_one + digit_four + digit_seven + digit_eight)

# part 2
total = 0
for line in data:
    notes, code = line.split("|", maxsplit=1)

    notes = notes.strip().split()
    code = code.strip().split()

    d = defaultdict(lambda: set())

    for c in notes:
        d[len(c)].add("".join(sorted(c)))

    # 2 segments : 1
    # 3 segments : 7
    # 4 segments : 4
    # 5 segments : 2 3 5
    # 6 segments : 0 6 9
    # 7 segments : 8

    map = {}

    one = d[2].pop()
    seven = d[3].pop()
    four = d[4].pop()
    eight = d[7].pop()

    for x in d[6]:  # 0 6 9
        if one[0] in x and one[1] in x:
            # si on n'a pas les deux segments du 1
            if four[0] in x and four[1] in x and four[2] in x and four[3] in x:
                # si 4 est inclus, c'est forcément un 9
                nine = x
            else:
                # sinon, c'est forcément un 0
                zero = x
        else:
            # sinon, c'est forcément un 6
            six = x

    for x in d[5]:  # 2 3 5
        # si 7 est inclus, c est forcément un 3
        if seven[0] in x and seven[1] in x and seven[2] in x:
            three = x
        # si c'est inclus dans 9, c'est forcément un 5
        elif x[0] in nine and x[1] in nine and x[2] in nine and x[3] in nine and x[4] in nine:
            five = x
        # sinon, c'est forcément un 2
        else:
            two = x

    # print(zero, ": 0")
    # print(one, ": 1")
    # print(two, ": 2")
    # print(three, ": 3")
    # print(four, ": 4")
    # print(five, ": 5")
    # print(six, ": 6")
    # print(seven, ": 7")
    # print(eight, ": 8")
    # print(nine, ": 9")

    map[zero] = 0
    map[one] = 1
    map[two] = 2
    map[three] = 3
    map[four] = 4
    map[five] = 5
    map[six] = 6
    map[seven] = 7
    map[eight] = 8
    map[nine] = 9

    r = 0
    for d in code:
        c = map["".join(sorted(d))]
        r = r * 10 + c
    total += r
print(total)
