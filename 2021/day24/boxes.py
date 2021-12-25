#!/usr/bin/env python3

a = open("input.txt").readlines()

for i in range(0, 252, 18):
    n = i // 18 + 1
    with open(f"box{n}.txt", "w") as f:
        f.write("".join(a[i : i + 18]))

for i in range(0, 18):
    print(f"{i:2}: " + "| ".join(a[i + j * 18].strip().ljust(10) for j in range(0, 14, 1)))
