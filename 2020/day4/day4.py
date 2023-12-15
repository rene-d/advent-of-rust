#!/usr/bin/env python3
# https://adventofcode.com/2020/day/4

import sys
from pathlib import Path

filename = ("test.txt" if sys.argv[1] == "-t" else sys.argv[1]) if len(sys.argv) > 1 else "input.txt"
data = Path(filename).read_text().strip()

passports = data.split("\n\n")

required = set(["eyr", "iyr", "byr", "ecl", "pid", "hcl", "hgt"])

valid = 0
for passport in passports:
    passport = passport.split()

    fields = list(map(lambda field: field.split(":")[0], passport))
    if len(required.difference(fields)) == 0:
        valid += 1

print(valid)


valid = 0
for passport in passports:
    fields = passport.split()

    ok = 0
    for field in fields:
        name, value = field.split(":", maxsplit=1)
        match name:
            case "byr":
                if len(value) == 4 and value.isdigit() and 1920 <= int(value) <= 2002:
                    ok |= 1 << 0

            case "iyr":
                if len(value) == 4 and value.isdigit() and 2010 <= int(value) <= 2020:
                    ok |= 1 << 1

            case "eyr":
                if len(value) == 4 and value.isdigit() and 2020 <= int(value) <= 2030:
                    ok |= 1 << 2

            case "hgt":
                if value.endswith("in"):
                    value = value.removesuffix("in")
                    if value.isdigit() and 59 <= int(value) <= 76:
                        ok |= 1 << 3

                elif value.endswith("cm"):
                    value = value.removesuffix("cm")
                    if value.isdigit() and 150 <= int(value) <= 193:
                        ok |= 1 << 3

            case "hcl":
                if len(value) == 7 and value[0] == "#":
                    for c in value[1:]:
                        if c not in "abcdef0123456789":
                            break
                    else:
                        ok |= 1 << 4

            case "ecl":
                if value in ("amb", "blu", "brn", "gry", "grn", "hzl", "oth"):
                    ok |= 1 << 5

            case "pid":
                if len(value) == 9 and value.isdigit():
                    ok |= 1 << 6

    if ok == 0b1111111:
        valid += 1

print(valid)
