#!/usr/bin/env python3

import argparse
import hashlib
import itertools
import os
import shutil
import sqlite3
import subprocess
import time
import typing as t
from collections import defaultdict
from datetime import datetime
from operator import itemgetter
from pathlib import Path

# from zlib import crc32

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
CR = "\r"

LANGUAGES = {
    "Python": "{year}/day{day}/day{day}.py",
    "Rust": "{year}/target/release/day{day}",
    "C": "{year}/build/day{day}_c",
    "C++": "{year}/build/day{day}_cpp",
}

INTERPRETERS = {
    "Python": {
        "Python": "python3",
        "PyPy": ".venv/pypy3.10/bin/python",
        "Py3.10": ".venv/py3.10/bin/python",
        "Py3.11": ".venv/py3.11/bin/python",
        "Py3.12": ".venv/py3.12/bin/python",
        "Py3.13": ".venv/py3.13/bin/python",
    }
}


def get_cache():
    """Retrieve the cache instance from memory or load it from disk."""
    cache = globals().get("_cache")
    if cache is None:
        cache_file = Path(__file__).parent.parent / "data" / "cache.db"

        cache_db = sqlite3.connect(cache_file)

        cache_db.executescript(
            "create table if not exists solutions ("
            " key text primary key not null,"
            " mtime_ns int,"
            " elapsed float,"
            " status text,"
            " answers text"
            ");"
            "create table if not exists inputs ("
            " key text primary key not null,"
            " mtime_ns int,"
            " crc32 text"
            ");"
        )
        cache = {"db": cache_db, "modified": False}
        globals()["_cache"] = cache

    return cache


def save_cache():
    pass
    # cache = get_cache()
    # if cache["modified"]:
    #     cache["db"].commit()
    #     cache["modified"] = False
    #     print(f"{FEINT}{ITALIC}cache commited{RESET}")


def check_cache(key, file_timestamp: Path, table: str, columns: t.Iterable[str], no_age_check=False):
    cache = get_cache()
    key = str(key)
    db = cache["db"]
    db.row_factory = sqlite3.Row
    cursor = db.execute(f"select * from `{table}` where key=?", (key,))
    row = cursor.fetchone()
    if row:
        timestamp = file_timestamp.stat().st_mtime_ns
        if row["mtime_ns"] == timestamp or no_age_check:
            return dict((column, row[column]) for column in columns)

        else:
            # seconds = round((timestamp - e["timestamp"]) / 1000000000)
            # delta = timedelta(seconds=seconds)
            # print(f"{FEINT}{ITALIC}entry {key} is out of date for {delta}{RESET}", end=f"{CR}")

            print(f"{FEINT}{ITALIC}entry {key} is out of date{RESET}", end=f"{CR}")

    else:
        print(f"{FEINT}{ITALIC}missing cache for {key}{RESET}", end=f"{CLEAR_EOL}{CR}")


def update_cache(key, timestamp: Path, table: str, row: t.Dict[str, t.Union[str, int]]) -> None:
    cache = get_cache()
    db = cache["db"]

    sql = f"insert or replace into `{table}` (key,mtime_ns"
    values = [str(key), timestamp.stat().st_mtime_ns]

    for k, v in row.items():
        sql += f",`{k}`"
        values.append(v)

    sql += ") values (?,?"
    sql += ",?" * len(row)
    sql += ")"

    db.execute(sql, values)

    db.commit()


def run(
    prog: Path,
    lang: str,
    interpreter: t.Union[None, str],
    file: Path,
    solution: t.List,
    warmup: bool,
) -> t.Dict[str, t.Any]:
    if not prog.is_file():
        return

    cmd = [prog.absolute().as_posix()]

    # add the interpreter
    if interpreter:
        cmd.insert(0, interpreter)

    if warmup and lang == "Rust":
        # under macOS, the first launch of a Rust program is slower (why ???)
        subprocess.call(cmd + ["--help"], stdout=subprocess.DEVNULL)

    cmd.append(file.absolute().as_posix())

    start = time.time_ns()
    out = subprocess.run(cmd, stdout=subprocess.PIPE)
    elapsed = time.time_ns() - start
    answers = " ".join(out.stdout.decode().strip().split("\n"))

    status = "unknown"
    if solution:
        solution = " ".join(solution.read_text().strip().split("\n"))
        if answers == solution:
            status = "ok"
        else:
            status = "error"
    else:
        if answers == "":
            status = "missing"
        else:
            status = "unknown"

    result = {"elapsed": elapsed, "status": status, "answers": answers}

    with Path("run.log").open("at") as f:
        line = f"{datetime.now()} {lang} {cmd} {elapsed/1e9} {status} '{answers}'"
        line = line.replace(Path(__file__).parent.parent.as_posix() + "/", "")
        print(line, file=f)

    return result


