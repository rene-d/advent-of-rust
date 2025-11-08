#!/usr/bin/env python3

# Day 18: Snailfish
# https://adventofcode.com/2021/day/18

import sys


class RegularNumber:
    """Represent a regular number."""

    def __init__(self, value):
        self.value = value

    def __eq__(self, other):
        if not isinstance(other, RegularNumber):
            return False
        return self.value == other.value


def to_snailfish(number):
    """
    Ensure the nested list is a list of RegularNumber elements.
    """
    if isinstance(number, int):
        return RegularNumber(number)
    if isinstance(number, RegularNumber):
        return RegularNumber(number.value)
    return (to_snailfish(number[0]), to_snailfish(number[1]))


def explode(number):
    """If any pair is nested inside four pairs, the leftmost such pair explodes."""

    def _flatten(length):
        return (length,) if isinstance(length, RegularNumber) else sum(map(_flatten, length), ())

    # flatten the nested list to get right and left numbers of number to explode
    # /!\ the RegularNumber are the same objects in the flattened and nested lists
    flat = _flatten(number)
    i_flat = 0
    exploded = False

    def _explode(number, depth):
        nonlocal i_flat, exploded
        # nonlocal: flat

        if exploded:
            # no more action to perform
            return number

        if isinstance(number, RegularNumber):
            # since we traverse the nested list left to right,
            # each time we find a regular number
            # we increment the index of the flat list
            i_flat += 1
            return number

        left, right = number

        if isinstance(left, RegularNumber) and isinstance(right, RegularNumber):
            # If any pair of regular numbers is nested inside four pairs, the leftmost such pair explodes
            if depth >= 4 and not exploded:
                # the pair's left value is added to the first regular number to the left of the exploding pair (if any)
                if i_flat > 0:
                    flat[i_flat - 1].value += flat[i_flat].value

                # the pair's right value is added to the first regular number
                # to the right of the exploding pair (if any)
                if i_flat + 1 < len(flat) - 1:
                    flat[i_flat + 2].value += flat[i_flat + 1].value

                exploded = True  # just one explode operation per turn
                return RegularNumber(0)

        return (_explode(left, depth + 1), _explode(right, depth + 1))

    return _explode(number, 0)


def split(number):
    """If any regular number is 10 or greater, the leftmost such regular number splits."""

    splitted = False

    def _split(number):
        nonlocal splitted

        if splitted:
            # one split operation per turn
            return number

        if isinstance(number, RegularNumber):
            if number.value >= 10 and not splitted:
                # split the number and terminate the recursion
                splitted = True
                return (RegularNumber(number.value // 2), RegularNumber(number.value - number.value // 2))
            return number

        left, right = number

        return (_split(left), _split(right))

    return _split(number)


def addition(a, b):
    """Basic addition, not reduced."""
    return (a, b)


def reduced_addition(a, b):
    """
    Perform addition, then reduce the snailfish number.
    To reduce a snailfish number, you must repeatedly explode/reduce until no action applies.
    """

    # create new objects to avoid modifying the original ones
    new_a = to_snailfish(a)
    new_b = to_snailfish(b)

    result = addition(new_a, new_b)

    while True:
        new = explode(result)
        if result != new:
            result = new
            continue
        new = split(result)
        if result != new:
            result = new
            continue

        return result


def magnitude(number):
    """
    The magnitude of a pair is 3 times the magnitude of its left element
    plus 2 times the magnitude of its right element.
    """

    if isinstance(number, RegularNumber):
        return number.value
    left, right = number
    return 3 * magnitude(left) + 2 * magnitude(right)


def main():
    """Solve the puzzle."""

    data = open("input.txt" if len(sys.argv) == 1 else sys.argv[1]).read().splitlines()
    numbers = [to_snailfish(eval(line)) for line in data]

    # part 1
    total = numbers[0]
    for number in numbers[1:]:
        total = reduced_addition(total, number)
    print(magnitude(total))

    # part 2
    print(max(magnitude(reduced_addition(a, b)) for a in numbers for b in numbers if a != b))


if __name__ == "__main__":
    main()
