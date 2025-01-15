#!/usr/bin/env python3

import argparse
import hashlib
import itertools
import logging
import os
import re
import shutil
import sqlite3
import subprocess
import sys
import time
import typing as t
from collections import defaultdict
from datetime import datetime
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
FEINT = "\033[2m"
ITALIC = "\033[3m"
BLINK = "\033[6m"
CLEAR_EOL = "\033[0K"
CR = "\r"
TRANSIENT = f"{CLEAR_EOL}{CR}"

TERMINAL_COLS = 96


LANGUAGES = {
    "Python": "{year}/day{day}/day{day}.py",
    "Rust": "{year}/target/release/day{day}",
    "C": "{year}/build/day{day}_c",
    "C++": "{year}/build/day{day}_cpp",
    "Lua": "{year}/day{day}/day{day}.lua",
    "JavaScript": "{year}/day{day}/day{day}.js",
    "Ruby": "{year}/day{day}/day{day}.rb",
    "Perl": "{year}/day{day}/day{day}.pl",
    "Bash": "{year}/day{day}/day{day}.sh",
    "Java": "{year}/build/day{day}.class",
    "Go": "{year}/build/day{day}_go",
    "C#": "{year}/build/day{day}_cs.exe",
    "Swift": "{year}/build/day{day}_swift",
}

INTERPRETERS = {
    "Python": {
        "Python": (".venv/python/bin/python3", "/venv/python/bin/python3", "~/.venv/bin/python3"),
        "PyPy": ".venv/pypy3.10/bin/python3",
        "Py3.10": ".venv/py3.10/bin/python3",
        "Py3.11": ".venv/py3.11/bin/python3",
        "Py3.12": ".venv/py3.12/bin/python3",
        "Py3.13": ".venv/py3.13/bin/python3",
    },
    "JavaScript": {
        "NodeJS": "node",
        "BunJS": "bun",
    },
}


def get_cache(cache_file: Path = None):
    """Retrieve the cache instance from memory or load it from disk."""
    cache = globals().get("_cache")
    if cache is None:
        cache_file = cache_file or Path(__file__).parent.parent / "data" / "cache.db"

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

        logging.debug(f"using cache file {cache_file}")

    return cache


def save_cache():
    pass
    # cache = get_cache()
    # if cache["modified"]:
    #     cache["db"].commit()
    #     cache["modified"] = False
    #     print(f"{FEINT}{ITALIC}cache commited{RESET}")


def shorten_key(key: str) -> str:
    key = re.sub(r":([a-f\d]{9}[a-f\d]+):", lambda m: f":{m[1][:8]}…:", key)
    return key


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

            print(f"{FEINT}{ITALIC}entry {shorten_key(key)} is out of date{RESET}", end=TRANSIENT)

    else:
        print(f"{FEINT}{ITALIC}missing cache for {shorten_key(key)}{RESET}", end=TRANSIENT)


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
    expected: t.List,
    nb_expected: int,
    warmup: bool,
) -> t.Dict[str, t.Any]:
    """
    TODO
    """

    if prog.suffix == ".class":
        cmd = ["java", "-cp", prog.parent, prog.stem]
    elif prog.suffix == ".exe":
        cmd = ["mono", prog.absolute().as_posix()]
    else:
        cmd = [prog.absolute().as_posix()]

    # add the interpreter
    if interpreter:
        cmd.insert(0, interpreter)

    if warmup and lang == "Rust":
        # under macOS, the first launch of a Rust program is slower (why ???)
        subprocess.call(cmd + ["--help"], stdout=subprocess.DEVNULL)

    cmd.append(file.absolute().as_posix())

    start = time.time_ns()
    try:
        env = os.environ
        env["NO_COLOR"] = "1"
        out = subprocess.run(cmd, stdout=subprocess.PIPE, env=env)
        elapsed = time.time_ns() - start

        if out.returncode != 0:
            status = "error"
            answers = ""
        else:
            answers = out.stdout.decode().strip().split("\n")
            nb_answers = len(answers)
            answers = " ".join(answers)

            status = "unknown"
            if expected:
                expected = " ".join(expected.read_text().strip().split("\n"))
                status = "ok" if answers == expected else "failed"
            else:
                status = "unknown" if nb_answers == nb_expected else "failed"

    except OSError:
        status = "error"
        answers = ""
        elapsed = 0

    result = {"elapsed": elapsed, "status": status, "answers": answers}

    with Path("run.log").open("at") as f:
        line = f"{datetime.now()} {lang} {cmd} {elapsed/1e9} {status} '{answers}'"
        line = line.replace(Path(__file__).parent.parent.as_posix() + "/", "")
        print(line, file=f)

    return result