def make(year: Path, source: Path, dest: Path, cmd: str):
    if not source.is_file():
        return

    build = year / "build"
    build.mkdir(parents=True, exist_ok=True)

    output = build / dest

    if output.is_file() and output.stat().st_mtime_ns >= source.stat().st_mtime_ns:
        return

    cmdline = f"{cmd} -o {output} -Wall -Wextra -O3 -DSTANDALONE -I{source.parent} {source}"
    print(f"{CR}{cmdline}", end="")
    subprocess.check_call(cmdline, shell=True)


def build_all(filter_year: int):
    for year in range(2015, 2024):
        if filter_year != 0 and year != filter_year:
            continue
        year = Path(str(year))
        if not year.is_dir():
            continue
        m = year / "Cargo.toml"
        if year.is_dir() and m.is_file():
            print(f"{FEINT}{ITALIC}cargo build {m}{RESET}", end=f"{CLEAR_EOL}{CR}")
            subprocess.check_call(["cargo", "build", "--manifest-path", m, "--release", "--quiet"])

        for day in range(1, 26):
            src = year / f"day{day}" / f"day{day}.c"
            if src.is_file():
                print(f"{FEINT}{ITALIC}compile {src}{RESET}", end=f"{CLEAR_EOL}{CR}")
                make(year, src, f"day{day}_c", "cc -std=c11")

            src = year / f"day{day}" / f"day{day}.cpp"
            if src.is_file():
                print(f"{FEINT}{ITALIC}compile {src}{RESET}", end=f"{CLEAR_EOL}{CR}")
                make(year, src, f"day{day}_cpp", "c++ -std=c++17")


def load_data(filter_year, filter_user, filter_yearday):
    inputs = defaultdict(dict)
    solutions = defaultdict(dict)

    for f in Path("data").rglob("*.in"):
        if f.name.startswith("._"):
            continue

        assert len(f.parts) == 4

        user = f.parent.parent.name

        if filter_user == "me":
            if not user.isdigit():
                continue
        elif filter_user and user != filter_user:
            continue

        year = int(f.parent.name)
        day = int(f.stem)

        if filter_year != 0 and year != filter_year:
            continue

        if filter_yearday and f"{year}:{day}" in filter_yearday:
            continue

        key = f"{year}:{day}:{user}"

        e = check_cache(key, f, "inputs", ("crc32",))
        if e:
            crc = e["crc32"]
        else:
            # crc = hex(crc32(f.read_bytes().strip()) & 0xFFFFFFFF)
            crc = hashlib.sha256(f.read_bytes().strip()).hexdigest()

            update_cache(key, f, "inputs", {"crc32": crc})

        if crc not in inputs[year, day]:
            inputs[year, day][crc] = f

        s = f.with_suffix(".ok")
        if s.is_file():
            solutions[year, day][crc] = s

    save_cache()
    return inputs, solutions


