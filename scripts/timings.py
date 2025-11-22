#!/usr/bin/env python3

import argparse
import os
import sqlite3
import sys
import time
import typing as t
from collections import defaultdict
from dataclasses import dataclass
from functools import lru_cache
from pathlib import Path

try:
    import curtsies
    import tabulate
except ImportError:
    print("This script requires the « tabulate » and « curtsies » modules.")

    if "VIRTUAL_ENV" in os.environ:
        print("VirtualEnv detected: try to install from PyPi...")
        if os.system(f"{sys.executable} -mpip install tabulate curtsies") != 0:
            sys.exit(1)
    elif sys.platform == "linux":
        distro = "unknown"
        r = Path("/etc/os-release")
        if r.is_file():
            for line in r.read_text(encoding="utf-8").splitlines():
                if line.startswith("ID="):
                    distro = line[3:].strip().strip('"').strip("'")
                    break
        if distro == "debian" or distro == "ubuntu":
            print("Debian/Ubuntu detected: try to install packages...")
            if os.system("sudo apt-get install -y python3-tabulate python3-curtsies") != 0:
                sys.exit(1)
        elif distro == "alpine":
            print("Alpine detected: try to install packages...")
            if os.system("sudo apk add --no-cache py3-tabulate") != 0:
                sys.exit(1)
        elif distro == "fedora":
            print("Fedora detected: try to install packages...")
            if os.system("sudo dnf install -y python3-tabulate python3-curtsies") != 0:
                sys.exit(1)
        else:
            sys.exit(0)
    else:
        sys.exit(1)

    import tabulate

    try:
        import curtsies
    except ImportError:
        curtsies = None


T1 = 0.5
T2 = 2
T3 = 5


@lru_cache(maxsize=None)
def aoc_available_puzzles(
    year: int | None = None, seconds: float | None = None
) -> t.Union[dict[int, list[int]], list[int]]:
    """
    Returns a dict of available puzzles by year or the list of available puzzles for the given year.
    """

    if year is not None:
        years = aoc_available_puzzles(seconds=seconds)
        return years.get(year, [])

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
        last_day = min(last_day, 25 if year <= 2024 else 12)

        puzzles[year] = list(range(1, last_day + 1))

    return puzzles


def fmt_elapsed(elapsed: float, _tablefmt) -> str:
    """Format elapsed time with color-coded ANSI escape sequences based on duration thresholds."""
    if elapsed < T1:
        return f"\033[32m{elapsed:.3f}\033[0m"
    elif elapsed < T2:
        return f"\033[34m{elapsed:.3f}\033[0m"
    elif elapsed < T3:
        return f"\033[33m{elapsed:.3f}\033[0m"
    else:
        return f"\033[31m{elapsed:.3f}\033[0m"


@dataclass
class Stats:
    """Container for timing statistics data including headers, table data, and solution timings."""

    headers: list
    data: dict
    solutions: dict


class Timings:
    """Manages and analyzes execution timing statistics for Advent of Code solutions."""

    def __init__(self, db: sqlite3.Connection):
        self.year_begin = min(aoc_available_puzzles())
        self.year_end = max(aoc_available_puzzles())

        self.user_inputs = defaultdict(set)
        for key_input, crc32 in db.execute("select key,crc32 from inputs"):
            year, day, user = key_input.split(":")
            year = int(year)
            day = int(day)
            self.user_inputs[crc32].add(user)

        self.solutions = defaultdict(lambda: defaultdict(dict))
        for key_solution, elapsed, status in db.execute("select key,elapsed,status from solutions"):
            if status == "ok":
                year, day, crc32, _binary, language = key_solution.split(":")
                year = int(year)
                day = int(day)
                elapsed /= 1_000_000_000

                # manage multiple solutions in different dayXX_xxx directories
                day_sols = self.solutions[year, day][language]
                other_elapsed = day_sols.get(crc32, float("inf"))
                day_sols[crc32] = min(elapsed, other_elapsed)

    def get_stats(self, user: str, lang: str, tablefmt: str) -> Stats:
        """
        Compute and return execution timing statistics for a given user and language.

        Args:
            user: User identifier or aggregation mode ('mean', 'min', 'max', 'minmax')
            lang: Programming language identifier
            tablefmt: Table format for output formatting

        Returns:
            Stats object containing headers, data, and solutions timing information.
        """
        stats = Stats(
            headers=["day"] + [i for i in range(self.year_begin, self.year_end + 1)],
            data=[[i] + [None] * (self.year_end - self.year_begin + 1) for i in range(1, 26)],
            solutions={},
        )

        for (year, day), languages in self.solutions.items():
            total_elapsed = 0
            min_elapsed = float("inf")
            max_elapsed = 0
            nb_elapsed = 0
            for key_hash, elapsed in languages[lang].items():
                if min_elapsed > elapsed:
                    min_elapsed = elapsed
                if max_elapsed < elapsed:
                    max_elapsed = elapsed
                if user in ("mean", "min", "max", "minmax") or user in self.user_inputs[key_hash]:
                    total_elapsed += elapsed
                    nb_elapsed += 1

            if nb_elapsed != 0:
                elapsed = total_elapsed / nb_elapsed

                if user == "min":
                    stats.solutions[year, day] = min_elapsed
                elif user == "max":
                    stats.solutions[year, day] = max_elapsed
                else:
                    stats.solutions[year, day] = elapsed

                if user == "minmax":
                    s = f"{fmt_elapsed(min_elapsed, tablefmt)} - {fmt_elapsed(max_elapsed, tablefmt)}"
                elif user == "min":
                    s = fmt_elapsed(min_elapsed, tablefmt)
                elif user == "max":
                    s = fmt_elapsed(max_elapsed, tablefmt)
                else:
                    s = fmt_elapsed(elapsed, tablefmt)

                stats.data[day - 1][year - self.year_begin + 1] = s

        return stats

    def print_stats(self, user: str, lang: str, tablefmt: str = "rounded_outline"):
        """Print timing statistics in a formatted table with performance breakdown."""
        stats = self.get_stats(user, lang, tablefmt)

        print(tabulate.tabulate(stats.data, stats.headers, tablefmt, floatfmt=".3f"))

        def timing(a, b):
            return sum(1 for _ in filter(lambda x: a <= x < b, stats.solutions.values()))

        def ids(a, b):
            return " ".join(f"{y}:{d:<2}" for (y, d), v in sorted(stats.solutions.items()) if a <= v < b)

        inf = float("inf")
        print()
        print(f"Solutions in \033[95m{lang.capitalize():<7}\033[0m : {len(stats.solutions):3}")
        print(f"Number missing       : {25 * (self.year_end - self.year_begin + 1) - len(stats.solutions):3}")
        print(f"Fast        (< {T1:3.1g}s) : \033[32m{timing(0, T1):3}\033[0m")
        print(f"Quite fast  (< {T2:3.1g}s) : \033[34m{timing(T1, T2):3}\033[0m")
        print(f"Fast enough (< {T3:3.1g}s) : \033[33m{timing(T2, T3):3}\033[0m   [ {ids(T2, T3)} ]")
        print(f"Slow        (≥ {T3:3.1g}s) : \033[31m{timing(T3, inf):3}\033[0m   [ {ids(T3, inf)} ]")
        print(f"Total                : {sum(stats.solutions.values()):7.3f} s")

        # import numpy as np

        # a = np.array(list(stats.solutions.values()))

        # # coefficient of variation
        # µ, σ = a.mean(), a.std()
        # cv = σ / µ
        # cv = round(cv * 100, 1)

        # # quartile coefficient of dispersion
        # for i in range(10,101,1):
        #     q1 = np.percentile(a, i)
        #     print(f"{i:3}  {q1:7.3f}s")