def make(year: int, source: Path, dest: Path, language: str):
    if not source.is_file():
        return

    build = Path(str(year)) / "build"
    build.mkdir(parents=True, exist_ok=True)

    output = build / dest

    if output.is_file() and output.stat().st_mtime_ns >= source.stat().st_mtime_ns:
        return

    if language == "C":
        cmd = "cc -std=c11"
        cmdline = f"{cmd} -o {output} -Wall -Wextra -Werror -O3 -DSTANDALONE -I{source.parent} {source}"
    elif language == "C++":
        cmd = "c++ -std=c++23"
        cmdline = f"{cmd} -o {output} -Wall -Wextra -Werror -O3 -DSTANDALONE -I{source.parent} {source}"
    elif language == "Java":
        cmdline = f"javac -d {build} {source}"
    elif language == "Go":
        cmdline = f"go build -o {output} {source}"
    elif language == "C#":
        cmdline = f"mcs -out:{output} {source}"
    elif language == "Swift":
        cmdline = f"swiftc -o {output} {source}"
    else:
        raise ValueError(language)

    print(f"{CR}{CLEAR_EOL}{cmdline}", end="")
    subprocess.check_call(cmdline, shell=True)


def build_all(filter_year: int, filter_lang: t.Iterable[str]):
    for year in range(2015, 2025):
        if filter_year != 0 and year != filter_year:
            continue

        # year = Path(str(year))
        # if not year.is_dir():
        #     continue

        if not filter_lang or "rust" in filter_lang:
            m = Path(str(year)) / "Cargo.toml"
            if m.is_file():
                env_copy = os.environ.copy()
                # env_copy["RUSTFLAGS"] = "-C target-cpu=native"
                print(f"{FEINT}{ITALIC}cargo build {m}{RESET}", end=TRANSIENT)
                subprocess.check_call(["cargo", "build", "--manifest-path", m, "--release", "--quiet"], env=env_copy)

        for day in range(1, 26):
            if not filter_lang or "c" in filter_lang:
                src = Path(str(year)) / f"day{day}" / f"day{day}.c"
                if src.is_file():
                    print(f"{FEINT}{ITALIC}compile {src}{RESET}", end=TRANSIENT)
                    make(year, src, f"day{day}_c", "C")

            if not filter_lang or "c++" in filter_lang:
                src = Path(str(year)) / f"day{day}" / f"day{day}.cpp"
                if src.is_file():
                    print(f"{FEINT}{ITALIC}compile {src}{RESET}", end=TRANSIENT)
                    make(year, src, f"day{day}_cpp", "C++")

            if not filter_lang or "java" in filter_lang:
                src = Path(str(year)) / f"day{day}" / f"day{day}.java"
                if src.is_file():
                    print(f"{FEINT}{ITALIC}compile {src}{RESET}", end=TRANSIENT)
                    make(year, src, f"day{day}.class", "Java")

            if not filter_lang or "go" in filter_lang:
                src = Path(str(year)) / f"day{day}" / f"day{day}.go"
                if src.is_file():
                    print(f"{FEINT}{ITALIC}compile {src}{RESET}", end=TRANSIENT)
                    make(year, src, f"day{day}_go", "Go")

            if not filter_lang or "c#" in filter_lang:
                src = Path(str(year)) / f"day{day}" / f"day{day}.cs"
                if src.is_file():
                    print(f"{FEINT}{ITALIC}compile {src}{RESET}", end=TRANSIENT)
                    make(year, src, f"day{day}_cs.exe", "C#")

            if not filter_lang or "swift" in filter_lang:
                src = Path(str(year)) / f"day{day}" / f"day{day}.swift"
                if src.is_file():
                    print(f"{FEINT}{ITALIC}compile {src}{RESET}", end=TRANSIENT)
                    make(year, src, f"day{day}_swift", "Swift")


