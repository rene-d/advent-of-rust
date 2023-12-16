#!/usr/bin/env python3
# https://adventofcode.com/2020/day/8

import sys
from pathlib import Path

verbose = "-v" in sys.argv
if verbose:
    sys.argv.remove("-v")
filename = ("test.txt" if sys.argv[1] == "-t" else sys.argv[1]) if len(sys.argv) > 1 else "input.txt"
data = Path(filename).read_text().strip()
lines = data.splitlines()


def run(program):
    acc = 0
    ip = 0
    visited = set()
    while ip < len(program) and ip not in visited:
        visited.add(ip)

        instr = program[ip]
        ip += 1

        op, imm = instr.split(maxsplit=1)
        if op == "nop":
            pass
        elif op == "acc":
            acc += int(imm)
        elif op == "jmp":
            ip += int(imm) - 1
        else:
            raise ValueError(instr)

    return acc, ip == len(program)


# part 1
acc, booted = run(lines)
assert booted is False
print(acc)


# part 2
for i in range(len(lines)):
    backup = lines[i]

    instr, imm = backup.split(maxsplit=1)
    if instr == "nop":
        lines[i] = f"jmp {imm}"
    elif instr == "jmp":
        lines[i] = f"nop {imm}"

    acc, booted = run(lines)
    if booted:
        break

    lines[i] = backup

print(acc)
