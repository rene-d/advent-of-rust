#!/usr/bin/env python3
# https://adventofcode.com/2020/day/18

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


# part 1


def get_token(expr, start):
    current = start

    if expr[current] == "(":
        current += 1
        nested = 1
        while nested > 0 and current < len(expr):
            c = expr[current]
            if c == "(":
                nested += 1
            elif c == ")":
                nested -= 1
            current += 1
        return evaluate(expr[start + 1 : current - 1]), current

    assert expr[current].isdigit()
    while current < len(expr) and expr[current].isdigit():
        current += 1
    return int(expr[start:current]), current


def evaluate(expr):
    expr = expr.replace(" ", "")

    pos = 0
    ltoken, pos = get_token(expr, pos)

    while pos < len(expr):
        op = expr[pos]
        pos += 1

        rtoken, pos = get_token(expr, pos)

        if op == "+":
            ltoken += rtoken
        elif op == "*":
            ltoken *= rtoken
        else:
            assert False

    return ltoken


assert evaluate("1 + 2 * 3 + 4 * 5 + 6") == 71
assert evaluate("1 + (2 * 3) + (4 * (5 + 6))") == 51
assert evaluate("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2") == 13632

print(sum(evaluate(line) for line in lines))


# part 2


class A:
    def __init__(self, s):
        self.v = int(s)

    def __sub__(self, o):
        # actually * but with + precedence
        return A(self.v * o.v)

    def __truediv__(self, o):
        # actually + but with / precedence
        return A(self.v + o.v)


result = 0
for line in lines:
    # change the operators for their precedences
    expr = line.replace("+", "/").replace("*", "-")
    expr = re.sub(r"(\d+)", R"A(\1)", expr)
    result += eval(expr).v
print(result)
