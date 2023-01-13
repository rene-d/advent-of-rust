#!/usr/bin/env python3

from pathlib import Path
import re
import requests
import argparse
import subprocess
from datetime import datetime
import sqlite3
import zlib
import time
import logging


class AocSession:
    last_submit_time = 0  # submissions must be at least 5 seconds apart
    rootdir = Path(__file__).parent.parent

    def __init__(self, session, force_update=False, dry_run=False) -> None:
        self.data_dir = self.rootdir / "data"
        self.stars = {}
        self.data_dir.mkdir(exist_ok=True, parents=True)
        self.force_update = force_update
        self.dry_run = dry_run
        self.db = sqlite3.connect(self.data_dir / "cache.db")

        self.db.executescript(
            """
            create table if not exists cache (url text,user text,last_modified date,content text);
            create table if not exists answers (user text,year integer,day integer,language text, st_mtime float,part1 text,part2 text);
            create unique index if not exists answers_idx on answers (user,year,day,language);
            """
        )

        if isinstance(session, tuple):
            self.user, cookie = session
        else:
            cookie = session
            self.user = None

        self.sess = requests.Session()
        self.sess.cookies["session"] = cookie

        if not self.user:
            r_text = self.get("https://adventofcode.com/settings").decode()

            m = re.search(r'<div class="user">(.+?)\s*<', r_text)
            if m is None:
                logging.error(f"Cannot retrieve session {cookie}")
                self.user = "unknown"
            else:
                user = m[1]

                m = re.search(r"<span>\(anonymous user #(\d+)\)</span>", r_text)
                user_id = m[1]

                if "anonymous user" in user:
                    self.user = f"anon-{user_id}"
                else:
                    self.user = user

        self.user_dir = self.data_dir / self.user
        self.user_dir.mkdir(parents=True, exist_ok=True)

        self.prefix = f"\033[1;36m[{self.user:<12}]\033[0m "

    def get_sessions():
        f = AocSession.rootdir / "session"
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
            self.db.execute("delete from cache where url=? and user=?", (url, self.user))
        else:
            cursor = self.db.execute(
                "select last_modified,content from cache where url=? and user=?", (url, self.user or self.sess.cookies["session"])
            )
            row = cursor.fetchone()
            if row:
                return zlib.decompress(row[1])

        r = self.sess.get(url)
        r.raise_for_status()

        self.db.execute(
            "insert into cache (url,user,last_modified,content) values (?,?,?,?)",
            (url, self.user or self.sess.cookies["session"], datetime.utcnow(), zlib.compress(r.content)),
        )
        self.db.commit()

        return r.content

    def is_available(year, day):
        now = datetime.utcnow()
        if (year > now.year) or (year == now.year and (now.month < 12 or now.day < day or (day == now.day and now.hour < 5))):
            return False
        return True

    def iter_all(func):
        def wrapper(self, year=None, day=None):
            if year is None:
                now = datetime.utcnow()
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
        nb_stars = self.get_stars(year, day)

        if nb_stars > 0:
            f = self.user_dir / str(year)
            f.mkdir(parents=True, exist_ok=True)
            f /= f"{day}.ok"

            if f.exists():
                nfound = len(f.read_text().splitlines())
                if nb_stars != nfound and not (nb_stars == 2 and nfound == 1 and day == 25):
                    f.unlink()

            if not f.exists():
                url = f"https://adventofcode.com/{year}/day/{day}"
                r_text = self.get(url).decode()
                answers = [answer for answer in re.findall(r"<p>Your puzzle answer was <code>([\w=-]+)</code>", r_text)]
                # print(nb_stars, len(answers),day)
                # assert (len(answers) == nb_stars) or (len(answers) == 1 and nb_stars == 2 and day == 25)
                if len(answers) > 0:
                    f.parent.mkdir(parents=True, exist_ok=True)
                    f.write_text("\n".join(answers) + "\n")

                    print(f"{self.prefix} Stars for {year} day {day}: {'⭐'*nb_stars}")

            return f

        # print(f"{self.prefix} Stars for {year} day {day}: ⃞⃞")

    @iter_all
    def check(self, year=None, day=None):

        n = self.get_stars(year, day)
        if n >= 1:
            answers = self.get_answers(year, day).read_text().splitlines()
        else:
            answers = None

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
                Path(f"{year}_{day}_{level}.log").write_bytes(r.content)

            print(f"{self.prefix} Submission for part {level}: {answer} ⇒ {r} {result}")
            return result == "SUCCESS"

        def run(p, language):
            p = self.rootdir / Path(p)
            if not p.exists():
                return

            def update_last_answers(parts):
                part1 = parts[0]
                part2 = parts[1] if len(parts) > 1 else None

                self.db.execute(
                    "insert or replace into answers (user,year,day,language,st_mtime,part1,part2) values (?,?,?,?,?,?,?)",
                    (self.user, year, day, language, p.stat().st_mtime, part1, part2),
                )
                self.db.commit()

            # check in program has been modified since last check
            cursor = self.db.execute(
                "select st_mtime,part1,part2 from answers where user=? and year=? and day=? and language=?",
                (self.user, year, day, language),
            )
            row = cursor.fetchone()
            if row and row[0] == p.stat().st_mtime:
                # not modified
                parts = [row[1]]
                if row[2]:
                    parts.append(row[2])
            else:
                # run the program to solve the puzzle
                cmd = [p.as_posix(), self.get_input(year, day).as_posix()]
                try:
                    parts = subprocess.check_output(cmd, stderr=subprocess.DEVNULL).decode().strip().split()
                except (subprocess.CalledProcessError, PermissionError) as e:
                    print(f"{self.prefix} Program for {year} day {day} in {language} \033[91mfailed\033[0m")
                    parts = []

            if parts == answers and len(parts) > 0:
                print(f"{self.prefix} Solution {year} day {day} in {language:<6} \033[92mok\033[0m")
                update_last_answers(parts)
                return

            if answers:

                if len(parts) == 2 and len(answers) == 1 and answers[0] == parts[0]:

                    if (
                        not self.dry_run
                        and input(
                            f"{self.prefix} Answer for {year} day {day} in part 2 missing: {parts[1]}. Submit it ({language}) (y/N) ? "
                        )
                        == "y"
                    ):
                        success = submit(2, parts[1])
                        if success:
                            update_last_answers(parts)

                        self.get_stars(year, day, True)
                        self.get_answers(year, day)

                else:
                    print(f"{self.prefix} {year} day {day} {language} \033[91merror\033[0m '{' '.join(cmd)}' {parts} != {answers}")

            elif len(parts) > 0:
                if (
                    not self.dry_run
                    and input(f"{self.prefix} Answers for {year} day {day} missing: {parts}. Submit them ({language}) (y/N) ? ") == "y"
                ):
                    success = submit(1, parts[0])
                    if len(parts) >= 2:
                        success = success and submit(2, parts[1])

                    if success:
                        update_last_answers(parts)

                    self.get_stars(year, day, True)
                    self.get_answers(year, day)

            else:
                # no solution yet or exec error
                pass

        run(f"{year}/target/release/day{day}", "Rust")
        run(f"{year}/day{day}/day{day}.py", "Python")

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


