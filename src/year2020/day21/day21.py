#!/usr/bin/env python3
# [Day 21: Allergen Assessment](https://adventofcode.com/2020/day/21)

import atexit
import itertools
import sys
import time
from pathlib import Path

verbose = "-v" in sys.argv
if verbose:
    sys.argv.remove("-v")
filename = ("test.txt" if sys.argv[1] == "-t" else sys.argv[1]) if len(sys.argv) > 1 else "input.txt"
data = Path(filename).read_text().strip()
if "--elapsed" in sys.argv:
    sys.argv.remove("--elapsed")
    start_time_ns = time.time_ns()
    atexit.register(lambda: print(f"elapsed: {(time.time_ns() - start_time_ns) / 1_000_000}ms"))


lines = data.splitlines()

menu = []
for line in lines:
    ingr, allerg = line.split("(contains ", maxsplit=1)
    ingr = ingr.split()
    allerg = allerg.removesuffix(")").split(", ")
    menu.append((set(ingr), set(allerg)))


allergens = {}
for ingr, allerg in menu:
    for allergen in allerg:
        if allergen in allergens:
            allergens[allergen].intersection_update(ingr)
        else:
            allergens[allergen] = set(ingr)


ingredients = set()
for i, _ in menu:
    ingredients.update(i)

# part 1
no_allergens = ingredients.difference(*itertools.chain(allergens.values()))
print(sum([len(no_allergens.intersection(ingr)) for ingr, _ in menu]))


# part 2
dangerous = []
while allergens:
    for k, v in allergens.items():
        if len(v) == 1:
            dangerous.append((k, v.pop()))
            break
    else:
        raise KeyError
    allergens.pop(dangerous[-1][0])
    for v in allergens.values():
        v.discard(dangerous[-1][1])
print(",".join(i for _, i in sorted(dangerous)))
