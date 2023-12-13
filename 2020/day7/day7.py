#!/usr/bin/env python3
# https://adventofcode.com/2020/day/7

from pathlib import Path
from copy import deepcopy
from collections import defaultdict, deque, namedtuple
import sys, re, math, itertools, time
from functools import reduce
import re

verbose = "-v" in sys.argv
if verbose:
    sys.argv.remove("-v")
filename = ("test.txt" if sys.argv[1] == "-t" else sys.argv[1]) if len(sys.argv) > 1 else "input.txt"
data = Path(filename).read_text().strip()
lines = data.splitlines()


bags = {}

for line in lines:
    bag, contain = re.match(r"^(.+) bags contain (.+)\.$", line).groups()

    contained = {}
    for s in contain.split(","):
        if s == "no other bags":
            continue
        n, sub_bag = re.match(r"^\s?(\d+) (.+) bags?$", s).groups()
        contained[sub_bag] = int(n)

    bags[bag] = contained


# part 1

def contains_color(bag, color):
    for sub_bag in bags[bag]:
        if sub_bag == color:
            return True
        if contains_color(sub_bag, color) == True:
            return True
    return False


print(sum(contains_color(bag, "shiny gold") for bag in bags))


# part 2

def count(bag):
    t = 0
    for s, n in bags[bag].items():
        t += (count(s) + 1) * n
    return t


print(count("shiny gold"))
