#!/usr/bin/env python3

import argparse
import logging
import os
import re
import sqlite3
import subprocess
import time
import typing as t
import zlib
from collections import defaultdict
from datetime import UTC, datetime
from pathlib import Path

import requests


class AocSession:
    last_submit_time = 0  # submissions must be at least 5 seconds apart
    rootdir = Path(__file__).parent.parent

    def __init__(self, cookie_session, force_update=False, dry_run=False) -> None:

        self.stars = {}
        (self.rootdir / "data").mkdir(exist_ok=True, parents=True)
        self.force_update = force_update
        self.dry_run = dry_run
        self.db = sqlite3.connect(self.rootdir / "data" / "cache.db")
        self.always_submit = False

        self.db.executescript(
            """
            create table if not exists cache (url text,user text,last_modified date,content text);
            create table if not exists answers (user text,year integer,day integer,language text, st_mtime float,part1 text,part2 text);
            create unique index if not exists answers_idx on answers (user,year,day,language);
            """
        )

        name = None
        self.user_id = ""

        self.sess = requests.Session()
        self.sess.cookies["session"] = cookie_session

        r_text = self.get("https://adventofcode.com/settings").decode()

        m = re.search(r'<div class="user">(.+?)\s*<', r_text)
        if m is None:
            logging.error(f"Cannot retrieve session {cookie_session}")
            name = "unknown"
        else:
            name = m[1]

            m = re.search(r"<span>\(anonymous user #(\d+)\)</span>", r_text)
            self.user_id = m[1]

            if "anonymous user" in name:
                name = f"anon-{self.user_id}"

        self.user_name = name
        self.user_dir = self.rootdir / "data" / self.user_id
        self.user_dir.mkdir(parents=True, exist_ok=True)

        self.prefix = f"\033[1;36m[{name:<20} {self.user_id:<6}]\033[0m "

    def year_dir(self, year: int) -> Path:

        return self.rootdir / "src" / f"year{year}"

    def get_cookie_sessions() -> t.List[str]:
        """Return the list of cookie sessions from `session` file."""
        f = AocSession.rootdir / ".session"
        sessions = []
        if f.exists():
            for line in f.read_text().splitlines():
                line = line.strip()
                if line and not line.startswith("#"):
                    sessions.append(line)
        return sessions

    def get(self, url, force_update=False):
        """Do a HTTP request."""

        if force_update or self.force_update:
            self.db.execute("delete from cache where url=? and user=?", (url, self.user_id))
            self.db.commit()
        else:
            cursor = self.db.execute(
                "select last_modified,content from cache where url=? and user=?",
                (url, self.user_id or self.sess.cookies["session"]),
            )
            row = cursor.fetchone()
            if row:
                return zlib.decompress(row[1])

        r = self.sess.get(url)
        r.raise_for_status()

        self.db.execute(
            "insert into cache (url,user,last_modified,content) values (?,?,?,?)",
            (
                url,
                self.user_id or self.sess.cookies["session"],
                datetime.now(UTC).isoformat(),
                zlib.compress(r.content),
            ),
        )
        self.db.commit()

        return r.content

    def is_available(year, day):
        now = datetime.now(UTC)
        if (year > now.year) or (
            year == now.year and (now.month < 12 or now.day < day or (day == now.day and now.hour < 5))
        ):
            return False
        return True

    def iter_all(func, b=None):

        def wrapper(self, year=None, day=None, *args, **kwargs):

            if year is None:
                now = datetime.now(UTC)
                last_year = now.year
                if AocSession.is_available(last_year, 1):
                    last_year += 1
                for year in range(2015, last_year):
                    wrapper(self, year, day)
            elif day is None:
                # iterate over every days
                for day in range(1, 26):
                    wrapper(self, year, day)
            elif day == 0:
                # special case to iterate only on years
                return func(self, year, 0)
            elif AocSession.is_available(year, day):
                return func(self, year, day)

        # if b is not None:
        #     def wrapper2(year, day):
        #         return wrapper(func, year,day )

        #     return wrapper2

        return wrapper

    @iter_all
    def get_input(self, year=None, day=None):
        f = self.user_dir / str(year)
        f.mkdir(parents=True, exist_ok=True)
        f /= f"{day}.in"
        if not f.exists():
            url = f"https://adventofcode.com/{year}/day/{day}/input"
            r = self.get(url)
            f.write_bytes(r)
            print(f"downloaded: {f}")
        return f

    def get_stars(self, year, day, force_update=False):
        stars = self.stars.get(year)

        if not stars or force_update:
            r_text = self.get(f"https://adventofcode.com/{year}", force_update).decode()

            parts_done = {
                "": 0,
                "calendar-complete": 1,
                "calendar-verycomplete": 2,
            }

            stars = dict((int(m[0]), parts_done[m[1]]) for m in re.findall(r'class="calendar-day(\d+) ?(.*?)"', r_text))
            self.stars[year] = stars

        return stars.get(day, 0)

    @iter_all
    def get_answers(self, year=None, day=None):
        logging.debug(f"get_answers {year} {day}")

        nb_stars = self.get_stars(year, day)

        logging.debug(f"nb_stars {nb_stars}")

        if nb_stars > 0:
            f = self.user_dir / str(year)
            f.mkdir(parents=True, exist_ok=True)
            f /= f"{day}.ok"

            logging.debug(f"f {f} {f.exists()}")

            if f.exists():
                nfound = len(f.read_text().splitlines())
                if nb_stars != nfound and not (nb_stars == 2 and nfound == 1 and day == 25):
                    f.unlink()

            if not f.exists():
                url = f"https://adventofcode.com/{year}/day/{day}"
                r_text = self.get(url).decode()
                answers = [
                    answer for answer in re.findall(r"<p>Your puzzle answer was <code>([\w,=-]+)</code>", r_text)
                ]

                logging.debug("answers", answers)

                if len(answers) == 0:
                    r_text = self.get(url, True).decode()
                    answers = [
                        answer for answer in re.findall(r"<p>Your puzzle answer was <code>([\w,=-]+)</code>", r_text)
                    ]

                    logging.debug("answers", answers)

                # print(nb_stars, len(answers),day)
                # assert (len(answers) == nb_stars) or (len(answers) == 1 and nb_stars == 2 and day == 25)
                if len(answers) > 0:
                    f.parent.mkdir(parents=True, exist_ok=True)
                    f.write_text("\n".join(answers) + "\n")

                    print(f"{self.prefix} Stars for {year} day {day:2}: {'⭐'*nb_stars}")

            return f

        # print(f"{self.prefix} Stars for {year} day {day}: ⃞⃞")

    @iter_all
    def check(self, year=None, day=None):
        def submit(level, answer):
            url = f"https://adventofcode.com/{year}/day/{day}/answer"

            data = {"answer": answer, "level": level}

            # do not submit answers too fast
            if self.last_submit_time != 0:
                time.sleep(max(0, 5 - (time.time() - self.last_submit_time)))
            r = self.sess.post(url, data=data)
            self.last_submit_time = time.time()

            if "That's not the right answer" in r.text:
                result = "FAIL"
            elif "That's the right answer" in r.text:
                result = "SUCCESS"
            elif "please wait" in r.text or "you have to wait" in r.text:
                result = "WAIT"
            else:
                result = "UNKNOWN"
                # Path(f"{year}_{day}_{level}.log").write_bytes(r.content)

            print(f"{self.prefix} Submission for part {level}: {answer} ⇒ {r} {result}")
            if not result == "SUCCESS":
                self.always_submit = False
            return result == "SUCCESS"

        def run(p, language):
            nb_stars = self.get_stars(year, day)
            if nb_stars >= 1:
                answers = self.get_answers(year, day).read_text().splitlines()
            else:
                answers = None

            p = self.rootdir / Path(p)
            if not p.exists():
                return

            def update_last_answers(parts):
                part1 = parts[0]
                part2 = parts[1] if len(parts) > 1 and parts[1] != "???" else None

                self.db.execute(
                    "insert or replace into answers (user,year,day,language,st_mtime,part1,part2) values (?,?,?,?,?,?,?)",
                    (self.user_id, year, day, language, p.stat().st_mtime, part1, part2),
                )
                self.db.commit()

            def submit_parts(step, parts):
                if self.dry_run:
                    return

                question = (
                    f"{self.prefix} Answer for {year} day {day:2} part 2 is missing: {parts[1]}. Submit it ({language}) (y/a/N) ? "
                    if step == "second"
                    else f"{self.prefix} Answers for {year} day {day:2} are missing: {parts}. Submit them ({language}) (y/a/N) ? "
                )

                if not self.always_submit:
                    while True:
                        resp = input(question)
                        if not resp:
                            continue
                        if resp == "N":
                            self.dry_run = True
                            return
                        if resp.lower() in "yan":
                            break
                        if resp.lower() in "qx":
                            exit(0)
                else:
                    resp = "y"

                if resp == "a":
                    self.always_submit = True
                    resp = "y"

                if resp == "y":
                    if step == "first":
                        success = submit(1, parts[0])
                    else:
                        success = True

                    if len(parts) >= 2 and parts[1] != "???":
                        success = success and submit(2, parts[1])

                    if success:
                        update_last_answers(parts)

                    self.get_stars(year, day, True)
                    self.get_answers(year, day)

            remarks = ""

            # check in program has been modified since last check
            cursor = self.db.execute(
                "select st_mtime,part1,part2 from answers where user=? and year=? and day=? and language=?",
                (self.user_id, year, day, language),
            )
            row = cursor.fetchone()
            if row and row[0] == p.stat().st_mtime:
                # not modified
                parts = [row[1]]
                if row[2]:
                    parts.append(row[2])

                remarks += " (cached)"

                cmd = [p.resolve().as_posix(), self.get_input(year, day).resolve().as_posix()]
            else:
                # run the program to solve the puzzle
                cmd = [p.as_posix()]
                if p.stem == "one":
                    cmd.append("-r")
                    cmd.append(f"{year}:{day}")
                cmd.append(self.get_input(year, day).as_posix())

                try:
                    start_time = datetime.now()
                    parts = subprocess.check_output(cmd, stderr=subprocess.DEVNULL).decode().strip().split()
                    elapsed = datetime.now() - start_time
                    remarks += f" ({elapsed.total_seconds():.6f}s)"
                except (subprocess.CalledProcessError, PermissionError):
                    print(f"{self.prefix} Program for {year} day {day:2} in {language} \033[91mfailed\033[0m")
                    parts = []

            if parts == answers and len(parts) > 0:
                print(f"{self.prefix} Solution {year} day {day:2} in {language:<6} \033[92mok\033[0m {remarks}")
                update_last_answers(parts)
                return True

            if len(parts) == 2 and parts[1] == "???":
                parts.pop(1)

            if answers:
                if len(parts) == 2 and len(answers) == 1 and answers[0] == parts[0]:
                    submit_parts("second", parts)
                else:
                    if parts == answers:
                        print(
                            f"{self.prefix} Solution {year} day {day:2} in {language} \033[93mwarning part 2 is missing\033[0m "
                        )
                    else:
                        print(
                            f"{self.prefix} {year} day {day:2} {language} \033[91merror\033[0m '{' '.join(cmd)}' {parts} != {answers}"
                        )
            elif len(parts) > 0:
                submit_parts("first", parts)

            else:
                # no solution yet or exec error
                pass

        (
            run(f"src/year{year}/day{day}/target/release/day{day}", "Rust")
            or run("target/release/one", "Rust")
            or run(f"src/year{year}/day{day}/day{day}.py", "Python")
        )

    @iter_all
    def print_stars(self, year=None, day=None):
        nb_stars = self.get_stars(year, day)
        print(f"{self.prefix} Stars for {year} day {day:2}: {'⭐'*nb_stars}")

    def print_stars_year(self, year=None):
        @AocSession.iter_all
        def iterate(self, year, day):
            nb_stars = sum(self.get_stars(year, day) for day in range(1, 26))
            print(f"{self.prefix} Stars for {year}: {nb_stars:2}⭐")

        iterate(self, year, 0)

    def get_title(self, year, day) -> str:
        """TODO."""

        url = f"https://adventofcode.com/{year}/day/{day}"
        r = self.get(url)
        title = re.search(r"<h2>\-\-\- (.+?) \-\-\-</h2>", r.decode()).group(1)
        title = title.replace("&apos;", "'")
        if "&" in title:
            print(title)
            exit(2)
        markdown = f"[{title}]({url})"
        return markdown

    def get_solutions(self, year, day) -> t.List[str]:
        """TODO."""
        sols = []

        year_dir = self.year_dir(year)
        days = [year_dir / f"day{day}"] + list(year_dir.glob(f"day{day}_*"))

        for day_dir in days:
            if day_dir.is_dir():
                for f in day_dir.glob("day*.*"):
                    if f.is_file():
                        sols.append(f)

        # print(f"solution for {year} {day}: {sols}")

        return sols

    @iter_all
    def get_titles(self, year=None, day=None):
        """TODO."""

        print(self.get_title(year, day))


