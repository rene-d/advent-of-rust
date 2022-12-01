#!/usr/bin/env python3

a = [sum(map(int, i.split())) for i in open("input.txt").read().split("\n\n")]
print("part1:", max(a))
print("part2:", sum(sorted(a)[-3:]))
