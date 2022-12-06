#!/usr/bin/env python3

data = open("input.txt").read()

for length in (4, 14):
    for i in range(len(data) - length):
        if len(set(data[i : i + length])) == length:
            print(i + length)
            break