def get_first_session(args):
    sessions = AocSession.get_cookie_sessions()

    sess = None
    for session in sessions:
        sess = AocSession(session, args.update, args.dry_run)
        if args.user and args.user not in (sess.user_id, sess.user_name):
            continue
        return sess

    print("no valid session found")
    exit(2)


def get_languages(sols, readme_dir: Path, rootdir: Path):
    files = []

    path_to_home = os.path.relpath(rootdir, readme_dir)

    for lang, suffix, icon in (
        ("Rust", ".rs", "rust"),
        ("Python", ".py", "python"),
        ("C", ".c", "c"),
        ("C++", ".cpp", "cpp"),
        ("Go", ".go", "go"),
        ("Ruby", ".rb", "ruby"),
        ("Perl", ".pl", "perl"),
        ("Lua", ".lua", "lua"),
        ("JS", ".js", "javascript"),
        ("Bash", ".sh", "bash"),
        ("Swift", ".swift", "swift"),
        ("Java", ".java", "java"),
        ("C#", ".cs", "csharp"),
        ("SQLite", ".sql", "sqlite"),
        ("Linux Kernel", ".kernel", "kernel"),
    ):

        for sol in sols:
            if sol.suffix == suffix:
                files.append(f"[![{lang}]({path_to_home}/scripts/assets/{icon}.png)]({sol.relative_to(readme_dir)})")

    sol_dir = sols[0].parent
    if sol_dir.name == "src":
        sol_dir = sol_dir.parent
    f = sol_dir / "README.md"
    if f.is_file():
        files.append(f"[🎁]({f.relative_to(readme_dir)})")

    return " ".join(files)


