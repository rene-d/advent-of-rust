#!/usr/bin/env python3
# https://adventofcode.com/2015/day/23

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

program = list(line.replace(",", " ").split() for line in data.splitlines())


regs = {}


def run_instr(ip):
    op = program[ip]
    match op[0]:
        case "hlf":
            regs[op[1]] //= 2
        case "tpl":
            regs[op[1]] *= 3
        case "inc":
            regs[op[1]] += 1
        case "jmp":
            return int(op[1])
        case "jie":
            if (regs[op[1]] % 2) == 0:
                return int(op[2])
        case "jio":
            if regs[op[1]] == 1:
                return int(op[2])
    return 1


def run(a):
    regs["a"] = a
    regs["b"] = 0
    ip = 0
    while ip < len(program):
        ip += run_instr(ip)
    return regs["b"]


print(run(0))
print(run(1))