def main():

    parser = argparse.ArgumentParser()
    parser.add_argument("-v", "--verbose", action="store_true", help="show request details")
    parser.add_argument("-y", "--year", type=int, help="optional day, automatic if in ./\033[3mYEAR\033[0m")
    parser.add_argument("-d", "--day", type=int, help="optional day, automatic if in ./\033[3mYEAR\033[0m/day\033[3mDAY\033[0m")
    parser.add_argument("-s", "--session", type=str, help="session cookie")
    parser.add_argument("--user", type=str, help="user")
    parser.add_argument("-u", "--update", action="store_true", help="force update")
    parser.add_argument("-n", "--dry-run", action="store_true", help="do nothing")
    parser.add_argument("--dstars", action="store_true", help="show stars for each day")
    parser.add_argument("--ystars", action="store_true", help="show stars by year")
    args = parser.parse_args()

    if args.verbose:
        logging.basicConfig(format="\033[2m%(asctime)s - %(levelname)s - %(message)s\033[0m", level=logging.DEBUG)

    sessions = AocSession.get_sessions()

    if args.session:
        sessions.clear()
        sessions.append(args.session)

    cwd = Path.cwd()
    if cwd.name.startswith("day") and args.day is None:
        args.year = int(cwd.parent.name)
        args.day = int(cwd.name[3:])
    if cwd.name.isdigit() and args.year is None:
        args.year = int(cwd.name)

    if args.dstars:

        users = []

        for session in sessions:
            sess = AocSession(session, args.update, args.dry_run)
            if args.user and sess.user != args.user:
                continue
            users.append(sess)

        @AocSession.iter_all
        def show_year(_self, year, _day):
            stars = {}
            for sess in users:
                stars[sess.user] = []
                for day in range(1, 26):
                    stars[sess.user].append(sess.get_stars(year, day))

            row = "|".join(f"\033[1;36m{sess.user:^12}\033[0m" for sess in users)
            print(f"  {year} |{row}")
            separator = "+".join("-" * 12 for _ in users)
            print(f"-------+{separator}")
            for day in range(1, 26):
                row = "|".join(f"\033[1;33m{'*' *  stars[sess.user][day-1]:^12}\033[0m" for sess in users)
                print(f"day {day:2} |{row}")
            print(f"-------+{separator}")
            print()

        show_year(None, args.year, 0)

        exit()

    for session in sessions:

        sess = AocSession(session, args.update, args.dry_run)

        if args.user and sess.user != args.user:
            continue

        if args.dstars:
            sess.print_stars(year=args.year, day=args.day)
        elif args.ystars:
            sess.print_stars_year(args.year)
        else:
            sess.check(year=args.year, day=args.day)


if __name__ == "__main__":
    main()