def make_readme(args):
    session = get_first_session(args)

    @AocSession.iter_all
    def readme(self, year, _day):

        puzzles = []
        now = datetime.now(UTC)
        for day in range(1, 26):
            available_date = datetime(year, 12, day, 5, 0, 0, 0, tzinfo=UTC)
            if available_date > now:
                continue

            stars = session.get_stars(year, day)
            title = session.get_title(year, day)
            sols = session.get_solutions(year, day)
            puzzles.append((day, stars, title, sols))

        stars = sum(n for _, n, _, _ in puzzles)
        rust = sum(1 for _, _, _, s in puzzles for f in s if f.suffix == ".rs")
        python = sum(1 for _, _, _, s in puzzles for f in s if f.suffix == ".py")

        year_dir = self.year_dir(year)

        md = []
        md.append("# Advent of Code in Rust 🦀")
        md.append("")
        md.append(f"![AoC{year}](https://img.shields.io/badge/Advent_of_Code-{year}-8A2BE2)")
        md.append(f"![Stars: {stars}](https://img.shields.io/badge/Stars-{stars}⭐-blue)")
        if rust:
            md.append(f"![Rust: {rust}](https://img.shields.io/badge/Rust-{rust}-cyan?logo=Rust)")
        if python:
            md.append(f"![Python: {python}](https://img.shields.io/badge/Python-{python}-cyan?logo=Python)")
        md.append("")
        md.append(f"## {year} ([Calendar](https://adventofcode.com/{year})) ([Solutions](./)) : {stars}⭐")
        md.append("")

        width = max(len(title) for _, _, title, _ in puzzles)

        md.append(f"{'Puzzle':<{width}} | Stars | Languages")
        md.append("-" * width + " | ----- | -----------")

        for day, stars, title, sols in puzzles:
            if stars > 0:
                stars = "⭐" * stars

                files = get_languages(sols, year_dir, session.rootdir)

                md.append(f"{title:<{width}} | {stars:<2}  | {files}")

        bonus = year_dir / ".bonus.md"
        if bonus.is_file():
            md.append("")
            md.extend(bonus.read_text().strip().splitlines())

        md.append("")
        md = "\n".join(md)

        if args.write:
            readme = year_dir / "README.md"
            if not readme.is_file() or md != readme.read_text():
                readme.write_text(md)
                print(f"{readme} written")
        else:
            print(md)

    readme(session, args.year, 0)


