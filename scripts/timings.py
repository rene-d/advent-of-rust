#!/usr/bin/env python3

import argparse
import sqlite3
from collections import defaultdict
from dataclasses import dataclass
from pathlib import Path

import tabulate
from curtsies import Input

YEAR_BEGIN = 2015
YEAR_END = 2024

T1 = 0.5
T2 = 2
T3 = 5


def fmt_elapsed(elapsed: float, tablefmt) -> str:
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
    headers: list
    data: dict
    solutions: dict


class Timings:

    def __init__(self, db: sqlite3.Connection):

        self.user_inputs = defaultdict(set)
        for key_input, hash in db.execute("select key,crc32 from inputs"):
            year, day, user = key_input.split(":")
            year = int(year)
            day = int(day)
            self.user_inputs[hash].add(user)

        self.solutions = defaultdict(lambda: defaultdict(dict))
        for key_solution, elapsed, status in db.execute("select key,elapsed,status from solutions"):
            if status == "ok":
                year, day, hash, binary, language = key_solution.split(":")
                year = int(year)
                day = int(day)
                elapsed /= 1_000_000_000

                # manage multiple solutions in different dayXX_xxx directories
                day_sols = self.solutions[year, day][language]
                other_elapsed = day_sols.get(hash, float("inf"))
                day_sols[hash] = min(elapsed, other_elapsed)

    def get_stats(self, user: str, lang: str, tablefmt: str) -> Stats:

        stats = Stats(
            headers=["day"] + [i for i in range(YEAR_BEGIN, YEAR_END + 1)],
            data=[[i] + [None] * (YEAR_END - YEAR_BEGIN + 1) for i in range(1, 26)],
            solutions={},
        )

        for (year, day), languages in self.solutions.items():

            total_elapsed = 0
            min_elapsed = float("inf")
            max_elapsed = 0
            nb_elapsed = 0
            for hash, elapsed in languages[lang].items():
                if min_elapsed > elapsed:
                    min_elapsed = elapsed
                if max_elapsed < elapsed:
                    max_elapsed = elapsed
                if user in ("mean", "min", "max", "minmax") or user in self.user_inputs[hash]:
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

                stats.data[day - 1][year - YEAR_BEGIN + 1] = s

        return stats

    def print_stats(self, user: str, lang: str, tablefmt: str = "rounded_outline"):

        stats = self.get_stats(user, lang, tablefmt)

        print(tabulate.tabulate(stats.data, stats.headers, tablefmt, floatfmt=".3f"))

        # Python doesn't like closures. I do.
        timing = lambda a, b: sum(1 for _ in filter(lambda x: a <= x < b, stats.solutions.values()))
        ids = lambda a, b: " ".join(f"{y}:{d:<2}" for (y, d), v in sorted(stats.solutions.items()) if a <= v < b)
        inf = float("inf")
        print()
        print(f"Solutions in \033[95m{lang.capitalize():<7}\033[0m : {len(stats.solutions):3}")
        print(f"Number missing       : {25 * (YEAR_END - YEAR_BEGIN + 1) - len(stats.solutions):3}")
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

    parser = argparse.ArgumentParser()
    parser.add_argument("-u", "--user", help="User ID")
    parser.add_argument("-l", "--lang", default="Rust", help="Language")
    parser.add_argument("-b", "--browse", action="store_true", help="Browse all users/languages")
    args = parser.parse_args()

    db = sqlite3.connect(Path(__file__).parent.parent / "data" / "cache.db")

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

        else:
            sql = "select distinct key from inputs order by key"
            users = list(sorted(set(map(lambda row: row[0].split(":")[2], db.execute(sql)))))

            users.insert(0, "max")
            users.insert(0, "min")
            users.insert(0, "minmax")
            users.insert(0, "mean")

            current_user = 0

            languages = ("Rust", "Python", "Py3.11", "Py3.12", "Py3.13", "Py3.14", "C", "C++", "Go")
            current_language = 0

            done = False

            while not done:
                print(end="\033[H\033[2J")

                print(f"User: {users[current_user]}")
                print()

                timings.print_stats(users[current_user], languages[current_language].casefold())

                print()
                print("← → : switch user     ↓ ↑ : switch language")

                with Input(keynames="curses") as input_generator:
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
