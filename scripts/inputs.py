#!/usr/bin/env python3

from collections import Counter
from pathlib import Path

import tabulate

datadir = Path(__file__).parent.parent / "data"

t = []
for year in range(2015, 2024):
    row = [f"{year}"]
    for day in range(1, 26):
        inputs = Counter()

        for f in datadir.glob("*"):
            if f.is_dir():
                f = f / f"{year}" / f"{day}.in"
                if f.is_file():
                    inputs.update([f.read_text().strip()])

        v = list(inputs.values())
        if len(v) == sum(v):
            row.append(f"{len(v)} *")
        else:
            row.append(f"{len(v)}/{sum(v)}")
        # row.append(len(v))

    t.append(row)

print(tabulate.tabulate(t, headers=["Year"] + [day for day in range(1, 26)], tablefmt="rounded_outline"))
