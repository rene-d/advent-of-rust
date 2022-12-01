#!/usr/bin/env python3
# [Day 1: Calorie Counting](https://adventofcode.com/2022/day/1)

lines = open("input.txt").readlines()
lines.append("")

energy = 0
reeinders = []
for line in lines:
    line = line.strip()
    if not line:
        reeinders.append(energy)
        energy = 0
    else:
        energy += int(line)

# part one
print(max(reeinders))

# part two
reeinders = sorted(reeinders, reverse=True)
print(reeinders[0] + reeinders[1] + reeinders[2])
