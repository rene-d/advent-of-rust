#!/usr/bin/env python3
# [Day 18: Snailfish](https://adventofcode.com/2021/day/18)

import functools
import itertools
import typing as t
from argparse import ArgumentParser
from pathlib import Path


class Snailfish:
    DEPTH = 5  # to handle addition before reduction
    MAX_SIZE = 2**DEPTH

    def __init__(self, value: t.Optional[str] = None):
        self.v = [None] * Snailfish.MAX_SIZE

        if isinstance(value, str):
            depth = 0
            i = 0
            k = 0
            while k < len(value):
                c = value[k]
                k += 1
                if c == "[":
                    depth += 1
                    assert depth <= Snailfish.DEPTH
                elif c == "]":
                    depth -= 1
                elif c.isdigit():
                    self.v[i] = int(c)
                    while value[k].isdigit():
                        self.v[i] = self.v[i] * 10 + int(value[k])
                        k += 1
                    i += 2 ** (Snailfish.DEPTH - depth)

            assert depth == 0

    def __str__(self):
        def fmt(v: t.List[t.Any]):
            if len(v) == 1 or v[len(v) // 2] is None:
                return str(v[0])
            else:
                left = fmt(v[: len(v) // 2])
                right = fmt(v[len(v) // 2 :])
                return f"[{left},{right}]"

        return fmt(self.v)

    def dump(self, idx: int = None):
        def gen():
            for k, v in enumerate(self.v):
                if k == idx:
                    yield "\033[32m"
                yield "⋅" if v is None else str(v)
                if k == idx:
                    yield "\033[0m"

        print(f"[{' '.join(gen())}]")

    def __add__(self, rhs):
        r = self.add(rhs)
        r.reduce()
        return r

    def __eq__(self, value):
        return self.v == value.v

    def add(self, rhs):
        if not isinstance(rhs, Snailfish):
            rhs = Snailfish(rhs)
        if all(num is None for num in self.v):
            return rhs
        r = Snailfish()
        r.v = self.v[0::2] + rhs.v[0::2]
        return r

    def reduce(self):
        while self.explode() or self.split():
            pass

    def explode(self):
        for i in range(0, Snailfish.MAX_SIZE, 2):

            # two consecutive numbers form a pair and this pair is necessarily «nested inside four pairs»
            if self.v[i] is not None and self.v[i + 1] is not None:

                # «the pair's left value is added to the first regular number to the left of the exploding pair (if any)»
                k = i
                while k > 0 and self.v[k - 1] is None:
                    k -= 1
                if k > 0:
                    self.v[k - 1] += self.v[i]

                # «"the pair's right value is added to the first regular number to the right of the exploding pair (if any)»
                k = i + 1
                while k < Snailfish.MAX_SIZE - 1 and self.v[k + 1] is None:
                    k += 1
                if k < 31:
                    self.v[k + 1] += self.v[i + 1]

                # «exploding pair is replaced with the regular number 0»
                self.v[i] = 0
                self.v[i + 1] = None

                return True

        return False

    def split(self):
        for i in range(0, Snailfish.MAX_SIZE, 2):

            # «If any regular number is 10 or greater, the leftmost such regular number splits»
            if self.v[i] is not None and self.v[i] >= 10:

                # «the left element of the pair should be the regular number divided by two and rounded down»
                num = self.v[i]
                self.v[i] = num // 2

                # «the right element of the pair should be the regular number divided by two and rounded up»
                # to "insert" a pair, we have to find an empty slot on the right
                # the right slots for a pair depend on the left one:
                #   slot 0:     1 2 4 8 16
                # i.e.  [a,b]               has slots {0:a,16:b}
                #       [[a,b],c]           has slots {0:a,8:b,16:c}
                #       [[[a,b],c],d]       has slots {0:a,4:b,8:c,16:d}
                #       [[[[a,b],c],d],e]   has slots {0:a,2:b,4:c,4:d,8:d,16:e}
                #   slot 2:     3
                # i.e. a number at slot 2 is left side of a pair nested 4 times:
                # so the only possible slot is 3 (in a 5-depth snailfish number)
                # etc.
                k = 1
                ii = i
                while ii % 2 == 0 and k < Snailfish.DEPTH:
                    k *= 2
                    ii /= 2
                while k > 0:
                    j = i + k
                    if j < Snailfish.MAX_SIZE and self.v[j] is None:
                        self.v[j] = (num + 1) // 2
                        return True
                    k //= 2

                assert False
        return False

    def magnitude(self):
        def mag(v):
            if len(v) == 1 or v[len(v) // 2] is None:
                return v[0]
            else:
                left = mag(v[: len(v) // 2])
                right = mag(v[len(v) // 2 :])
                return 3 * left + 2 * right

        return mag(self.v)


def solve(data: str):
    numbers = list(map(Snailfish, data.splitlines()))
    sum_part1 = functools.reduce(lambda a, b: a + b, numbers)
    part1 = sum_part1.magnitude()
    part2 = max((a + b).magnitude() for a, b in itertools.permutations(numbers, 2))
    return sum_part1, part1, part2


def test():
    def assert_eq(a: Snailfish, s: str):
        assert str(a) == s
        assert a == Snailfish(s)

    ########################################################

    def assert_explode(phrase: str):
        before, after = phrase.split(" becomes ")
        a = Snailfish(before)
        assert_eq(a, before)
        a.explode()
        assert_eq(a, after)

    assert_explode("[[[[[9,8],1],2],3],4] becomes [[[[0,9],2],3],4]")
    assert_explode("[7,[6,[5,[4,[3,2]]]]] becomes [7,[6,[5,[7,0]]]]")
    assert_explode("[[6,[5,[4,[3,2]]]],1] becomes [[6,[5,[7,0]]],3]")
    assert_explode("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]] becomes [[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]")
    assert_explode("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]] becomes [[3,[2,[8,0]]],[9,[5,[7,0]]]]")

    ########################################################

    a = Snailfish("[[[[4,3],4],4],[7,[[8,4],9]]]")
    b = Snailfish("[1,1]")

    c = a.add(b)
    assert_eq(c, "[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]")
    c.explode()
    assert_eq(c, "[[[[0,7],4],[7,[[8,4],9]]],[1,1]]")
    c.explode()
    assert_eq(c, "[[[[0,7],4],[15,[0,13]]],[1,1]]")
    c.split()
    assert_eq(c, "[[[[0,7],4],[[7,8],[0,13]]],[1,1]]")
    c.split()
    assert_eq(c, "[[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]")
    c.explode()
    assert_eq(c, "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]")

    ########################################################

    a = Snailfish()
    a += "[1,1]"
    a += "[2,2]"
    a += "[3,3]"
    a += "[4,4]"
    assert_eq(a, "[[[[1,1],[2,2]],[3,3]],[4,4]]")

    a += "[5,5]"
    assert_eq(a, "[[[[3,0],[5,3]],[4,4]],[5,5]]")

    a += "[6,6]"
    assert_eq(a, "[[[[5,0],[7,4]],[5,5]],[6,6]]")

    ########################################################

    additions = Path("sample_7.txt").read_text().split("\n\n")
    for addition in additions:
        a, b, r = addition.splitlines()
        a = a.strip()
        b = b.removeprefix("+").strip()
        r = r.removeprefix("=").strip()
        assert_eq(Snailfish(a) + Snailfish(b), r)

    ########################################################

    def assert_mag(phrase: str):
        a, value = phrase.split(" becomes ")
        assert Snailfish(a).magnitude() == int(value)

    assert Snailfish("[9,1]").magnitude() == 29
    assert Snailfish("[[9,1],[1,9]]").magnitude() == 129
    assert_mag("[[1,2],[[3,4],5]] becomes 143")
    assert_mag("[[[[0,7],4],[[7,8],[6,0]]],[8,1]] becomes 1384")
    assert_mag("[[[[1,1],[2,2]],[3,3]],[4,4]] becomes 445")
    assert_mag("[[[[3,0],[5,3]],[4,4]],[5,5]] becomes 791")
    assert_mag("[[[[5,0],[7,4]],[5,5]],[6,6]] becomes 1137")
    assert_mag("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]] becomes 3488")

    ########################################################

    sum, mag, mag_max = solve(Path("sample_8.txt").read_text())
    assert_eq(sum, "[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]")
    assert mag == 4140
    assert mag_max == 3993

    a = Snailfish("[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]")
    b = Snailfish("[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]")
    c = a + b
    assert_eq(c, "[[[[7,8],[6,6]],[[6,0],[7,7]]],[[[7,8],[8,8]],[[7,9],[0,6]]]]")
    assert c.magnitude() == 3993

    print("ok")


def main():
    parser = ArgumentParser()
    parser.add_argument("-v", "--verbose", action="store_true")
    parser.add_argument("-t", "--test", action="store_true")
    parser.add_argument("--elapsed", action="store_true")
    parser.add_argument("filename", nargs="?", type=Path, default="input.txt")
    args = parser.parse_args()

    if args.test:
        return test()

    data = args.filename.read_text().strip()
    _, part1, part2 = solve(data)
    print(part1)
    print(part2)


if __name__ == "__main__":
    main()
