#!/usr/bin/env python3

import argparse
import hashlib
import itertools
import logging
import os
import random
import re
import shutil
import sqlite3
import subprocess
import sys
import time
import typing as t
from collections import defaultdict
from dataclasses import dataclass
from datetime import datetime
from functools import lru_cache
from operator import itemgetter
from pathlib import Path

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
BOLD = "\033[1m"
FEINT = "\033[2m"
ITALIC = "\033[3m"
BLINK = "\033[6m"
CLEAR_EOL = "\033[0K"
CR = "\r"
TRANSIENT = f"{CLEAR_EOL}{CR}"


@lru_cache(maxsize=None)
def aoc_available_puzzles(
    year: t.Optional[int] = None, seconds: t.Optional[float] = None
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


class Env:
    """
    Variables that can be overridden by environment variables.
    """

    AOC_TARGET_DIR = os.environ.get("AOC_TARGET_DIR", "target")
    CARGO_TARGET_DIR = os.environ.get("CARGO_TARGET_DIR", "target")
    AOC_VERBOSE = bool(os.environ.get("AOC_VERBOSE", False))
    CC = os.environ.get("CC", "cc")
    CXX = os.environ.get("CXX", "c++")


LANGUAGES = {
    "Rust": "src/year{year}/day{day}/day{day}.rs",
    "Python": "src/year{year}/day{day}/day{day}.py",
    "C": "{AOC_TARGET_DIR}/build/year{year}/day{day}_c",
    "C++": "{AOC_TARGET_DIR}/build/year{year}/day{day}_cpp",
    "Lua": "src/year{year}/day{day}/day{day}.lua",
    "JavaScript": "src/year{year}/day{day}/day{day}.js",
    "Ruby": "src/year{year}/day{day}/day{day}.rb",
    "Perl": "src/year{year}/day{day}/day{day}.pl",
    "Bash": "src/year{year}/day{day}/day{day}.bash",
    "Java": "{AOC_TARGET_DIR}/build/year{year}/day{day}.class",
    "Go": "{AOC_TARGET_DIR}/build/year{year}/day{day}_go",
    "C#": "{AOC_TARGET_DIR}/build/year{year}/day{day}_cs.exe",
    "Swift": "{AOC_TARGET_DIR}/build/year{year}/day{day}_swift",
    "Tcl": "src/year{year}/day{day}/day{day}.tcl",
}

INTERPRETERS = {
    "Python": {
        "Python": (
            "{AOC_TARGET_DIR}/venv/python/bin/python3",
            "/venv/python/bin/python3",
        ),
        "PyPy": "{AOC_TARGET_DIR}/venv/pypy3.10/bin/python3",
        "Py3.10": "{AOC_TARGET_DIR}/venv/py3.10/bin/python3",
        "Py3.11": "{AOC_TARGET_DIR}/venv/py3.11/bin/python3",
        "Py3.12": "{AOC_TARGET_DIR}/venv/py3.12/bin/python3",
        "Py3.13": "{AOC_TARGET_DIR}/venv/py3.13/bin/python3",
        "Py3.13t": "{AOC_TARGET_DIR}/venv/py3.13t/bin/python3",
        "Py3.14": "{AOC_TARGET_DIR}/venv/py3.14/bin/python3",
        "Py3.14t": "{AOC_TARGET_DIR}/venv/py3.14t/bin/python3",
    },
    "JavaScript": {
        "NodeJS": "node",
        "BunJS": "bun",
    },
    "Lua": {"Lua": "lua5.4"},
    "Ruby": {"Ruby": "ruby"},
    "Perl": {"Perl": "perl"},
    "Tcl": {"Tcl": "tclsh"},
    "Bash": {"Bash": "bash"},
}

LANGUAGES_VERSIONS = {
    "Rust": "rustc --version",
    "Go": "go version",
    "C": "{CC} --version",
    "C++": "{CXX} --version",
    "Swift": "swiftc -version",
    "Java": "javac -version",
    "C#": "mcs --version",
    "Python": "{interpreter} -VV",
    "Ruby": "{interpreter} --version",
    "Lua": "{interpreter} -v",
    "JavaScript": {
        "NodeJS": "{interpreter} -v",
        "BunJS": "{interpreter} --version",
    },
    "Perl": "{interpreter} -v",
    "Tcl": "echo 'puts [info patchlevel]' | {interpreter}",
    "Bash": "{interpreter} --version",
}


def get_language_version(puzzle_lang: str) -> str:
    """
    Return a human-readable version string for the given puzzle language/variant.
    """

    for language, variants in INTERPRETERS.items():
        # language = Python
        # variants = Py3.11, Py3.12, ...
        for variant, interpreters in variants.items():
            if variant == puzzle_lang:
                # language should be in the "show version" dict
                if language not in LANGUAGES_VERSIONS:
                    continue

                # interpreter could be a string or an iterable of strings
                if isinstance(interpreters, str):
                    interpreters = (interpreters,)

                # get the first available interpreter
                for interpreter in interpreters:
                    interpreter = interpreter.format(**vars(Env))
                    interpreter = shutil.which(interpreter)
                    if interpreter and os.path.isfile(interpreter):
                        break
                else:
                    return ""

                cmd = LANGUAGES_VERSIONS[language]
                if isinstance(cmd, dict):
                    for v, cmd_v in cmd.items():
                        if v == variant:
                            cmd = cmd_v
                            break
                    else:
                        # variant has no "show version" command
                        return ""

                cmd = cmd.format(interpreter=interpreter)
                cmd = cmd.format(**vars(Env))
                try:
                    lang_version = (
                        subprocess.check_output(
                            cmd,
                            shell=True,
                            stderr=subprocess.DEVNULL,
                        )
                        .decode()
                        .strip()
                        .splitlines()[0]
                    )
                    return lang_version
                except Exception:
                    return ""

    if puzzle_lang in LANGUAGES_VERSIONS:
        cmd = LANGUAGES_VERSIONS[puzzle_lang]
        cmd = cmd.format(**vars(Env))
        try:
            lang_version = (
                subprocess.check_output(
                    cmd,
                    shell=True,
                    stderr=subprocess.DEVNULL,
                )
                .decode()
                .strip()
                .splitlines()[0]
            )
            return lang_version
        except Exception:
            return ""

    return ""


def print_log(line: str = None, end: str = None):
    if Env.AOC_VERBOSE:
        if line:
            line = line.replace(CLEAR_EOL, "").replace(CR, "")
            logging.debug(line)
    else:
        print(line, end=end)


class CacheKey:
    def __str__(self):
        string = ":".join(str(v) for k, v in vars(self).items() if not k.startswith("_"))
        string = re.sub(r":([a-f\d]{9}[a-f\d]+):", lambda m: f":{m[1][:8]}…:", string)
        return string

    def __iter__(self):
        for k, v in vars(self).items():
            if not k.startswith("_"):
                yield k, v

    def _columns_values(self):
        columns = list()
        values = list()
        for k, v in vars(self).items():
            if not k.startswith("_"):
                columns.append(k)
                values.append(v)
        return columns, values

    def _values(self):
        return list(v for k, v in vars(self).items() if not k.startswith("_"))

    def _select(self):
        columns, values = self._columns_values()
        sql = " and ".join(f"`{column}`=?" for column in columns)
        return sql, values


@dataclass
class InputKey(CacheKey):
    year: int
    day: int
    user: str


@dataclass
class SolutionKey(CacheKey):
    year: int
    day: int
    crc: str
    prog: str
    lang: str


def get_cache(cache_file: Path = None):
    """
    Retrieve the cache instance from memory or load it from disk.
    """

    cache = globals().get("_cache")
    if cache is None:
        if cache_file is None:
            if "AOC_TARGET_DIR" in os.environ:
                cache_file = Path(os.environ["AOC_TARGET_DIR"]) / "cache.db"

        if cache_file is None:
            cache_file = Path("data") / "cache.db"

        logging.debug(f"cache: {cache_file.resolve()}")

        cache_db = sqlite3.connect(cache_file)

        cache_db.executescript("create table if not exists runday_version (version integer)")

        DB_VERSION = 2

        runday_version = cache_db.execute("select version from runday_version").fetchone()
        if runday_version is None or runday_version[0] < DB_VERSION:
            logging.info(f"create database tables v{DB_VERSION}")

            # database version
            cache_db.execute("delete from runday_version")
            cache_db.execute("insert into runday_version values (?)", (DB_VERSION,))

            # table solutions
            cache_db.executescript(
                "drop index if exists solutions_idx;"
                "drop table if exists solutions;"
                "create table if not exists solutions ("
                " year integer not null,"  # key
                " day integer not null,"  # key
                " crc text not null,"  # key
                " prog text not null,"  # key
                " lang text not null,"  # key
                " updated date,"  # cache control
                " mtime_ns integer,"  # cache control
                " sha256 text,"  # cache control
                " elapsed int,"  # elapsed time in nanoseconds
                " status text,"
                " answers text"
                ");"
                "create unique index solutions_idx on solutions (year,day,crc,prog,lang);"
            )

            # table inputs
            cache_db.executescript(
                "drop index if exists inputs_idx;"
                "drop table if exists inputs;"
                "create table if not exists inputs ("
                " year integer not null,"  # key
                " day integer not null,"  # key
                " user text not null,"  # key
                " updated date,"  # cache control
                " mtime_ns integer,"  # cache control
                " sha256 text,"  # cache control
                " crc text"
                ");"
                "create unique index inputs_idx on inputs (year,day,user);"
            )

            # view
            cache_db.executescript(
                "drop view if exists user_solutions;"
                "create view user_solutions as"
                " select i.user,s.year,s.day,s.lang,s.elapsed,s.updated,s.prog,s.mtime_ns"
                " from solutions s,inputs i"
                " where s.crc=i.crc and s.year=i.year and s.day=i.day and (s.status='ok' or s.status='unknown');"
            )

        cache = {"db": cache_db, "modified": False}
        globals()["_cache"] = cache

        logging.debug(f"using cache file {cache_file}")

    return cache


def check_cache(
    key: CacheKey,
    timestamp_file: Path,
    table: str,
    columns: t.Iterable[str],
    no_age_check=False,
    having=None,
):
    cache = get_cache()
    db = cache["db"]
    db.row_factory = sqlite3.Row

    key_columns, key_values = key._columns_values()
    where = " and ".join(f"`{column}`=?" for column in key_columns)
    sql = f"select * from `{table}` where {where}"

    if having:
        sql += " group by " + ",".join(f"`{column}`" for column in key_columns)
        sql += " having " + having

    cursor = db.execute(sql, key_values)

    row = cursor.fetchone()
    if row:
        timestamp = timestamp_file.stat().st_mtime_ns

        if row["mtime_ns"] == timestamp or no_age_check:
            return dict((column, row[column]) for column in columns)

        # mtime changed: verify sha256 if enabled
        if row["sha256"] is not None:
            sha256 = hashlib.sha256(Path(timestamp_file).read_bytes()).hexdigest()
            if row["sha256"] == sha256:
                return dict((column, row[column]) for column in columns)

        else:
            print_log(f"{FEINT}{ITALIC}entry {key} is out of date{RESET}", end=TRANSIENT)

    else:
        print_log(f"{FEINT}{ITALIC}missing cache for {key}{RESET}", end=TRANSIENT)


def prune_cache(key: CacheKey, table: str) -> None:
    cache = get_cache()
    db = cache["db"]
    where, key_values = key._select()
    db.execute(f"delete from `{table}` where {where}", key_values)


def update_cache(
    key: CacheKey,
    timestamp_file: Path,
    table: str,
    row: t.Dict[str, t.Union[str, int]],
    use_sha256: bool = False,
) -> None:
    cache = get_cache()
    db = cache["db"]

    mtime_ns = timestamp_file.stat().st_mtime_ns
    if use_sha256:
        sha256 = hashlib.sha256(Path(timestamp_file).read_bytes()).hexdigest()
    else:
        sha256 = None

    sql = f"insert or replace into `{table}` (updated,mtime_ns,sha256"
    values = [datetime.now().isoformat(), mtime_ns, sha256]

    placeholders = ""
    for column, value in key:
        sql += f",`{column}`"
        placeholders += ",?"
        values.append(value)

    for k, v in row.items():
        sql += f",`{k}`"
        values.append(v)

    sql += ") values (?,?,?"  # placeholders for updated,mtime_ns,sha256
    sql += placeholders  # placeholders for the key columns
    sql += ",?" * len(row)  # placeholders the values
    sql += ")"

    db.execute(sql, values)

    db.commit()


def make(year: int, source: Path, dest: Path, language: str, disable_language: t.Callable):
    if not source.is_file():
        return

    build_dir = Path(f"{Env.AOC_TARGET_DIR}/build/year{year}")
    build_dir.mkdir(parents=True, exist_ok=True)

    output = build_dir / dest

    if output.is_file() and output.stat().st_mtime_ns >= source.stat().st_mtime_ns:
        return

    if language == "C":
        cmd = "{CC} -std=c11".format(CC=Env.CC)
        cmdline = f"{cmd} -o {output} -Wall -Wextra -Werror -O3 -DSTANDALONE -I{source.parent} {source}"
    elif language == "C++":
        cmd = "{CXX} -std=c++23".format(CXX=Env.CXX)
        cmdline = f"{cmd} -o {output} -Wall -Wextra -Werror -O3 -DSTANDALONE -I{source.parent} {source}"
    elif language == "Java":
        cmdline = f"javac -d {build_dir} {source}"
    elif language == "Go":
        cmdline = f"go build -o {output} {source}"
    elif language == "C#":
        cmdline = f"mcs -out:{output} {source}"
    elif language == "Swift":
        cmdline = f"swiftc -o {output} {source}"
    else:
        raise ValueError(language)

    print_log(f"{CR}{CLEAR_EOL}{cmdline}", end="")
    try:
        subprocess.check_call(cmdline, shell=True)
    except subprocess.CalledProcessError as e:
        print_log(f"{CR}{CLEAR_EOL}{RED}FAIL {year} {dest} {language}", end="")
        if e.returncode == 127:  # not found
            disable_language(language)


def build_all(filter_year: int, filter_lang: t.Iterable[str], languages: dict):
    def disable_language(lang: str):
        for k in languages:
            if k.casefold() == lang.casefold():
                logging.debug(f"disabling {lang} because error")
                del languages[k]
                break

    def is_available(lang: str):
        if not filter_lang or lang in filter_lang:
            for k in languages:
                if lang == k.casefold():
                    return True
        return False

    if is_available("rust"):
        try:
            m = Path(__file__).parent.parent / "Cargo.toml"
            if m.is_file():
                env_copy = os.environ.copy()
                # env_copy["RUSTFLAGS"] = "-C target-cpu=native"
                print_log(f"{FEINT}{ITALIC}cargo build {m}{RESET}", end=TRANSIENT)
                subprocess.check_call(["cargo", "build", "--manifest-path", m, "--release", "--quiet"], env=env_copy)

        except FileNotFoundError:
            print_log(
                f"{RED}Rust and Cargo are requited. Install them with:{RESET}"
                f" {YELLOW}curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh{RESET}"
            )
            disable_language("rust")

    for year in aoc_available_puzzles():
        if filter_year != 0 and year != filter_year:
            continue

        for day in range(1, 26):
            src = Path(f"src/year{year}/day{day}/day{day}")

            if is_available("c"):
                src = src.with_suffix(".c")
                if src.is_file():
                    print_log(f"{FEINT}{ITALIC}compile {src}{RESET}", end=TRANSIENT)
                    make(year, src, f"day{day}_c", "C", disable_language)

            if is_available("c++"):
                src = src.with_suffix(".cpp")
                if src.is_file():
                    print_log(f"{FEINT}{ITALIC}compile {src}{RESET}", end=TRANSIENT)
                    make(year, src, f"day{day}_cpp", "C++", disable_language)

            if is_available("java"):
                src = src.with_suffix(".java")
                if src.is_file():
                    print_log(f"{FEINT}{ITALIC}compile {src}{RESET}", end=TRANSIENT)
                    make(year, src, f"day{day}.class", "Java", disable_language)

            if is_available("go"):
                src = src.with_suffix(".go")
                if src.is_file():
                    print_log(f"{FEINT}{ITALIC}compile {src}{RESET}", end=TRANSIENT)
                    make(year, src, f"day{day}_go", "Go", disable_language)

            if is_available("c#"):
                src = src.with_suffix(".cs")
                if src.is_file():
                    print_log(f"{FEINT}{ITALIC}compile {src}{RESET}", end=TRANSIENT)
                    make(year, src, f"day{day}_cs.exe", "C#", disable_language)

            if is_available("swift"):
                src = src.with_suffix(".swift")
                if src.is_file():
                    print_log(f"{FEINT}{ITALIC}compile {src}{RESET}", end=TRANSIENT)
                    make(year, src, f"day{day}_swift", "Swift", disable_language)


def load_data(filter_year, filter_user, filter_yearday, with_answers):
    inputs = defaultdict(dict)
    answers = defaultdict(dict)

    all_inputs = list(sorted(Path("data").rglob("*.in")))

    me_user = all_inputs[0].parent.parent.name if len(all_inputs) > 0 else None

    for input in all_inputs:
        if input.name.startswith("._"):
            continue

        if len(input.parts) > 4:
            # files must be
            #   data/<user>/<year>/<day>.in
            #   data/<user>/<year>/<day>.ok
            continue

        user = input.parent.parent.name

        if filter_user == "me":
            if user != me_user:
                continue
        elif filter_user == "mine":
            if not user.isdigit():
                continue
        elif filter_user and user != filter_user and not user[user.find("-") :].startswith(f"-{filter_user}"):
            continue

        year = int(input.parent.name)
        day = int(input.stem)

        if filter_year != 0 and year != filter_year:
            continue

        if filter_yearday and f"{year}:{day}" in filter_yearday:
            continue

        answer = input.with_suffix(".ok")
        if not answer.is_file():
            answer = None

        if not answer and with_answers:
            continue

        key = InputKey(year, day, user)

        e = check_cache(key, input, "inputs", ("crc",))
        if e:
            crc = e["crc"]
        else:
            crc = hashlib.sha256(input.read_bytes().strip()).hexdigest()

            update_cache(key, input, "inputs", {"crc": crc})

        if crc not in inputs[year, day]:
            inputs[year, day][crc] = input
            answers[year, day][crc] = answer

    return inputs, answers


def run(
    prog: Path,
    lang: str,
    interpreter: t.Union[None, str],
    file: Path,
    expected: t.List,
    nb_expected: int,
    year: int,
    day: int,
    quiet: int,
) -> t.Dict[str, t.Any]:
    """
    TODO
    """

    if lang == "Rust":
        cmd = []
        target = Env.CARGO_TARGET_DIR

        f = Path(f"{prog.parent}/{prog.stem}/{target}/release/{prog.stem}")
        if f.is_file():
            cmd.append(f)
        else:
            cmd.append(f"{target}/release/aor")
            cmd.append("-r")

            alt = re.match(r"day\d+_(\w+)$", prog.stem)
            if alt:
                cmd.append(f"{year}:{day}:{alt[1]}")
            else:
                cmd.append(f"{year}:{day}")

    else:
        if prog.suffix == ".class":
            cmd = ["java", "-cp", prog.parent, prog.stem]
        elif prog.suffix == ".exe":
            cmd = ["mono", prog.absolute().as_posix()]
        else:
            cmd = [prog.absolute().as_posix()]

        # add the interpreter
        if interpreter:
            cmd.insert(0, interpreter)

    cmd.append(file.absolute().as_posix())
    cmd.append("--elapsed")

    env = os.environ.copy()
    env["NO_COLOR"] = "1"
    cmdline = " ".join(map(str, cmd))
    cmdline = cmdline.replace(Path(__file__).parent.parent.as_posix() + "/", "")
    cmdline = cmdline.replace(Path.home().as_posix(), "~")

    if quiet == 0 or Env.AOC_VERBOSE:
        print_log(f"{FEINT}{cmdline}{RESET}", end=TRANSIENT)

    elapsed_measurement = "process"
    start = time.time_ns()

    try:
        out = subprocess.run(cmd, stdout=subprocess.PIPE, env=env)
        elapsed = time.time_ns() - start

        if out.returncode != 0:
            status = "error"
            answers = ""
        else:
            answers = out.stdout.decode().strip().splitlines()

            for i in range(len(answers) - 1, -1, -1):
                if answers[i].startswith("elapsed:"):
                    elapsed2 = answers.pop(i).removeprefix("elapsed:")

                    if elapsed2.endswith("ns"):
                        elapsed = round(elapsed2.removesuffix("ns"))
                    elif elapsed2.endswith("µs"):
                        elapsed = round(1000 * float(elapsed2.removesuffix("µs")))
                    elif elapsed2.endswith("ms"):
                        elapsed = round(10**6 * float(elapsed2.removesuffix("ms")))
                    elif elapsed2.endswith("s"):
                        elapsed = round(10**9 * float(elapsed2.removesuffix("s")))
                    else:
                        logging.fatal("unknown time suffix %s %s", elapsed, elapsed2)
                        exit()

                    elapsed_measurement = "internal"

            nb_answers = len(answers)
            answers = " ".join(answers)

            status = "unknown"
            if expected:
                expected = " ".join(expected.read_text().strip().splitlines())
                status = "ok" if answers == expected else "failed"
            else:
                status = "unknown" if nb_answers == nb_expected else "failed"

    except OSError:
        status = "error"
        answers = ""
        elapsed = 0

    result = {"elapsed": elapsed, "status": status, "answers": answers}

    run_log = Path(Env.AOC_TARGET_DIR) / "run.log"

    with run_log.open("at") as f:
        line = f"{datetime.now()} {lang} {cmd} {elapsed / 1e9} {elapsed_measurement} {status} '{answers}'"
        line = line.replace(Path(__file__).parent.parent.as_posix() + "/", "")
        print(line, file=f)

    return result


def run_day(
    year: int,
    day: int,
    mday: str,
    day_inputs: dict,
    day_answers: dict,
    languages: dict,
    problems: list[str],
    refresh: bool,
    prune: bool,
    dry_run: bool,
    terminal_columns: int,
    wait: float,
    quiet: int,
):
    elapsed_by_lang = defaultdict(list)
    improved_timing = 0
    improved_timing_gain = 0

    day_suffix = mday.removeprefix(str(day))
    name_max_len = 16 - len(day_suffix)

    # for puzzle inputs
    for crc, file in sorted(day_inputs.items(), key=itemgetter(1)):
        input_name = file.parent.parent.name
        input_name = input_name.removeprefix("tmp-")[:16]
        input_name = input_name.removeprefix("ok-")[:17]
        input_name = input_name.removeprefix("other-")[:14]

        prefix = f"[{year}-{day:02d}{day_suffix}] {input_name[:name_max_len]:<{name_max_len}}"

        if day % 2 == 1:
            prefix = f"{BLUE}{prefix}{RESET}"
        else:
            prefix = f"{CYAN}{prefix}{RESET}"

        # set to check if all answers are identical
        # answers_set = set()

        # for all available languages
        for lang, (pattern, interpreter) in languages.items():
            prog = Path(pattern.format(year=year, day=mday, AOC_TARGET_DIR=Env.AOC_TARGET_DIR))
            if not prog.is_file():
                continue

            key = SolutionKey(year, day, crc, str(prog), lang.lower())

            if prune:
                prune_cache(key, "solutions")
                continue

            e = check_cache(key, prog, "solutions", ("elapsed", "status", "answers"), no_age_check=dry_run)
            in_cache = e is not None

            timing_status = "☽" if in_cache else " "

            if (not in_cache and not dry_run) or refresh:
                nb_expected = 1 if day == 25 else 2

                cached_e = e

                e = run(prog, lang, interpreter, file, day_answers.get(crc), nb_expected, year, day, quiet)

                if e:
                    timing = e["elapsed"]

                    # if a valid solution exists in the database
                    if cached_e and cached_e["status"] in ("ok", "unknown"):
                        # if the solution is valid and faster than the cached one, update it
                        if e["status"] in ("ok", "unknown") and cached_e["elapsed"] > timing:
                            update_cache(key, prog, "solutions", e, use_sha256=True)
                            timing_status = "☀️"
                            improved_timing += 1
                            improved_timing_gain += cached_e["elapsed"] - timing
                        else:
                            # otherwise, the timing for the solution is the cached one
                            timing = cached_e["elapsed"]
                    else:
                        # no cached solution or invalid solution in the cache
                        update_cache(key, prog, "solutions", e, use_sha256=True)

                if wait is not None:
                    if quiet == 0:
                        print_log(f"{CR}{CLEAR_EOL}waiting {wait:g}s...", end="")
                    time.sleep(wait)

            elif e is not None:
                timing = e["elapsed"]

            if not e:
                continue

            if (e["status"] == "unknown" and day_answers.get(crc)) or e["status"] in ("error", "failed"):
                resolve_script = Path(Env.AOC_TARGET_DIR) / f"resolve_{e['status']}.sh"

                if not globals().get(resolve_script):
                    with resolve_script.open("wt") as f:
                        print("#!/bin/sh", file=f)
                    resolve_script.chmod(0o755)
                    globals()[resolve_script] = True

                with resolve_script.open("at") as f:
                    print(f"{__file__} --no-build -u {input_name} -l {lang} -r {year} {day}", file=f)

            answers = e["answers"]

            status_color = {
                "ok": GREEN,
                "unknown": GRAY,
                "failed": MAGENTA,
                "error": RED,
            }[e["status"]]

            line = (
                f"{CR}{RESET}{CLEAR_EOL}"
                f"{prefix}"
                f" {YELLOW}{lang:<7}{RESET}:"
                f" {status_color}{e['status']:7}{RESET}"
                f" {WHITE}{e['elapsed'] / 1e9:7.3f}s"
                f" {GRAY}{timing_status}"
            )

            if quiet > 0:
                pass
            elif terminal_columns < 60:
                print(line)
            elif terminal_columns < 130:
                print(line, f"{status_color}{str(answers)}{RESET}")
            else:
                print(line, f" {status_color}{str(answers):<50}{RESET} {FEINT}{file}{RESET}")

            if e["status"] in ("error", "failed"):
                problems.append(line)

            # answers_set.add(answers)

            elapsed_by_lang[lang].append(timing / 1e9)

        # if len(answers_set) > 1:
        #     line = f"{prefix} {RED}{BLINK}MISMATCH BETWEEN SOLUTIONS{RESET}"
        #     print(line)
        #     problems.append(line)

    samples_set = set(len(i) for i in elapsed_by_lang.values())
    if not dry_run:
        # if not dry run, all languages should have the same puzzle input count
        # if dry run, some languages may have not be run
        assert len(samples_set) == 1 or len(samples_set) == 0
    nb_samples = 0 if len(samples_set) == 0 else max(samples_set)

    return (
        dict((lang, sum(t) / len(t)) for lang, t in elapsed_by_lang.items()),
        nb_samples,
        improved_timing,
        improved_timing_gain,
    )


def get_languages(filter_lang: t.Iterable[str]) -> t.Dict[str, t.Tuple[str, t.Union[str, None]]]:
    languages = {}
    for lang, v in LANGUAGES.items():
        if lang in INTERPRETERS:
            for lang2, interpreter in INTERPRETERS[lang].items():
                if filter_lang and lang2.casefold() not in filter_lang:
                    continue

                def lookup(interpreter: str) -> str:
                    """Resolve an interpreter."""

                    if "{" in interpreter:
                        # interpreter = interpreter.format_map(globals())
                        interpreter = interpreter.format(AOC_TARGET_DIR=Env.AOC_TARGET_DIR)

                    if "/" not in interpreter and "\\" not in interpreter:
                        interpreter = shutil.which(interpreter)
                        if not interpreter:
                            return None

                        if lang == "Python":
                            hexversion = int(
                                subprocess.check_output(
                                    [interpreter, "-c", "import sys;print(sys.hexversion)"]
                                ).decode()
                            )
                            if hexversion < 0x30A0000:  # 3.10.x
                                return None

                        languages[lang2] = (v, interpreter)
                    else:
                        interpreter = Path(interpreter).expanduser().absolute()
                        if interpreter.is_file() and (interpreter.stat().st_mode & os.X_OK) != 0:
                            return interpreter.as_posix()
                        else:
                            # print(f"language {lang2} : interpreter {interpreter} not found")
                            return None

                if isinstance(interpreter, str):
                    interpreter = lookup(interpreter)
                elif isinstance(interpreter, (tuple, list)):
                    try:
                        interpreter = next(filter(None, map(lookup, interpreter)))
                    except StopIteration:
                        interpreter = None  # none of them available

                if interpreter:
                    languages[lang2] = (v, interpreter)
        else:
            if filter_lang and lang.casefold() not in filter_lang:
                continue
            languages[lang] = (v, None)

    return languages


def consistency(filter_year: int, filter_day: set[int], filter_lang: set[str]):
    try:
        import numpy as np
        import tabulate
    except ImportError:
        print("This option requires the « tabulate » and « numpy » modules.")
        exit(1)

    db = get_cache()["db"]

    elapsed_times = defaultdict(list)
    cursor = db.execute("select year,day,crc,prog,lang,elapsed from solutions where status!='error'")
    for year, day, crc, prog, lang, elapsed in cursor:
        if filter_year:
            if filter_year != int(year):
                continue
            if filter_day and int(day) not in filter_day:
                continue
        if filter_lang and lang not in filter_lang:
            continue
        if "_" in Path(prog).stem:
            continue  # ignore alternate solution
        elapsed_times[year, day, lang].append((round(elapsed / 1e9, 3), crc))
    cursor.close()

    inputs = dict()
    for user, crc in db.execute("select user,crc from inputs"):
        if crc not in inputs or user.isdigit():
            inputs[crc] = user

    rows = list()
    cmds = list()

    for k, v in sorted(elapsed_times.items()):
        a = np.array(list(map(itemgetter(0), v)))

        # coefficient of variation
        µ, σ = a.mean(), a.std()
        if µ == 0:
            continue
        cv = σ / µ

        # quartile coefficient of dispersion
        q1 = np.percentile(a, 25)
        q3 = np.percentile(a, 75)
        if q3 + q1 == 0:
            continue
        qcd = (q3 - q1) / (q3 + q1)

        cv = round(cv * 100, 1)
        qcd = round(qcd * 100, 1)

        if µ > 3 and (cv > 15 or qcd > 10):
            a.sort()
            rows.append((" ".join(map(str, k)), µ, σ, cv, qcd, a))

            for elapsed, crc in v:
                deviation = (elapsed - µ) / σ
                if deviation > 1.5:
                    year, day, lang = k
                    user = inputs[crc]
                    cmd = f"./scripts/runall.py -u {user:<35} -l {lang:<8} {year} {day:<2} -r"
                    comment = f"  # t={elapsed:7.3f} µ={µ:7.3f} d={deviation:4.1f} σ"
                    cmds.append(((year, day, user), cmd + comment))

    print(tabulate.tabulate(rows, headers=("solution", "µ", "σ", "CV %", "qcd %", "values"), tablefmt="fancy_grid"))

    if cmds:
        print()
        previous_key = None
        flip_color = 0
        for key, cmd in sorted(cmds):
            if key != previous_key:
                flip_color = 1 - flip_color
                previous_key = key
            color = [YELLOW, MAGENTA][flip_color]
            print(f"{color}{cmd}{RESET}")


def get_task_list(filter_year: int, filter_day: set[int], inputs: dict, alt: bool):
    """
    TODO.
    """
    for year in aoc_available_puzzles():
        if filter_year != 0 and year != filter_year:
            continue

        for day in aoc_available_puzzles(year):
            if filter_day and day not in filter_day:
                continue

            if (year, day) not in inputs:
                continue

            day_solutions = list(Path(f"src/year{year}").glob(f"day{day}"))

            if alt:
                day_solutions += Path(f"src/year{year}").glob(f"day{day}_*")

            for mday in day_solutions:
                mday = mday.name.removeprefix("day")

                yield (year, day, mday)


def run_task(year, day, mday: str, inputs, answers, languages, problems, args, terminal_columns, stats_elapsed):
    """
    TODO.
    """

    without_improvement_count = 0

    for loop in itertools.count(start=1):
        if args.loop < 0:
            print(f"(loop {loop})")

        elapsed, nb_samples, improved_count, improved_gain = run_day(
            year,
            day,
            mday,
            inputs[year, day],
            answers.get((year, day)),
            languages,
            problems,
            args.refresh,
            args.prune,
            args.dry_run,
            terminal_columns,
            args.wait,
            args.quiet,
        )

        if not elapsed or args.prune:
            return

        if args.quiet == 0:
            if nb_samples > 1:
                print(
                    f"{CR}{CLEAR_EOL}--> "
                    + " | ".join((f"{lang} : {t:.3f}s" for lang, t in elapsed.items()))
                    + f" {FEINT}({nb_samples} input{'s' if nb_samples > 1 else ''}){RESET}"
                )
            else:
                print(end=f"{CR}{CLEAR_EOL}")

        nb_runs = len(elapsed) * nb_samples

        for lang, e in elapsed.items():
            stats_elapsed[year, day, mday, lang] = (e, nb_samples)

        # do not repeat (many cases)
        if args.loop == 0 or args.dry_run or not args.refresh:
            return

        if args.loop < 0:
            # repeat the given number of times
            if loop <= args.loop:
                break
        else:
            # repeat the given number of times without any improvement
            elapsed_overall = sum(e * nb_samples for e in elapsed.values())
            elapsed_overall_str = (
                f"{elapsed_overall:10.6f} s" if elapsed_overall <= 0.002 else f"{elapsed_overall:7.3f} s   "
            )

            SIGNIFICANT_GAIN = 10_000  # 10 µs

            if improved_gain == 0:
                gain_str = ""
            elif improved_gain < SIGNIFICANT_GAIN:
                gain_str = f"(insignificant gain {improved_gain / 1e3:g} µs)"
            else:
                gain_str = f"(gain: {improved_gain / 1e9:10.6f} s)"

            if improved_count == 0:
                # no gain at all
                without_improvement_count += 1
                if args.quiet < 2:
                    print(
                        f"{GREEN}[{year}-{day:2}] loop: {loop:2}"
                        f" improved timings: {improved_count:3}/{nb_runs:3}"
                        f" ( {without_improvement_count} of {args.loop})"
                        f"  elapsed: {elapsed_overall_str} {gain_str}"
                        f"{RESET}"
                    )

            elif abs(improved_gain) < SIGNIFICANT_GAIN:
                # gain less than 1 µs
                without_improvement_count += 1
                if args.quiet < 2:
                    print(
                        f"{CYAN}[{year}-{day:2}] loop: {loop:2}"
                        f" improved timings: {improved_count:3}/{nb_runs:3}"
                        f" ( {without_improvement_count} of {args.loop})"
                        f"  elapsed: {elapsed_overall_str} {gain_str}"
                        f"{RESET}"
                    )

            else:
                # start over: there is a significative gain in elapsed time
                if args.quiet < 2:
                    print(
                        f"{RED}[{year}-{day:2}] loop: {loop:2}"
                        f" improved timings: {improved_count:3}/{nb_runs:3}"
                        " (restart)"
                        f"  elapsed: {elapsed_overall_str} {gain_str}"
                        f"{RESET}"
                    )
                without_improvement_count = 0

            if without_improvement_count >= args.loop:
                break


def set_auto_filter(args):
    if len(args.n) != 0:
        return

    cwd = Path.cwd()

    if len(cwd.parts) >= 2 and cwd.parts[-2] == "src":
        year = cwd.parts[-1]
        if year.startswith("year") and year[4:].isdigit():
            args.n.append(int(year[4:]))

    elif len(cwd.parts) >= 3 and cwd.parts[-3] == "src":
        year = cwd.parts[-2]
        day = cwd.parts[-1]

        if year.startswith("year") and year[4:].isdigit() and day.startswith("day"):
            day_alt = day[3:].split("_", maxsplit=1)

            if day_alt[0].isdigit():
                args.n.append(int(year[4:]))
                args.n.append(int(day_alt[0]))

                if len(day_alt) == 2:
                    args.alt = True


def main():
    parser = argparse.ArgumentParser(formatter_class=lambda prog: argparse.HelpFormatter(prog, max_help_position=28))

    parser.add_argument("-v", "--verbose", action="store_true", help="Be more verbose")
    parser.add_argument("-q", "--quiet", action="count", default=0, help="Be more quiet")
    parser.add_argument("--cache", type=Path, help="Cache database")
    parser.add_argument("--working-dir", type=Path, help=argparse.SUPPRESS)

    # parser.add_argument("--venv", type=Path, help="create and install virtual environment")
    # parser.add_argument("--reqs", action="store_true", help="install requirements into virtual environments")
    parser.add_argument("-c", "--consistency", action="store_true", help="Verify duration consistency")

    parser.add_argument("-u", "--user", dest="filter_user", metavar="USER", type=str, help="Filter by user id")
    parser.add_argument("--me", action="store_true", help="Only first user id")
    parser.add_argument("-l", "--language", type=str, action="append", metavar="LANG", help="Filter by language")
    parser.add_argument("-x", "--exclude", type=str, action="append", metavar="Y:D", help="Exclude day")
    parser.add_argument("--verified", action="store_true", help="Only inputs with solution")
    parser.add_argument("--no-slow", action="store_true", help="Exclude slow solutions")
    parser.add_argument("--alt", action="store_true", help="Run alternarive solutions too")

    parser.add_argument("-r", "--refresh", action="store_true", help="Relaunch solutions")
    parser.add_argument("-n", "--dry-run", action="store_true", help="Do not run")
    parser.add_argument("--no-build", action="store_true", help="Do not build")
    parser.add_argument("--prune", action="store_true", help="Prune timings BEFORE run")
    parser.add_argument("-w", "--wait", type=float, help="Wait seconds between each solution")
    parser.add_argument("-s", "--shuffle", action="store_true", help="Shuffle before running solutions")
    parser.add_argument(
        "--loop", type=int, default=0, help="Repeat (positive: loops without improvement, negative: loops)"
    )

    parser.add_argument("-C", "--comparison", action="store_true", help="Show languages commparison")

    parser.add_argument("n", type=int, nargs="*", help="Filter by year or year/day")

    args = parser.parse_args()

    if args.verbose:
        Env.AOC_VERBOSE = True
        logging.basicConfig(format="\033[2m%(asctime)s - %(levelname)s - %(message)s\033[0m", level=logging.DEBUG)
    else:
        logging.basicConfig(format="\033[2m%(asctime)s - %(levelname)s - %(message)s\033[0m", level=logging.INFO)

    if args.cache:
        get_cache(args.cache.resolve())

    if not sys.stdout.isatty() and os.environ.get("CLICOLOR_FORCE") != "1":
        global RED, GREEN, BLUE, DARK_GREEN, GRAY, MAGENTA, CYAN, WHITE, YELLOW
        RED = GREEN = BLUE = DARK_GREEN = GRAY = MAGENTA = CYAN = WHITE = YELLOW = ""

        global RESET, FEINT, ITALIC, BLINK, CLEAR_EOL, CR, TRANSIENT
        RESET = FEINT = ITALIC = BLINK = CLEAR_EOL = ""
        TRANSIENT = "\n"
        CR = ""

    # the terminal size
    try:
        terminal_columns = os.get_terminal_size().columns
    except OSError:
        terminal_columns = 132
        pass

    if args.working_dir and args.working_dir.is_dir():
        logging.debug(f"set working directory to: {args.working_dir}")
        os.chdir(args.working_dir)

    try:
        if args.me:
            args.filter_user = "me"

        problems = []
        stats_elapsed = dict()

        # if in subdirectoy "src/year<year>/day<day>" set the filter
        set_auto_filter(args)

        os.chdir(Path(__file__).parent.parent)

        # prepare the filtering by date
        filter_year = 0 if len(args.n) == 0 else int(args.n.pop(0))
        filter_day = set(args.n)

        # prepare the filtering by language
        filter_lang = set(map(str.casefold, args.language or ()))
        languages = get_languages(filter_lang)

        # actions
        if args.consistency:
            return consistency(filter_year, filter_day, filter_lang)

        # set the exclude list
        args.exclude = args.exclude or []
        if args.no_slow:
            args.exclude.extend(
                # " -x 2016:5"
                # " -x 2016:11"
                " -x 2016:14"
                # " -x 2016:23"
                " -x 2018:21"
                " -x 2018:23"  # z3 solution is really slow...
                # " -x 2020:15"
                # " -x 2021:18"
                # " -x 2022:15"
                # " -x 2023:5 -x 2023:10 -x 2023:23"
                .split()
            )

        # build the solutions if needed
        if not args.no_build:
            build_all(filter_year, filter_lang, languages)
            print_log(end=f"{CR}{CLEAR_EOL}")

        # load inputs and answers
        inputs, answers = load_data(filter_year, args.filter_user, args.exclude, args.verified)

        for script in Path(Env.AOC_TARGET_DIR).glob("resolve_*.sh"):
            script.unlink()

        # build the list of solutions to run
        tasks = list(get_task_list(filter_year, filter_day, inputs, args.alt))

        if len(tasks) == 0:
            exit()

        # here we go!

        if args.shuffle:
            random.shuffle(tasks)

        prev_shown_year = 0
        shown_in_year = 0

        for year, day, mday in tasks:
            if not args.shuffle and args.quiet == 0 and prev_shown_year != year and shown_in_year > 0:
                if prev_shown_year != 0:
                    line = (
                        "=========================="  # prefix
                        " ============================"  # language, status
                        " =================================================="  # answers
                        " =================================="  # input path
                    )
                    print(line[: terminal_columns - 1])
                    shown_in_year = 0

            run_task(year, day, mday, inputs, answers, languages, problems, args, terminal_columns, stats_elapsed)

            shown_in_year += 1
            prev_shown_year = year

        if args.prune:
            get_cache()["db"].commit()

    except KeyboardInterrupt:
        pass

    finally:
        if args.quiet < 2 and stats_elapsed:
            languages = sorted(set(map(itemgetter(3), stats_elapsed.keys())))

            nb_puzzles = len(set((y, d) for y, d, _, _ in stats_elapsed.keys()))
            nb_solutions = 0

            # compute elapsed time by language
            total_time = 0
            lines = []
            for lang in languages:
                t = list(t for (_, _, _, i), (t, _) in stats_elapsed.items() if lang == i)
                n = len(t)
                t = sum(t)

                average = t / n
                if average <= 0.002:
                    average = f"{average:10.6f} s"
                else:
                    average = f"{average:7.3f} s   "

                stats = (
                    f"{YELLOW}{lang:<10}{RESET}"
                    f" : {GREEN}{t:9.3f}s{RESET} for {WHITE}{n:3}{RESET} puzzle{'s' if n > 1 else ' '},"
                    f" average: {GREEN}{average}{RESET}"
                )
                ver = get_language_version(lang)
                if ver:
                    stats += f" ∞ {ITALIC}{ver}{RESET}"

                lines.append((t, stats))
                total_time += t
                nb_solutions += n

            print(f"{CR}{CLEAR_EOL}")
            print(f"{BOLD}ELAPSED TIME:{RESET}")
            print("\n".join(map(itemgetter(1), sorted(lines, key=itemgetter(1)))))
            print(
                "total     "
                f" : {GREEN}{total_time:9.3f}s{RESET}"
                f" for {WHITE}{nb_puzzles:3}{RESET} puzzle{'s' if nb_puzzles > 1 else ' '}"
                f" and {WHITE}{nb_solutions:5}{RESET} solution{'s' if nb_solutions > 1 else ''}"
            )

            overall_total_time = sum(t * ns for t, ns in stats_elapsed.values())
            overall_nb_solutions = sum(ns for _, ns in stats_elapsed.values())
            if overall_nb_solutions != nb_solutions:
                inputs_per_puzzle = round(overall_nb_solutions / nb_solutions, 1)
                inputs_per_puzzle = f" with {inputs_per_puzzle} inputs/puzzle"
            else:
                inputs_per_puzzle = ""
            print(
                "overall   "
                f" : {GREEN}{overall_total_time:9.3f}s{RESET}"
                f" for {WHITE}{nb_puzzles:3}{RESET} puzzle{'s' if nb_puzzles > 1 else ' '}"
                f" and {WHITE}{overall_nb_solutions:5}{RESET} solution{'s' if overall_nb_solutions > 1 else ''}"
                f"{inputs_per_puzzle}"
            )

            # compute languages comparison
            if args.comparison:
                lines = []
                puzzles = set(map(itemgetter(0, 1), stats_elapsed.keys()))
                for lang1, lang2 in itertools.combinations(languages, 2):
                    n, t1, t2 = 0, 0, 0
                    for y, d in puzzles:
                        t = dict(
                            (lang, t) for (yy, dd, dd_str, lang), (t, _) in stats_elapsed.items() if (yy, dd) == (y, d)
                        )
                        if lang1 in t and lang2 in t:
                            n += 1
                            t1 += t[lang1]
                            t2 += t[lang2]
                    if n > 0:
                        if t2 < t1:
                            t1, t2 = t2, t1
                            lang1, lang2 = lang2, lang1
                        faster = t2 / t1
                        lines.append(
                            (
                                t1 / n,
                                t2 / n,
                                f"{YELLOW}{lang1:<7}{RESET}"
                                f" vs. {YELLOW}{lang2:<7}{RESET}:"
                                f" {GREEN}{t1 / n:7.3f}s{RESET} vs. {GREEN}{t2 / n:7.3f}s{RESET}"
                                f" (x {faster:5.1f} faster) on {WHITE}{n:3}{RESET} puzzle{'s' if n > 1 else ''}",
                            )
                        )
                print()
                print(f"{BOLD}LANGUAGES COMPARISON:{RESET}")
                print("\n".join(map(itemgetter(2), sorted(lines))))

            elif len(languages) > 1:
                print("Use option -C/--comparison to display duration comparison.")

        if args.quiet < 2 and problems:
            print()
            print("LIST OF PROBLEMS:")
            for problem in problems:
                print(problem)

        if args.quiet < 2:
            for i, f in enumerate(Path(Env.AOC_TARGET_DIR).glob("resolve_*.sh")):
                if i == 0:
                    print("\nFix errors then run:")
                print(f"  {RED}{f}{RESET}")


if __name__ == "__main__":
    try:
        main()
    except KeyboardInterrupt:
        sys.exit(1)
