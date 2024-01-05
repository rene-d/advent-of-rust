#!/usr/bin/env python3
# https://adventofcode.com/2018/day/8

from pathlib import Path
from copy import deepcopy
from collections import defaultdict, deque, namedtuple, Counter
import sys, re, math, itertools, time
from functools import reduce
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
nodes = list(map(int, data.split()))


def rec(pos):
    children, metadata = nodes[pos : pos + 2]
    pos += 2
    sum_metadata = 0

    for _ in range(children):
        pos, m = rec(pos)
        sum_metadata += m

    sum_metadata += sum(nodes[pos : pos + metadata])
    pos += metadata

    return pos, sum_metadata


print(rec(0)[1])


def rec2(pos):
    children, metadata = nodes[pos : pos + 2]
    pos += 2

    if children == 0:
        value = sum(nodes[pos : pos + metadata])

    else:
        values = []
        for _ in range(children):
            pos, n = rec2(pos)
            values.append(n)

        value = 0
        for n in nodes[pos : pos + metadata]:
            if 0 < n <= len(values):
                value += values[n-1]

    pos += metadata
    return pos, value


print(rec2(0)[1])
