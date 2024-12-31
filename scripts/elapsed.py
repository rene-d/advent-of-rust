#!/usr/bin/env python3

import argparse
import sqlite3
from dataclasses import dataclass
from pathlib import Path

import tabulate
from curtsies import Input

YEAR_BEGIN = 2015
YEAR_END = 2024

T1 = 0.5
T2 = 2
T3 = 5


@dataclass
class Stats:
    headers: list
    data: dict
    solutions: dict


cache_data = {}


def get_stats(db: sqlite3.Connection, user: str, lang: str) -> Stats:

    cache = cache_data.get((user, lang))

    if cache:
        return cache

    cache = Stats(
        headers=["day"] + [i for i in range(YEAR_BEGIN, YEAR_END + 1)],
        data=[[i] + [None] * (YEAR_END - YEAR_BEGIN + 1) for i in range(1, 26)],
        solutions={},
    )
    cache_data[(user, lang)] = cache

    for key_input, crc32 in db.execute("select key,crc32 from inputs where key like ?", (f"%:{user}",)):
        key_solution = ":".join(key_input.split(":")[0:2] + [crc32, "%"])
        for key, elapsed, status in db.execute(
            "select key,elapsed,status from solutions where key like ?", (key_solution,)
        ):
            year, day, _, _, language = key.split(":")

            year = int(year)
            day = int(day)
            elapsed /= 1_000_000_000

            if language != lang or status != "ok":
                continue

            cache.solutions[year, day] = elapsed

            if elapsed < T1:
                elapsed = f"\033[32m{elapsed:.3f}\033[0m"
            elif elapsed < T2:
                elapsed = f"\033[34m{elapsed:.3f}\033[0m"
            elif elapsed < T3:
                elapsed = f"\033[33m{elapsed:.3f}\033[0m"
            else:
                elapsed = f"\033[31m{elapsed:.3f}\033[0m"

            cache.data[day - 1][year - YEAR_BEGIN + 1] = elapsed

    return cache


def print_stats(db: sqlite3.Connection, user: str, lang: str):

    cache = get_stats(db, user, lang)

    print(tabulate.tabulate(cache.data, cache.headers, tablefmt="rounded_outline", floatfmt=".3f"))

    timing = lambda a, b: sum(1 for _ in filter(lambda x: a <= x < b, cache.solutions.values()))
    ids = lambda a, b: " ".join(f"{y}:{d:<2}" for (y, d), v in sorted(cache.solutions.items()) if a <= v < b)
    inf = float("inf")
    print()
    print(f"Solutions in {lang.capitalize():<7} : {len(cache.solutions):3}")
    print(f"Number missing       : {25 * (YEAR_END - YEAR_BEGIN + 1) - len(cache.solutions):3}")
    print(f"Fast        (< {T1:3.1g}s) : \033[32m{timing(0, T1):3}\033[0m")
    print(f"Quite fast  (< {T2:3.1g}s) : \033[34m{timing(T1, T2):3}\033[0m")
    print(f"Fast enough (< {T3:3.1g}s) : \033[33m{timing(T2, T3):3}\033[0m   [ {ids(T2, T3)} ]")
    print(f"Slow        (≥ {T3:3.1g}s) : \033[31m{timing(T3, inf):3}\033[0m   [ {ids(T3, inf)} ]")
    print(f"Total                : {sum(cache.solutions.values()):7.3f} s")


def main():

    parser = argparse.ArgumentParser()
    parser.add_argument("-u", "--user", help="User ID")
    parser.add_argument("-l", "--lang", default="Rust", help="Language")
    parser.add_argument("-b", "--browse", action="store_true", help="Browse all users/languages")
    args = parser.parse_args()

    db = sqlite3.connect(Path(__file__).parent.parent / "data" / "cache.db")

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
            print_stats(db, args.user, args.lang)

        else:
            sql = "select distinct key from inputs order by key"
            users = list(sorted(set(map(lambda row: row[0].split(":")[2], db.execute(sql)))))
            current_user = 0

            languages = ("Rust", "Python", "C", "C++", "Go")
            current_language = 0

            done = False

            while not done:
                print(end="\033[H\033[2J")

                print(f"User: {users[current_user]}")
                print()

                print_stats(db, users[current_user], languages[current_language].casefold())

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
