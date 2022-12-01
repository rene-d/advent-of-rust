#!/usr/bin/env python3

list(filter(lambda a: print(max(a), sum(sorted(a)[-3:])), ([sum(map(int, i.split())) for i in open("input.txt").read().split("\n\n")],)))
