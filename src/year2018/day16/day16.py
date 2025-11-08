#!/usr/bin/env python3
# [Day 16: Chronal Classification](https://adventofcode.com/2018/day/16)

import atexit
import sys
import time
import typing as t
from collections import defaultdict
from copy import deepcopy
from pathlib import Path

verbose = "-v" in sys.argv
if verbose:
    sys.argv.remove("-v")
self_tests = "-T" in sys.argv
if self_tests:
    sys.argv.remove("-T")
filename = ("test.txt" if sys.argv[1] == "-t" else sys.argv[1]) if len(sys.argv) > 1 else "input.txt"
data = Path(filename).read_text().strip()
if "--elapsed" in sys.argv:
    sys.argv.remove("--elapsed")
    start_time_ns = time.time_ns()
    atexit.register(lambda: print(f"elapsed: {(time.time_ns() - start_time_ns) / 1_000_000}ms"))


OPCODES = (
    "addi",
    "addr",
    "bani",
    "banr",
    "bori",
    "borr",
    "eqir",
    "eqri",
    "eqrr",
    "gtir",
    "gtri",
    "gtrr",
    "muli",
    "mulr",
    "seti",
    "setr",
)


def emulate(opcode, a, b, c, regs: t.List) -> t.List:
    regs = deepcopy(regs)
    match opcode:
        # Addition
        case "addr":
            regs[c] = regs[a] + regs[b]
        case "addi":
            regs[c] = regs[a] + b

        # Multiplication
        case "mulr":
            regs[c] = regs[a] * regs[b]
        case "muli":
            regs[c] = regs[a] * b

        # Bitwise AND
        case "banr":
            regs[c] = regs[a] & regs[b]
        case "bani":
            regs[c] = regs[a] & b

        # Bitwise OR
        case "borr":
            regs[c] = regs[a] | regs[b]
        case "bori":
            regs[c] = regs[a] | b

        # Assignment
        case "setr":
            regs[c] = regs[a]
        case "seti":
            regs[c] = a

        # Greater-than testing
        case "gtir":
            regs[c] = 1 if a > regs[b] else 0
        case "gtri":
            regs[c] = 1 if regs[a] > b else 0
        case "gtrr":
            regs[c] = 1 if regs[a] > regs[b] else 0

        # Equality testing
        case "eqir":
            regs[c] = 1 if a == regs[b] else 0
        case "eqri":
            regs[c] = 1 if regs[a] == b else 0
        case "eqrr":
            regs[c] = 1 if regs[a] == regs[b] else 0

        case _:
            raise ValueError(f"bad opcode {opcode}")

    return regs


samples, program = data.split("\n\n\n\n", maxsplit=1)


mapping = defaultdict(set)

result = 0
for sample in samples.split("\n\n"):
    before, instruction, after = sample.splitlines()

    before = list(map(int, before.removeprefix("Before: [").removesuffix("]").split(",")))
    opcode, a, b, c = map(int, instruction.split())
    after = list(map(int, after.removeprefix("After:  [").removesuffix("]").split(",")))

    n = 0
    for o in OPCODES:
        if after == emulate(o, a, b, c, before):
            n += 1
            mapping[opcode].add(o)
    if n >= 3:
        result += 1

# part 1
print(result)

# resolve the mapping
opcodes = dict()
while mapping:
    for k, v in mapping.items():
        if len(v) == 1:
            name = v.pop()
            mapping.pop(k)
            opcodes[k] = name
            for _, v in mapping.items():
                v.discard(name)
            break
    else:
        raise ValueError

# run the test program
regs = [0, 0, 0, 0]
for instruction in program.splitlines():
    opcode, a, b, c = map(int, instruction.split())
    regs = emulate(opcodes[opcode], a, b, c, regs)

# part 2
print(regs[0])
