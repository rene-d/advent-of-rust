#!/usr/bin/env python3
# https://adventofcode.com/2015/day/24

from pathlib import Path
from copy import deepcopy
from collections import defaultdict, deque, namedtuple, Counter
import sys, re, math, itertools, time
from functools import reduce
from operator import mul
import re
import unittest

verbose = "-v" in sys.argv
if verbose:
    sys.argv.remove("-v")
self_tests = "-T" in sys.argv
if self_tests:
    sys.argv.remove("-T")
filename = ("test.txt" if sys.argv[1] == "-t" else sys.argv[1]) if len(sys.argv) > 1 else "input.txt"
data = Path(filename).read_text().strip()

packages = list(map(int, data.splitlines()))


def solve(ngroups):
    weight = sum(packages) // ngroups
    for n in range(1, len(packages) + 1):
        g = [x for x in itertools.combinations(packages, n) if sum(x) == weight]
        if g:
            return min(reduce(mul, p) for p in g)


print(solve(3))
print(solve(4))
