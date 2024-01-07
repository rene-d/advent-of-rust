#!/usr/bin/env python3
# [Day 21: Monkey Math](https://adventofcode.com/2022/day/21)

import sys
from pathlib import Path

from sympy import Integer, factor, symbols
from sympy.solvers import solve

filename = ("test.txt" if sys.argv[1] == "-t" else sys.argv[1]) if len(sys.argv) > 1 else "input.txt"
data = Path(filename).read_text()
lines = data.splitlines()


def part1():
    monkeys = {}
    nums = {}

    for line in lines:
        monkey, job = line.split(":")
        job = job.strip()
        if job.isdigit():
            nums[monkey] = int(job)
        else:
            job = job.split()
            monkeys[monkey] = job

    def f(monkey):
        if monkey in nums:
            return nums[monkey]

        m1, op, m2 = monkeys[monkey]

        if isinstance(m1, str):
            m1 = f(m1)
        if isinstance(m2, str):
            m2 = f(m2)

        match op:
            case "+":
                r = m1 + m2
            case "-":
                r = m1 - m2
            case "*":
                r = m1 * m2
            case "/":
                assert m1 % m2 == 0
                r = m1 // m2
            case _:
                assert False

        del monkeys[monkey]
        nums[monkey] = r
        return r

    print(f("root"))


def part2():
    monkeys = {}
    nums = {}

    for line in lines:
        monkey, job = line.split(":")
        job = job.strip()
        if job.isdigit():
            nums[monkey] = Integer(job)
        else:
            job = job.split()
            monkeys[monkey] = job

    def f(monkey):
        if monkey == "humn":
            return symbols("n")

        if monkey in nums:
            return nums[monkey]

        m1, op, m2 = monkeys[monkey]

        if isinstance(m1, str):
            m1 = f(m1)
        if isinstance(m2, str):
            m2 = f(m2)

        if op == "+":
            r = m1 + m2
        elif op == "-":
            r = m1 - m2
        elif op == "*":
            r = m1 * m2
        elif op == "/":
            r = m1 / m2
        else:
            assert False

        del monkeys[monkey]
        nums[monkey] = r
        return r

    root1, _, root2 = monkeys["root"]
    del monkeys["root"]

    expr = factor(f(root1) - f(root2))

    print(solve(expr, symbols("n"))[0])


part1()
part2()