def make_readme_main(args):
    session = get_first_session(args)

    puzzles = []

    @AocSession.iter_all
    def parse(self, year, _day):
        now = datetime.now(UTC)
        for day in range(1, 26):
            available_date = datetime(year, 12, day, 5, 0, 0, 0, tzinfo=UTC)
            if available_date > now:
                continue
            stars = session.get_stars(year, day)
            title = session.get_title(year, day)
            sols = session.get_solutions(year, day)
            puzzles.append((year, day, stars, title, sols))

    parse(session, None, 0)

    stars = sum(n for _year, _day, n, _title, _sols in puzzles)
    # rust = sum(1 for _year, _day, _stars, _title, sols in puzzles for f in sols if f.suffix == ".rs")
    # python = sum(1 for _year, _day, _stars, _title, sols in puzzles for f in sols if f.suffix == ".py")

    rust = defaultdict(lambda: 0)
    python = defaultdict(lambda: 0)
    all_stars = defaultdict(lambda: 0)
    bonus = defaultdict(list)

    for year, day, stars, title, sols in puzzles:
        if any(f.suffix == ".rs" for f in sols):
            rust[year] += 1
        if any(f.suffix == ".py" for f in sols):
            python[year] += 1
        all_stars[year] += stars

        if sols:
            n = sols[0].parent
            if n.name == "src":
                n = n.parent
            n = n / "README.md"
            if n.is_file():
                bonus[year].append(n)

    total_rust = sum(rust.values())
    total_python = sum(python.values())
    total_stars = sum(all_stars.values())
    current_calendar = 2024  # max(all_stars.keys())

    rows = []
    for year in sorted(all_stars.keys(), reverse=True):
        row = [
            f"[Advent of Code {year}](https://adventofcode.com/{year})",
            f"[Solutions]({session.year_dir(year).relative_to(session.rootdir)}/README.md)",
            f"{all_stars[year]:>3}⭐",
            f"{rust[year]:>3}",
            f"{python[year]:>3}",
            f"{len(bonus[year]) or '':>3}",
        ]
        rows.append(" | ".join(row))

    readme = session.rootdir / "README.md"

    md = []
    skip = False
    for line in readme.read_text().splitlines():

        if skip:
            if line.startswith("##"):
                skip = False
            else:
                continue

        if line == "## Bonus 🎁":
            skip = True
            md.append(line)
            md.append("")
            md.append(" | ".join(("Year", "Count", "Days")))
            md.append(" | ".join(("----", "-----", "--------------------")))
            for year in reversed(sorted(bonus)):
                if bonus[year]:
                    s = f"{year} | {len(bonus[year]):5} |"
                    for b in bonus[year]:
                        day = b.parent.stem.removeprefix("day")
                        s += f" [{day}]({b.relative_to(session.rootdir)})"
                    md.append(s)
            md.append("")
            continue

        if line == "## All years":
            skip = True
            md.append(line)
            md.append("")
            md.append(" | ".join(("Calendar", "Solutions", "Stars", "Rust", "Python", "🎁")))
            md.append(" | ".join(("--------", "---------", "-----", "----", "------", "--")))
            md.extend(rows)
            md.append("")
            continue

        if line.startswith("## Current year") or "current event" in line:
            skip = True
            line = (
                f"## {current_calendar} (current event)"
                f" ([Calendar](https://adventofcode.com/{current_calendar}))"
                f" ([Solutions]({session.year_dir(year).relative_to(session.rootdir)}/)) :"
                f" {all_stars[current_calendar]}⭐"
            )
            md.append(line)
            md.append("")

            width = 10
            for year, day, stars, title, sols in puzzles:
                if year != current_calendar:
                    continue
                width = max(width, len(title))

            md.append(f"{'Puzzle':<{width}} | Stars | Languages")
            md.append("-" * width + " | ----- | -----------")

            for year, day, stars, title, sols in puzzles:
                if year != current_calendar:
                    continue

                if stars > 0:
                    stars = "⭐" * stars

                    files = get_languages(sols, session.rootdir, session.rootdir)

                    md.append(f"{title:<{width}} | {stars:<2}  | {files}")

            md.append("")
            continue

        if line.startswith("![Stars:"):
            line = f"![Stars: {total_stars}](https://img.shields.io/badge/Stars-{total_stars}⭐-blue)"

        elif line.startswith("![Rust:"):
            line = f"![Rust: {total_rust}](https://img.shields.io/badge/Rust-{total_rust}-cyan?logo=Rust)"

        elif line.startswith("![Python:"):
            line = f"![Python: {total_python}](https://img.shields.io/badge/Python-{total_python}-cyan?logo=Python)"

        md.append(line)

    if args.write:
        if md[-1] != "":
            md.append("")
        md = "\n".join(md)
        if not readme.is_file() or md != readme.read_text():
            readme.write_text(md)
            print(f"{readme} written")
    else:
        print("\n".join(md))


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("-v", "--verbose", action="store_true", help="show request details")
    parser.add_argument("-y", "--year", type=int, help="optional day")
    parser.add_argument("-d", "--day", type=int, help="optional day")
    # parser.add_argument("-s", "--session", type=str, help="session cookie")
    parser.add_argument("--user", type=str, help="user")
    parser.add_argument("-u", "--update", action="store_true", help="force update")
    parser.add_argument("-n", "--dry-run", action="store_true", help="do nothing")
    parser.add_argument("--yes", action="store_true", help="always yes")
    parser.add_argument("--inputs", action="store_true", help="download inputs")
    parser.add_argument("--get-titles", action="store_true", help="get puzzle title")
    parser.add_argument("--readme", action="store_true", help="make readme")
    parser.add_argument("-w", "--write", action="store_true", help="write the readme")

    args = parser.parse_args()

    if args.verbose:
        logging.basicConfig(format="\033[2m%(asctime)s - %(levelname)s - %(message)s\033[0m", level=logging.DEBUG)

    cwd = Path.cwd()
    if cwd.name.startswith("day") and args.day is None:
        args.year = int(cwd.parent.name.removeprefix("year"))
        n = cwd.name.removeprefix("day")
        n = n[: n.find("_")] if "_" in n else n
        args.day = int(n)
    if cwd.name.startswith("year") and args.year is None:
        args.year = int(cwd.name.removeprefix("year"))

    if args.readme:
        make_readme(args)
        if args.write or args.year is None:
            make_readme_main(args)

    elif args.get_titles:
        session = get_first_session(args)
        session.get_titles(year=args.year, day=args.day)

    else:
        for session in AocSession.get_cookie_sessions():
            sess = AocSession(session, args.update, args.dry_run)

            if args.user and args.user not in (sess.user_id, sess.user_name):
                continue

            if args.inputs:
                sess.get_input(year=args.year, day=args.day)
            else:
                sess.always_submit = args.yes
                sess.check(year=args.year, day=args.day)


if __name__ == "__main__":
    main()
