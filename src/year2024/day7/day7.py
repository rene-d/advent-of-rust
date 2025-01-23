#!/usr/bin/env python3
# [Day 7: Bridge Repair](https://adventofcode.com/2024/day/7)

import sys
from pathlib import Path

verbose = "-v" in sys.argv
if verbose:
    sys.argv.remove("-v")
self_tests = "-T" in sys.argv
if self_tests:
    sys.argv.remove("-T")
filename = ("test.txt" if sys.argv[1] == "-t" else sys.argv[1]) if len(sys.argv) > 1 else "input.txt"
data = Path(filename).read_text().strip()
lines = data.splitlines()


def check_equation_two_operators(test_value, values):

    for i in range(2 ** (len(values) - 1)):

        result = values[0]
        for value in values[1:]:
            i, r = divmod(i, 2)
            if r == 0:
                result += value
            else:
                result *= value

            if result > test_value:
                break

        if result == test_value:
            return True

    return False


def check_equation_three_operators(test_value, values):

    p10 = []
    for value in values[1:]:
        p = 1
        while value != 0:
            p *= 10
            value //= 10
        p10.append(p)

    for i in range(3 ** (len(values) - 1)):
        result = values[0]
        for k, value in enumerate(values[1:]):
            i, r = divmod(i, 3)
            if r == 0:
                result += value
            elif r == 1:
                result *= value
            else:
                result = result * p10[k] + value

            if result > test_value:
                break

        if result == test_value:
            return True

    return False


calibration = 0
for line in lines:
    test_value, values = line.split(":")
    values = list(map(int, values.split()))
    test_value = int(test_value)
    if check_equation_two_operators(test_value, values):
        calibration += test_value
print(calibration)


calibration = 0
for line in lines:
    test_value, values = line.split(":")
    values = list(map(int, values.split()))
    test_value = int(test_value)
    if check_equation_three_operators(test_value, values):
        calibration += test_value
print(calibration)