def run_day(
    year: int,
    day: int,
    mday: str,
    day_inputs: t.Dict,
    day_sols: t.Dict,
    languages: dict,
    problems: t.Set,
    refresh: bool,
    dry_run: bool,
):
    elapsed = defaultdict(list)

    warmup = defaultdict(lambda: True)

    day_suffix = mday.removeprefix(str(day))
    name_max_len = 16 - len(day_suffix)

    for crc, file in sorted(day_inputs.items(), key=itemgetter(1)):
        input_name = file.parent.parent.name.removeprefix("tmp-")[:16]
        prefix = f"[{year}-{day:02d}{day_suffix}] {input_name[:name_max_len]:<{name_max_len}}"

        if day % 2 == 1:
            prefix = f"{BLUE}{prefix}{RESET}"
        else:
            prefix = f"{CYAN}{prefix}{RESET}"

        results = set()

        for lang, (pattern, interpreter) in languages.items():
            prog = Path(pattern.format(year=year, day=mday))
            key = ":".join(map(str, (year, day, crc, prog, lang.lower())))

            if refresh:
                e = None
                in_cache = False
            else:
                e = check_cache(key, prog, "solutions", ("elapsed", "status", "answers"), dry_run)
                in_cache = e is not None

            if not in_cache and not dry_run:
                e = run(prog, lang, interpreter, file, day_sols.get(crc), warmup[lang])

                if e:
                    warmup[lang] = False
                    update_cache(key, prog, "solutions", e)

            if not e:
                continue

            if e["status"] != "ok":
                info = f" {file}"
            else:
                info = ""

            status_color = {"missing": MAGENTA, "unknown": GRAY, "error": RED, "ok": GREEN}[e["status"]]

            answers = e["answers"]

            line = (
                f"{CR}{RESET}{CLEAR_EOL}"
                f"{prefix}"
                f" {YELLOW}{lang:<7}{RESET}:"
                f" {status_color}{e['status']:7}{RESET}"
                f" {WHITE}{e['elapsed']/1e9:7.3f}s"
                f" {GRAY}{'☽' if in_cache else ' '}"
                f" {status_color}{str(answers):<40}{RESET}"
                f"{info}"
            )
            print(line)

            if e["status"] == "missing" or e["status"] == "error":
                problems.append(line)

            if not in_cache and e["elapsed"] / 1e9 > 5:
                save_cache()

            results.add(answers)

            elapsed[lang].append(e["elapsed"] / 1e9)

        if len(results) > 1:
            line = f"{prefix} {RED}{BLINK}MISMATCH BETWEEN SOLUTIONS{RESET}"
            print(line)
            problems.append(line)

    nb_samples = set(len(t) for _, t in elapsed.items())
    assert len(nb_samples) == 1 or len(nb_samples) == 0
    nb_samples = 0 if len(nb_samples) == 0 else nb_samples.pop()

    return dict((lang, sum(t) / len(t)) for lang, t in elapsed.items()), nb_samples


def get_languages(filter_lang: t.Iterable[str]) -> t.Dict[str, t.Tuple[str, t.Union[str, None]]]:
    filter_lang = set(map(str.casefold, filter_lang or ()))

    languages = {}
    for lang, v in LANGUAGES.items():
        if lang in INTERPRETERS:
            for lang2, interpreter in INTERPRETERS[lang].items():
                if filter_lang and lang2.casefold() not in filter_lang:
                    continue

                if "/" not in interpreter and "\\" not in interpreter:
                    interpreter = shutil.which(interpreter)
                    languages[lang2] = (v, interpreter)
                else:
                    interpreter = Path(interpreter).expanduser().absolute()
                    if interpreter.is_file() and (interpreter.stat().st_mode & os.X_OK) != 0:
                        languages[lang2] = (v, interpreter.as_posix())
                    else:
                        # print(f"language {lang2} : interpreter {interpreter} not found")
                        pass
        else:
            if filter_lang and lang.casefold() not in filter_lang:
                continue
            languages[lang] = (v, None)

    return languages


def install_venv(interpreter: Path):
    try:
        slug = 'import sys;print(((hasattr(sys, "subversion") and getattr(sys, "subversion")) or ("Py",))[0] + f"{sys.version_info.major}.{sys.version_info.minor}")'

        slug = subprocess.check_output([interpreter, "-c", slug]).decode().strip()

        venv = f".venv/{slug.lower()}"

        # subprocess.check_call([interpreter, "-mensurepip"])
        subprocess.check_output([interpreter, "-mvenv", venv])
        subprocess.check_call(
            [
                f"{venv}/bin/python3",
                "-mpip",
                "install",
                "--no-input",
                "--quiet",
                "--upgrade",
                "pip",
            ]
        )
        subprocess.check_call(
            [
                f"{venv}/bin/python3",
                "-mpip",
                "install",
                "--no-input",
                "--quiet",
                "--upgrade",
                "-r",
                "scripts/requirements.txt",
            ]
        )

        print(f"Virtual environment for {MAGENTA}{interpreter}{RESET} installed into: {GREEN}{venv}{RESET}")

    except subprocess.CalledProcessError as e:
        print(e)