def load_data(filter_year, filter_user, filter_yearday, with_answers):
    inputs = defaultdict(dict)
    answers = defaultdict(dict)

    all_inputs = list(sorted(Path("data").rglob("*.in")))

    me_user = all_inputs[0].parent.parent.name

    for input in all_inputs:
        if input.name.startswith("._"):
            continue

        assert len(input.parts) == 4

        user = input.parent.parent.name

        if filter_user == "me":
            if user != me_user:
                continue
        elif filter_user == "mine":
            if not user.isdigit():
                continue
        elif filter_user and user != filter_user and user[: 4 + len(filter_user)] != f"tmp-{filter_user}":
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

        key = f"{year}:{day}:{user}"

        e = check_cache(key, input, "inputs", ("crc32",))
        if e:
            crc = e["crc32"]
        else:
            # crc = hex(crc32(f.read_bytes().strip()) & 0xFFFFFFFF)
            crc = hashlib.sha256(input.read_bytes().strip()).hexdigest()

            update_cache(key, input, "inputs", {"crc32": crc})

        if crc not in inputs[year, day]:
            inputs[year, day][crc] = input
            answers[year, day][crc] = answer

    save_cache()
    return inputs, answers


def run_day(
    year: int,
    day: int,
    mday: str,
    day_inputs: t.Dict,
    day_answers: t.Dict,
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

            if not prog.is_file():
                # special case for day13_alt/day13.py
                if "_" in prog.stem and prog.stem == prog.parent.name:
                    prog = prog.with_stem(prog.stem[: prog.stem.find("_")])
            if not prog.is_file():
                continue

            if refresh:
                e = None
                in_cache = False
            else:
                e = check_cache(key, prog, "solutions", ("elapsed", "status", "answers"), dry_run)
                in_cache = e is not None

            if not in_cache and not dry_run:
                nb_expected = 2 if day <= 24 else 1
                e = run(prog, lang, interpreter, file, day_answers.get(crc), nb_expected, warmup[lang])

                if e:
                    warmup[lang] = False
                    update_cache(key, prog, "solutions", e)

            if not e:
                continue

            if (e["status"] == "unknown" and day_answers.get(crc)) or e["status"] in ("error", "failed"):
                script = Path(f"resolve_{e['status']}.sh")
                if not globals().get(script):
                    with script.open("wt") as f:
                        print("#!/bin/sh", file=f)
                    script.chmod(0o755)
                    globals()[script] = True

                with script.open("at") as f:
                    print(f"./scripts/runall.py --no-build -u {input_name} -l {lang} -r {year} {day}", file=f)

            if e["status"] != "ok":
                info = f" {file}"
            else:
                info = ""

            status_color = {
                "ok": GREEN,
                "unknown": GRAY,
                "failed": MAGENTA,
                "error": RED,
            }[e["status"]]

            answers = e["answers"]

            line = (
                f"{CR}{RESET}{CLEAR_EOL}"
                f"{prefix}"
                f" {YELLOW}{lang:<7}{RESET}:"
                f" {status_color}{e['status']:7}{RESET}"
                f" {WHITE}{e['elapsed']/1e9:7.3f}s"
                f" {GRAY}{'☽' if in_cache else ' '}"
                f" {status_color}{str(answers):<40}{RESET}"
            )
            if TERMINAL_COLS >= 130:
                print(line, info)
            else:
                print(line)

            if e["status"] in ("error", "failed"):
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
    languages = {}
    for lang, v in LANGUAGES.items():

        if lang in INTERPRETERS:

            for lang2, interpreter in INTERPRETERS[lang].items():

                if filter_lang and lang2.casefold() not in filter_lang:
                    continue

                def lookup(interpreter: str) -> str:
                    """Resolve an interpreter."""
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
                    interpreter = next(filter(None, map(lookup, interpreter)))

                if interpreter:
                    languages[lang2] = (v, interpreter)
        else:
            if filter_lang and lang.casefold() not in filter_lang:
                continue
            languages[lang] = (v, None)

    return languages


def consistency(filter_year, filter_day):
    import numpy as np
    import tabulate

    db = get_cache()["db"]  # sqlite3.connect("data/cache.db")

    elapsed_times = defaultdict(list)
    cursor = db.execute("select key,elapsed from solutions where status!='error'")
    for key, elapsed in cursor:
        year, day, hash, prog, lang = key.split(":")
        if filter_year:
            if filter_year != int(year):
                continue
            if filter_day and int(day) not in filter_day:
                continue
        elapsed_times[int(year), int(day), lang].append((round(elapsed / 1e9, 3), hash))
    cursor.close()

    inputs = dict()
    for hash, key in db.execute("select crc32,key from inputs"):
        _, _, user = key.split(":")
        if hash not in inputs or user.isdigit():
            inputs[hash] = user

    rows = list()
    cmds = list()

    for k, v in sorted(elapsed_times.items()):
        a = np.array(list(map(itemgetter(0), v)))

        # coefficient of variation
        µ, σ = a.mean(), a.std()
        cv = σ / µ
        cv = round(cv * 100, 1)

        # quartile coefficient of dispersion
        q1 = np.percentile(a, 25)
        q3 = np.percentile(a, 75)
        qcd = (q3 - q1) / (q3 + q1)
        qcd = round(qcd * 100, 1)

        if µ > 3 and (cv > 15 or qcd > 10):
            rows.append((" ".join(map(str, k)), µ, σ, cv, qcd, a))

            for elapsed, hash in v:
                deviation = (elapsed - µ) / σ
                if deviation > 1.5:
                    year, day, lang = k
                    user = inputs[hash]
                    cmd = f"./scripts/runall.py -u {user:<22} -l {lang:<8} {year} {day:<2} -r"
                    # comment = f"{RESET}  # t={YELLOW}{elapsed:7.3f}{RESET} µ={GREEN}{µ:7.3f}{RESET} d={YELLOW}{deviation:4.1f}{RESET} σ"
                    comment = f"  # t={elapsed:7.3f} µ={µ:7.3f} d={deviation:4.1f} σ"
                    cmds.append(((year, day, user), cmd + comment))

    print(tabulate.tabulate(rows, headers=("solution", "µ", "σ", "CV", "qcd", "values"), tablefmt="fancy_grid"))

    cmds = sorted(cmds)

    previous_key = None
    flip_color = 0
    for key, cmd in cmds:
        if key != previous_key:
            flip_color = 1 - flip_color
            previous_key = key
        color = [BLUE, CYAN][flip_color]
        print(f"{color}{cmd}{RESET}")


def main():
    parser = argparse.ArgumentParser(formatter_class=lambda prog: argparse.HelpFormatter(prog, max_help_position=28))

    parser.add_argument("-v", "--verbose", action="store_true", help="verbose")
    parser.add_argument("--cache", type=Path, help="cache database")
    parser.add_argument("--working-dir", type=Path, help=argparse.SUPPRESS)

    parser.add_argument("--venv", type=Path, help="create and install virtual environment")
    parser.add_argument("--reqs", action="store_true", help="install requirements into virtual environments")
    parser.add_argument("-c", "--consistency", action="store_true", help="verify duration consistency")

    parser.add_argument("-u", "--user", dest="filter_user", metavar="USER", type=str, help="filter by user id")
    parser.add_argument("--me", action="store_true", help="only first user id")
    parser.add_argument("-l", "--language", type=str, action="append", metavar="LANG", help="filter by language")
    parser.add_argument("-x", "--exclude", type=str, action="append", metavar="Y:D", help="exclude day")
    parser.add_argument("--verified", action="store_true", help="only inputs with solution")
    parser.add_argument("--no-slow", action="store_true", help="exclude slow solutions")

    parser.add_argument("-r", "--refresh", action="store_true", help="relaunch solutions")
    parser.add_argument("-n", "--dry-run", action="store_true", help="do not run")
    parser.add_argument("--no-build", action="store_true", help="do not build")

    parser.add_argument("-C", "--comparison", action="store_true", help="Show languages commparison")

    parser.add_argument("n", type=int, nargs="*", help="filter by year or year/day")

    args = parser.parse_args()

    if args.verbose:
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
    cols = subprocess.getoutput("tput cols")
    if cols.isdigit():
        global TERMINAL_COLS
        TERMINAL_COLS = int(cols)

    if args.working_dir and args.working_dir.is_dir():
        logging.debug(f"set working directory to: {args.working_dir}")
        os.chdir(args.working_dir)

    try:
        if args.me:
            args.filter_user = "me"

        problems = []
        stats_elapsed = dict()

        # if in subdirectoy "<year>/day<day>" set the filter
        if len(args.n) == 0:
            cwd = Path.cwd()
            if cwd.name.isdigit():
                args.n.append(int(cwd.name))
            elif cwd.name.startswith("day") and cwd.parent.name.isdigit():
                day = cwd.name.removeprefix("day")
                if "_" in day:
                    day = day[: day.find("_")]
                args.n.extend((int(cwd.parent.name), int(day)))

        os.chdir(Path(__file__).parent.parent)

        # prepare the filtering by date
        filter_year = 0 if len(args.n) == 0 else int(args.n.pop(0))
        filter_day = set(args.n)

        # actions
        if args.consistency:
            return consistency(filter_year, filter_day)

        # WTF ? why I have kept this ? get_languages() seems to do the job
        # # resolve interpreters
        # for lang, variants in INTERPRETERS.items():
        #     for variant in list(variants.keys()):
        #         interpreters = variants[variant]
        #         if isinstance(interpreters, tuple) or isinstance(interpreters, list):
        #             for prog in interpreters:
        #                 prog = shutil.which(prog)
        #                 if prog:
        #                     variants[variant] = prog
        #                     break
        #             else:
        #                 variants.pop(variant)
        #         else:
        #             if not shutil.which(interpreters):
        #                 variants.pop(variant)

        # prepare the language filtering
        filter_lang = set(map(str.casefold, args.language or ()))
        languages = get_languages(filter_lang)

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
            build_all(filter_year, filter_lang)
            print(end=f"{CR}{CLEAR_EOL}")

        # load inputs and answers
        inputs, answers = load_data(filter_year, args.filter_user, args.exclude, args.verified)

        for script in Path(".").glob("resolve_*.sh"):
            script.unlink()

        # here we go!
        for year in range(2015, 2025):
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
                        answers[year, day],
                        languages,
                        problems,
                        args.refresh,
                        args.dry_run,
                    )
                    save_cache()

                    if elapsed:
                        if nb_samples > 1:
                            print(
                                f"{CR}{CLEAR_EOL}--> ",
                                " | ".join((f"{lang} : {t:.3f}s" for lang, t in elapsed.items())),
                                f"{FEINT}({nb_samples} input{'s' if nb_samples>1 else ''}){RESET}",
                            )
                        else:
                            print(end=f"{CR}{CLEAR_EOL}")

                        for lang, e in elapsed.items():
                            stats_elapsed[year, day, lang] = (e, nb_samples)

            if filter_year == 0:
                print(
                    "=========================="  # prefix
                    " ==========================="  # language, status
                    " ========================================"  # answers
                    " =================================="  # input path
                )

    except KeyboardInterrupt:
        pass

    # except Exception as e:
    #     print(f"{RED}ERROR {e}{RESET}")

    finally:
        if stats_elapsed:
            languages = sorted(set(map(itemgetter(2), stats_elapsed.keys())))

            nb_puzzles = len(set((y, d) for y, d, _ in stats_elapsed.keys()))
            nb_solutions = 0

            # compute elapsed time by language
            total_time = 0
            lines = []
            for lang in languages:
                t = list(t for (_, _, i), (t, _) in stats_elapsed.items() if lang == i)
                n = len(t)
                t = sum(t)
                lines.append(
                    (
                        t,
                        f"{YELLOW}{lang:<10}{RESET}"
                        f" : {GREEN}{t:9.3f}s{RESET} for {WHITE}{n:3}{RESET} puzzle{'s' if n>1 else ' '},"
                        f" average: {GREEN}{t/n:7.3f}s{RESET}",
                    )
                )
                total_time += t
                nb_solutions += n

            print()
            print("ELAPSED TIME:")
            print("\n".join(map(itemgetter(1), sorted(lines))))
            print(
                "total     "
                f" : {GREEN}{total_time:9.3f}s{RESET}"
                f" for {WHITE}{nb_puzzles:3}{RESET} puzzle{'s' if nb_puzzles>1 else ' '}"
                f" and {WHITE}{nb_solutions:5}{RESET} solution{'s' if nb_solutions>1 else ''}"
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
                f" for {WHITE}{nb_puzzles:3}{RESET} puzzle{'s' if nb_puzzles>1 else ' '}"
                f" and {WHITE}{overall_nb_solutions:5}{RESET} solution{'s' if overall_nb_solutions>1 else ''}"
                f"{inputs_per_puzzle}"
            )

            # compute languages comparison
            if args.comparison:
                lines = []
                puzzles = set(map(itemgetter(0, 1), stats_elapsed.keys()))
                for lang1, lang2 in itertools.combinations(languages, 2):
                    n, t1, t2 = 0, 0, 0
                    for y, d in puzzles:
                        t = dict((lang, t) for (yy, dd, lang), (t, _) in stats_elapsed.items() if (yy, dd) == (y, d))
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
                                f" {GREEN}{t1/n:7.3f}s{RESET} vs. {GREEN}{t2/n:7.3f}s{RESET}"
                                f" (x {faster:4.1f} faster) on {WHITE}{n:3}{RESET} puzzle{'s' if n>1 else ''}",
                            )
                        )
                print()
                print("LANGUAGES COMPARISON:")
                print("\n".join(map(itemgetter(2), sorted(lines))))

            elif len(languages) > 1:
                print("Use option -C/--comparison to display duration comparison.")

        if problems:
            print()
            print("LIST OF PROBLEMS:")
            for problem in problems:
                print(problem)


if __name__ == "__main__":
    main()
