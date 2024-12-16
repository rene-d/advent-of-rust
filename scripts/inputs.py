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
for year in range(2015, 2025):
    values = []

    min_inputs = float("inf")
    max_inputs = 0
    nb_inputs = 0

    for day in range(1, 26):
        inputs = Counter()

        for f in datadir.glob("*"):
            # if not f.stem.isdigit():
            #     continue
            if f.is_dir():
                f = f / f"{year}" / f"{day}.in"
                if f.is_file():
                    inputs.update([f.read_text().strip()])

        v = list(inputs.values())
        values.append((len(v), sum(v)))

        min_inputs = min(min_inputs, len(v))
        max_inputs = max(max_inputs, len(v))
        nb_inputs = max(nb_inputs, sum(v))

    row = [f"{year}"]
    for a, b in values:
        if a == min_inputs == max_inputs:
            a = f"\033[34m{a}{RESET}"
        elif a == min_inputs:
            a = f"{GREEN}{a}{RESET}"
        elif a == max_inputs:
            a = f"{MAGENTA}{a}{RESET}"
        b = f"{YELLOW}{b}{RESET}" if b == nb_inputs else f"{b}"
        row.append(f"{a} / {b}")

    row.append(f"{GREEN}{min_inputs:2}{RESET} → {MAGENTA}{max_inputs:2}{RESET}")
    row.append(f"{YELLOW}{nb_inputs:2}{RESET}")

    t.append(row)


# print(tabulate.tabulate(t, headers=["Year"] + [day for day in range(1, 26)], tablefmt="rounded_outline"))

t.insert(0, ["year"] + list(range(1, 26)) + [f"{GREEN}↓{MAGENTA}↑{RESET} ≠", f"{YELLOW}max{RESET}"])


t = transpose(t)
t.pop(0)
print(
    tabulate.tabulate(
        t,
        headers=["Day"] + [year for year in range(2015, 2025)],
        stralign="right",
        tablefmt="rounded_outline",
    )
)
