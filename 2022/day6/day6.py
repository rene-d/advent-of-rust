#!/usr/bin/env python3

data = open("input.txt").read()
print(list(min(i + length for i in range(len(data) - length) if len(set(data[i : i + length])) == length) for length in (4, 14)))