def main():
    parser = argparse.ArgumentParser(formatter_class=lambda prog: argparse.HelpFormatter(prog, max_help_position=28))

    parser.add_argument("--venv", type=Path, help="install virtual environment")
    parser.add_argument("-l", "--language", type=str, action="append", metavar="LANG", help="filter by language")
    parser.add_argument("-x", "--exclude", type=str, action="append", metavar="Y:D", help="exclude day")
    parser.add_argument("-r", "--refresh", action="store_true", help="relaunch solutions")
    parser.add_argument("-n", "--dry-run", action="store_true", help="do not run")
    parser.add_argument("--no-build", action="store_true", help="do not build")
    parser.add_argument("-u", "--user", dest="filter_user", metavar="USER", type=str, help="filter by user id")
    parser.add_argument("--no-slow", action="store_true", help="exclude slow solutions")
    parser.add_argument("--no-64", action="store_true", help="exclude 64-bit only solutions")
    parser.add_argument("n", type=int, nargs="*", help="filter by year or year/day")

    args = parser.parse_args()

    try:
        problems = []
        stats_elapsed = dict()

        os.chdir(Path(__file__).parent.parent)

        if args.venv:
            return install_venv(args.venv)

        languages = get_languages(args.language)

        args.exclude = args.exclude or []
        if args.no_slow:
            args.exclude.extend(
                " -x 2016:5 -x 2016:11 -x 2016:14 -x 2016:23"
                " -x 2018:21 -x 2018:23 "
                " -x 2019:25"  # no generic solution
                " -x 2020:15"
                " -x 2021:18"
                " -x 2022:15"
                " -x 2023:5 -x 2023:10 -x 2023:23".split()
            )
        if args.no_64:
            args.exclude.extend(
                " -x 2016:9 -x 2016:15" " -x 2022:11 -x 2022:20" " -x 2020:23" " -x 2023:8 -x 2023:11 -x 2023:21"
            )

        filter_year = 0 if len(args.n) == 0 else int(args.n.pop(0))
        filter_day = set(args.n)

        if not args.no_build:
            build_all(filter_year)
            print(end=f"{CR}{CLEAR_EOL}")

        inputs, sols = load_data(filter_year, args.filter_user, args.exclude)

        for year in range(2015, 2024):
            if filter_year != 0 and year != filter_year:
                continue

            for day in range(1, 26):
                if filter_day and day not in filter_day:
                    continue

                for mday in list(Path(f"{year}").glob(f"day{day}")) + list(Path(f"{year}").glob(f"day{day}_*")):
                    mday = mday.name.removeprefix("day")

                    elapsed, nb_samples = run_day(
                        year,
                        day,
                        mday,
                        inputs[year, day],
                        sols[year, day],
                        languages,
                        problems,
                        args.refresh,
                        args.dry_run,
                    )
                    save_cache()

                    if elapsed:
                        print(
                            f"{CLEAR_EOL}--> ",
                            " | ".join((f"{lang} : {t:.3f}s" for lang, t in elapsed.items())),
                            f"{FEINT}({nb_samples} input{'s' if nb_samples>1 else ''}){RESET}",
                        )
                        for lang, e in elapsed.items():
                            stats_elapsed[year, day, lang] = e

            if filter_year == 0:
                print(
                    "=========================="  # prefix
                    " ==============================="  # language, status
                    " ========================================"  # answers
                    " =================================="  # input path
                )

    except KeyboardInterrupt:
        pass

    # except Exception as e:
    #     print(f"{RED}ERROR {e}{RESET}")

    finally:
        if stats_elapsed:
            print()
            print("ELAPSED TIME:")
            languages = sorted(set(map(itemgetter(2), stats_elapsed.keys())))
            for lang in languages:
                t = list(t for (_, _, i), t in stats_elapsed.items() if lang == i)
                n = len(t)
                t = sum(t)
                print(
                    f"{YELLOW}{lang:<10}{RESET}"
                    f" : {GREEN}{t:7.3f}s{RESET} for {WHITE}{n:3}{RESET} puzzle{'s' if n>1 else ' '},"
                    f" average: {GREEN}{t/n:7.3f}s{RESET}"
                )

            print()
            print("LANGUAGES COMPARISON:")
            puzzles = set(map(itemgetter(0, 1), stats_elapsed.keys()))
            for lang1, lang2 in itertools.combinations(languages, 2):
                n, t1, t2 = 0, 0, 0
                for y, d in puzzles:
                    t = dict((lang, t) for (yy, dd, lang), t in stats_elapsed.items() if (yy, dd) == (y, d))
                    if lang1 in t and lang2 in t:
                        n += 1
                        t1 += t[lang1]
                        t2 += t[lang2]
                if n > 0:
                    if t2 < t1:
                        t1, t2 = t2, t1
                        lang1, lang2 = lang2, lang1
                    faster = t2 / t1
                    print(
                        f"{YELLOW}{lang1:<7}{RESET}"
                        f" vs. {YELLOW}{lang2:<7}{RESET}:"
                        f" {GREEN}{t1/n:7.3f}s{RESET} vs. {GREEN}{t2/n:7.3f}s{RESET}"
                        f" (x {faster:4.1f} faster) on {WHITE}{n:3}{RESET} puzzle{'s' if n>1 else ''}"
                    )

        if problems:
            print()
            print("LIST OF PROBLEMS:")
            for problem in problems:
                print(problem)


if __name__ == "__main__":
    main()
