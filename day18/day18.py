#!/usr/bin/env python3

# Day 18: Snailfish
# https://adventofcode.com/2021/day/18

import sys


class Node:
    def __init__(self, value):
        self.value = value

    def __eq__(self, other):
        if not isinstance(other, Node):
            return False
        return self.value == other.value


def to_nodes(number):
    if isinstance(number, int):
        return Node(number)
    if isinstance(number, Node):
        return Node(number.value)
    return (to_nodes(number[0]), to_nodes(number[1]))


def addition(a, b):
    return (a, b)


def explode(number):
    flatten = lambda l: sum(map(flatten, l), ()) if isinstance(l, tuple) else (l,)
    flat = flatten(number)

    i = 0
    exploded = False

    def _explode(number, depth):
        nonlocal i, flat, exploded

        if isinstance(number, Node):
            i += 1
            return number

        left, right = number
        if isinstance(left, Node) and isinstance(right, Node):

            # If any pair is nested inside four pairs, the leftmost such pair explodes
            if depth >= 4 and not exploded:

                # the pair's left value is added to the first regular number to the left of the exploding pair (if any)
                if i > 0:
                    flat[i - 1].value += flat[i].value

                # the pair's right value is added to the first regular number to the right of the exploding pair (if any)
                if i + 1 < len(flat) - 1:
                    flat[i + 2].value += flat[i + 1].value

                exploded = True
                return Node(0)

        return (_explode(left, depth + 1), _explode(right, depth + 1))

    return _explode(number, 0)


def split(number):
    def _split(a, splitted):
        if isinstance(a, Node):
            if a.value >= 10 and not splitted:
                return (Node(a.value // 2), Node(a.value - a.value // 2)), True
            return a, splitted

        left, splitted = _split(a[0], splitted)
        right, splitted = _split(a[1], splitted)

        return (left, right), splitted

    return _split(number, False)[0]


def reduced_addition(a, b):

    # create new objects to avoid modifying the original ones
    a = to_nodes(a)
    b = to_nodes(b)

    result = addition(a, b)

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
    if isinstance(number, Node):
        return number.value
    left, right = number
    return 3 * magnitude(left) + 2 * magnitude(right)


def main():
    data = open("input.txt" if len(sys.argv) == 1 else sys.argv[1]).read().splitlines()
    numbers = [to_nodes(eval(line)) for line in data]

    # part 1
    total = numbers[0]
    for number in numbers[1:]:
        total = reduced_addition(total, number)
    print(magnitude(total))

    # part 2
    print(max(magnitude(reduced_addition(a, b)) for a in numbers for b in numbers if a != b))


if __name__ == "__main__":
    main()
