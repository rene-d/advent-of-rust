#!/usr/bin/env python3

import argparse
import time
from collections import Counter
from functools import lru_cache
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


# ---------


@lru_cache(maxsize=None)
def aoc_available_puzzles_dict(seconds: float | None = None) -> dict[int, list[int]]:
    """
    Returns a dict of available puzzles by year.
    """

    now = time.gmtime(seconds)

    # available years
    first_year = 2015
    if now.tm_mon <= 11 or (now.tm_mday == 1 and now.tm_hour < 5):
        last_year = now.tm_year - 1
    else:
        last_year = now.tm_year

    puzzles = dict()
    for year in range(first_year, last_year + 1):
        # available puzzles in year
        if year == now.tm_year:
            last_day = now.tm_mday - 1 if now.tm_hour < 5 else now.tm_mday
        else:
            last_day = 25
        last_day = min(last_day, aoc_puzzles_by_year(year))

        puzzles[year] = list(range(1, last_day + 1))

    return puzzles


def aoc_puzzles_by_year(year: int) -> int:
    """
    Returns the maximum number of puzzles for the given year.
    """
    if 2015 <= year <= 2024:
        return 25
    if year >= 2025:
        return 12
    return 0


def aoc_nb_answers(year: int, day: int) -> int:
    """
    Returns the number of answers.
    """
    if 1 <= day < aoc_puzzles_by_year(year):
        return 2
    elif day == aoc_puzzles_by_year(year):
        return 1
    else:
        return 0


def aoc_available_years():
    """
    Generator over all available years.
    """
    for year in aoc_available_puzzles_dict():
        yield year


def aoc_available_days(year: int):
    """
    Generator over all available days for the given year.
    """
    for day in aoc_available_puzzles_dict().get(year, []):
        yield day


def aoc_available_puzzles(filter_year: int = 0):
    for year, days in aoc_available_puzzles_dict().items():
        if filter_year is not None and filter_year != 0 and filter_year != year:
            continue
        for day in days:
            yield year, day


# ---------


def transpose(m):
    rows = range(len(m))
    cols = range(len(m[0]))
    t = [[None for _ in rows] for _ in cols]

    for row in rows:
        for col in cols:
            t[col][row] = m[row][col]

    return t


parser = argparse.ArgumentParser()
parser.add_argument("--ok", action="store_true", help="Only inputs with solution")
args = parser.parse_args()


datadir = Path(__file__).parent.parent / "data"

data = []
for year in aoc_available_years():
    values = []

    min_inputs = float("inf")
    max_inputs = 0
    nb_inputs = 0

    available_days = aoc_available_days(year)

    for day in range(1, 26):
        if day > aoc_puzzles_by_year(year):
            values.append((None, "-"))
            continue
        if day not in available_days:
            values.append((None, "⏰"))
            continue

        inputs = Counter()

        for f in datadir.glob("*"):
            # if not f.stem.isdigit():
            #     continue
            if f.is_dir():
                f = f / f"{year}" / f"{day}.in"

                if args.ok and not f.with_suffix(".ok").is_file():
                    continue

                if f.is_file():
                    inputs.update([f.read_text().strip()])

        v = list(inputs.values())
        values.append((len(v), sum(v)))

        if len(v) != 0:
            min_inputs = min(min_inputs, len(v))
        max_inputs = max(max_inputs, len(v))
        nb_inputs = max(nb_inputs, sum(v))

    row = [f"{year}"]
    for a, b in values:
        if a is None:
            row.append(b)
            continue

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

    data.append(row)


# print(tabulate.tabulate(data, headers=["Year"] + [day for day in range(1, 26)], tablefmt="rounded_outline"))

data.insert(0, ["year"] + list(range(1, 26)) + [f"{GREEN}↓{MAGENTA} ↑{RESET} ≠", f"{YELLOW}max{RESET}"])


data = transpose(data)
data.pop(0)
print(
    tabulate.tabulate(
        data,
        headers=["Day"] + [year for year in aoc_available_years()],
        stralign="right",
        tablefmt="rounded_outline",
    )
)


# SQL request to find out how many unique inputs each user has
sql = """
WITH uniques AS (
    SELECT MIN(user) AS user
    FROM inputs
    WHERE user REGEXP '^[0-9]+$'
    GROUP BY year,day,crc HAVING COUNT(crc)=1
)
SELECT user,COUNT(*)
FROM uniques GROUP BY user;
"""