def main():
    """Main entry point for the timings script. Parses command-line arguments and displays timing statistics."""
    parser = argparse.ArgumentParser()
    parser.add_argument("-u", "--user", help="User ID")
    parser.add_argument("-l", "--lang", default="Rust", help="Language")
    parser.add_argument("-b", "--browse", action="store_true", help="Browse all users/languages")
    args = parser.parse_args()

    if "AOC_TARGET_DIR" in os.environ:
        cache_file = Path(os.environ["AOC_TARGET_DIR"]) / "cache.db"
    else:
        cache_file = Path(__file__).parent.parent / "data" / "cache.db"
    if not cache_file.is_file():
        print(f"Database {cache_file} does not exist.")
        exit(1)

    db = sqlite3.connect(cache_file)

    timings = Timings(db)

    if not args.user:
        row = db.execute("select key from inputs order by key limit 1").fetchone()
        if row:
            args.user = row[0].split(":")[2]

    if not args.user:
        parser.error("missing user")

    args.lang = args.lang.lower()
    args.user = args.user.rstrip("/")

    try:
        if not args.browse:
            timings.print_stats(args.user, args.lang)

        elif curtsies is None:
            print("Install the « curtsies » module.")

        else:
            sql = "select distinct key from inputs order by key"
            users = list(sorted(set(map(lambda row: row[0].split(":")[2], db.execute(sql)))))

            users.insert(0, "max")
            users.insert(0, "min")
            users.insert(0, "minmax")
            users.insert(0, "mean")

            current_user = 0

            languages = (
                "Rust",
                # "Python",
                "Py3.10",
                "Py3.11",
                "Py3.12",
                "Py3.13",
                # "Py3.13t",
                "Py3.14",
                # "Py3.14t",
                "Python",
                "C",
                "C++",
                "Go",
            )
            current_language = 0

            done = False

            while not done:
                print(end="\033[H\033[2J")

                print(f"User: {users[current_user]}")
                print()

                timings.print_stats(users[current_user], languages[current_language].casefold())

                print()
                print("← → : switch user     ↓ ↑ : switch language")

                with curtsies.Input(keynames="curses") as input_generator:
                    for e in input_generator:
                        if e in ("q", "Q", "x", "X", "\033"):
                            done = True
                        elif e == "KEY_LEFT":
                            if current_user > 0:
                                current_user = (current_user - 1) % len(users)
                        elif e == "KEY_RIGHT":
                            if current_user < len(users) - 1:
                                current_user = (current_user + 1) % len(users)
                        elif e == "KEY_UP":
                            if current_language > 0:
                                current_language = (current_language - 1) % len(languages)
                        elif e == "KEY_DOWN":
                            if current_language < len(languages) - 1:
                                current_language = (current_language + 1) % len(languages)
                        else:
                            print(f"unknown event: {repr(e)}")
                            continue
                        break

    except KeyboardInterrupt:
        pass

    db.close()


if __name__ == "__main__":
    main()
