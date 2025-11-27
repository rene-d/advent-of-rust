#!/usr/bin/env python3

import argparse
import sqlite3
from pathlib import Path

from tabulate import tabulate

parser = argparse.ArgumentParser()
parser.add_argument("-v", "--verbose", action="store_true", help="Show details")
parser.add_argument("-y", "--year", type=int, help="Filter by year")
parser.add_argument("-d", "--day", type=int, help="Filter by day")
parser.add_argument("-u", "--user", type=str, help="Filter by user")
parser.add_argument("--me", action="store_true", help="Filter by user")
parser.add_argument("languages", nargs="*")
args = parser.parse_args()

data_dir = Path(__file__).parent.parent / "data"

if len(args.languages) == 1:
    parser.error("bad language count (0 or 2 and more)")

if args.me:
    for d in sorted(data_dir.glob("*")):
        if d.is_dir() and d.name.isdigit():
            args.user = d.name
            break

db = sqlite3.connect(data_dir / "cache.db")


def fetch(lang: str) -> dict:
    timings = dict()

    if args.user:
        sql = (
            r"select year,day,user,elapsed from user_solutions"
            r" where lang=? and user=? and prog not like '%\_%/%' escape '\'"
        )
        params = [lang, args.user]
    else:
        sql = (
            r"select year,day,crc,elapsed from solutions"
            r" where lang=? and (status='ok' or status='unknown') and prog not like '%\_%/%' escape '\'"
        )
        params = [lang]

    if args.year:
        sql += " and year=?"
        params.append(args.year)

    if args.day:
        sql += " and day=?"
        params.append(args.day)

    for year, day, user_or_crc, elapsed in db.execute(sql, params):
        timings[year, day, user_or_crc] = elapsed
    return timings


languages = args.languages
timings = []
elapsed = []

if len(languages) == 0:
    languages.append("py3.11")
    languages.append("py3.14")

for lang in languages:
    timings.append(fetch(lang))

keys = set(timings[0].keys())
for t in timings[1:]:
    keys.intersection_update(t.keys())

if len(keys) == 0:
    print("aucune solution commune")
    exit()

for t in timings:
    elapsed.append(sum(map(lambda v: t[v], keys)))

cmp = []
for t1, t2 in zip(timings, timings[1:]):
    cmp.append(sum(t2[key] / t1[key] for key in keys) / len(keys))


if args.verbose:
    data = []
    headers = ["puzzle"]
    for lang in languages:
        headers.append(lang)
    if len(languages) == 2:
        headers.append("ratio")
    for key in sorted(keys):
        year, day, _ = key
        row = [f"{year} {day}"]

        if len(languages) == 2:
            t0 = timings[0][key]
            t1 = timings[1][key]
            row.extend((t0 / 1e9, t1 / 1e9, t1 / t0))
        else:
            for t in timings:
                row.append(t[key] / 1e9)

        data.append(row)

    floatfmt = ["", ".5f", ".5f", ".2f"] if len(languages) == 2 else ".5f"
    print(tabulate(data, headers, tablefmt="rounded_outline", floatfmt=floatfmt))


print(f"Solutions : {len(keys)}")
for lang, e in zip(languages, elapsed):
    print(f"{lang:8}  : {e / 1e9:.5f} s")

for i in range(0, len(languages) - 1):
    print(
        f"{languages[i]:8} vs. {languages[i + 1]:8} "
        f" average : {cmp[i] * 100:.1f} %"
        f" - overall : {(elapsed[i + 1] / elapsed[i]) * 100:.1f} %"
    )
