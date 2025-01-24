#!/usr/bin/env python3
# [Day 8: I Heard You Like Registers](https://adventofcode.com/2017/day/8)

import re
import sys
from collections import defaultdict
from pathlib import Path

filename = ("test.txt" if sys.argv[1] == "-t" else sys.argv[1]) if len(sys.argv) > 1 else "input.txt"
data = Path(filename).read_text().strip()
lines = data.splitlines()

registers = defaultdict(lambda: 0)

part2 = 0

for line in lines:
    m = re.match(
        r"^(?P<target>\w+) (?P<op>(inc|dec)) (?P<value>-?\d+)"
        r" if (?P<register>\w+) (?P<condition>(==|<=|<|!=|>=|>)) (?P<compare>-?\d+)",
        line,
    )

    compare_register = registers[m["register"]]
    compare_value = int(m["compare"])

    match m["condition"]:
        case "==":
            result = compare_register == compare_value
        case "!=":
            result = compare_register != compare_value
        case ">":
            result = compare_register > compare_value
        case ">=":
            result = compare_register >= compare_value
        case "<":
            result = compare_register < compare_value
        case "<=":
            result = compare_register <= compare_value
        case _:
            raise ValueError

    if result:
        match m["op"]:
            case "inc":
                registers[m["target"]] += int(m["value"])
            case "dec":
                registers[m["target"]] -= int(m["value"])
            case _:
                raise ValueError

    part1 = max(registers.values())
    part2 = max(part2, part1)

print(part1)
print(part2)
