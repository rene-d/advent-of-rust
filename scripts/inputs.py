#!/usr/bin/env python3

from collections import Counter
from pathlib import Path

import tabulate

RED = "\033[91m"
GREEN = "\033[92m"
BLUE = "\033[94m"
DARK_GREEN = "\033[32m"
GRAY = "\033[37m"
MAGENTA = "\033[95m"
CYAN = "\033[96m"
WHITE = "\033[97m"
YELLOW = "\033[93m"
RESET = "\033[0m"
FEINT = "\033[2m"
ITALIC = "\033[3m"
BLINK = "\033[6m"
CLEAR_EOL = "\033[0K"


def transpose(m):
    rows = range(len(m))
    cols = range(len(m[0]))
    t = [[None for _ in rows] for _ in cols]

    for row in rows:
        for col in cols:
            t[col][row] = m[row][col]

    return t


datadir = Path(__file__).parent.parent / "data"

t = []
for year in range(2015, 2024):
    row = [f"{year}"]

    min_inputs = float("inf")
    max_inputs = 0
    nb_inputs = 0

    for day in range(1, 26):
        inputs = Counter()

        for f in datadir.glob("*"):
            if f.is_dir():
                f = f / f"{year}" / f"{day}.in"
                if f.is_file():
                    inputs.update([f.read_text().strip()])

        v = list(inputs.values())
        if len(v) == sum(v):
            # row.append(f"{len(v)} *")
            # row.append(f"{GREEN}{len(v):2} / {sum(v):2}{RESET}")
            row.append(f"{len(v)} / --")
        else:
            row.append(f"{len(v):2} / {sum(v):2}")
        # row.append(len(v))

        min_inputs = min(min_inputs, len(v))
        max_inputs = max(max_inputs, len(v))
        nb_inputs = max(nb_inputs, sum(v))

    row.append(f"{YELLOW}{min_inputs:2} → {max_inputs:2}{RESET}")
    row.append(f"{YELLOW}{nb_inputs:2}{RESET}")

    t.append(row)


# print(tabulate.tabulate(t, headers=["Year"] + [day for day in range(1, 26)], tablefmt="rounded_outline"))

t.insert(0, ["year"] + list(range(1, 26)) + [f"{YELLOW}↑↓{RESET}", f"{YELLOW}nb{RESET}"])
t = transpose(t)
t.pop(0)
print(
    tabulate.tabulate(
        t,
        headers=["Day"] + [year for year in range(2015, 2024)],
        stralign="right",
        tablefmt="rounded_outline",
    )
)
