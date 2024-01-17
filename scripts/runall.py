#!/usr/bin/env python3

import argparse
import itertools
import json
import os
import subprocess
import time
import typing as t
from collections import defaultdict
from copy import deepcopy
from operator import itemgetter
from pathlib import Path
from zlib import crc32

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

LANGUAGES = {
    "Python": "{year}/day{day}/day{day}.py",
    "Rust": "{year}/target/release/day{day}",
    "C": "{year}/build/day{day}_c",
    "C++": "{year}/build/day{day}_cpp",
}


def get_cache():
    cache = globals().get("_cache")
    if cache is None:
        cache_file = Path(__file__).parent.parent / "data" / "cache.json"
        if cache_file.is_file():
            cache = json.loads(cache_file.read_bytes())
        else:
            cache = {}
        globals()["_cache"] = cache
        cache["modified"] = False
    return cache


def save_cache():
    cache = get_cache()
    if cache["modified"]:
        cache.pop("modified")
        cache_file = Path(__file__).parent.parent / "data" / "cache.json"
        cache_file.write_text(json.dumps(cache, indent=2, ensure_ascii=True))
        cache["modified"] = False
        print(f"{FEINT}{ITALIC}cache commited{RESET}")


def check_cache(key, timestamp: Path):
    cache = get_cache()
    key = str(key)
    e = cache.get(key, None)
    if e:
        timestamp = timestamp.stat().st_mtime_ns
        if e["timestamp"] == timestamp:
            return e


def update_cache(key, timestamp: Path, elapsed, status, answers):
    cache = get_cache()
    key = str(key)
    e = cache.get(key, {})
    e["timestamp"] = timestamp.stat().st_mtime_ns
    e["elapsed"] = elapsed
    e["status"] = status
    e["answers"] = answers
    cache[key] = e
    cache["modified"] = True
    return e


def run(key: str, prog: Path, file: Path, solution: t.List, refresh: bool):
    if not prog.is_file():
        return

    prog = prog.absolute()

    if refresh:
        e = None
    else:
        e = check_cache(key, prog)

    if e:
        in_cache = True
    else:
        in_cache = False

        print(f"{FEINT}{ITALIC}missing cache for {key}{RESET}", end="\r")

        start = time.time_ns()
        out = subprocess.run([prog, file.absolute()], cwd=prog.parent, stdout=subprocess.PIPE)
        elapsed = time.time_ns() - start
        answers = out.stdout.decode().strip()

        status = "unknown"
        if solution:
            solution = solution.read_text()
            if answers == solution.strip():
                status = "ok"
            else:
                status = "error"
        else:
            if answers == "":
                status = "fail"
            else:
                status = "unknown"

        e = update_cache(key, prog, elapsed, status, answers.split("\n"))

    e = deepcopy(e)
    e["cache"] = in_cache

    return e


def make(year: Path, source: Path, dest: Path, cmd: str):
    if not source.is_file():
        return

    build = year / "build"
    build.mkdir(parents=True, exist_ok=True)

    output = build / dest

    if output.is_file() and output.stat().st_mtime_ns >= source.stat().st_mtime_ns:
        return

    cmdline = f"{cmd} -o {output} -Wall -Wextra -O3 -DSTANDALONE -I{source.parent} {source}"
    print(cmdline)
    subprocess.check_call(cmdline, shell=True)


def build_all():
    for year in range(2015, 2024):
        year = Path(str(year))
        if not year.is_dir():
            continue
        m = year / "Cargo.toml"
        if year.is_dir() and m.is_file():
            print(f"{FEINT}{ITALIC}cargo build {m}{RESET}", end="\033[0K\r")
            subprocess.check_call(["cargo", "build", "--manifest-path", m, "--release", "--quiet"])

        for day in range(1, 26):
            src = year / f"day{day}" / f"day{day}.c"
            print(f"{FEINT}{ITALIC}compile {src}{RESET}", end="\033[0K\r")
            make(year, src, f"day{day}_c", "cc -std=c11")

            src = year / f"day{day}" / f"day{day}.cpp"
            print(f"{FEINT}{ITALIC}compile {src}{RESET}", end="\033[0K\r")
            make(year, src, f"day{day}_cpp", "c++ -std=c++17")


def load_data():
    inputs = defaultdict(dict)
    solutions = defaultdict(dict)

    for f in Path("data").rglob("*.in"):
        assert len(f.parts) == 4

        year = int(f.parent.name)
        day = int(f.stem)

        e = check_cache(f, f)
        if e:
            crc = e["status"]
        else:
            crc = hex(crc32(f.read_bytes().strip()) & 0xFFFFFFFF)
            update_cache(f, f, 0, crc, 0)

        if crc not in inputs[year, day]:
            inputs[year, day][crc] = f

        s = f.with_suffix(".ok")
        if s.is_file():
            solutions[year, day][crc] = s

    save_cache()
    return inputs, solutions


def run_day(year: int, day: int, mday: str, inputs: t.Dict, sols: t.Dict, problems: t.Set, filter_lang, refresh):
    elapsed = defaultdict(list)

    first = True

    day_suffix = mday.removeprefix(str(day))
    name_max_len = 16 - len(day_suffix)

    for crc, file in inputs[year, day].items():
        input_name = file.parent.parent.name.removeprefix("tmp-")[:16]
        prefix = f"[{year}-{day:02d}{day_suffix}] {input_name[:name_max_len]:<{name_max_len}}"

        if day % 2 == 1:
            prefix = f"{BLUE}{prefix}{RESET}"
        else:
            prefix = f"{CYAN}{prefix}{RESET}"

        results = set()

        for lang, pattern in LANGUAGES.items():
            if filter_lang and lang.lower() != filter_lang.lower():
                continue

            prog = Path(pattern.format(year=year, day=mday))
            key = ":".join(map(str, (year, day, crc, prog, lang.lower())))

            if lang.lower() == "rust" and first and prog.is_file():
                # under macOS, the first launch of a program is slower
                first = False
                subprocess.call([prog, "--help"], stdout=subprocess.DEVNULL)

            e = run(key, prog, file, sols[year, day].get(crc), refresh)

            if not e:
                continue

            if e["status"] in ["unknown", "fail"]:
                info = f" {file}"
            else:
                info = ""

            status_color = {"fail": MAGENTA, "unknown": GRAY, "error": RED, "ok": GREEN}[e["status"]]

            line = (
                f"{prefix}"
                f" {YELLOW}{lang:<7}{RESET}:"
                f" {status_color}{e['status']:7}{RESET}"
                f" {WHITE}{e['elapsed']/1e9:7.3f}s"
                f" {GRAY}{'☽' if e['cache'] else ' '}"
                f" {status_color}{str(e['answers']):<40}{RESET}"
                f"{info}"
            )
            print(line)

            if e["status"] == "fail":
                problems.append(line)

            if not e["cache"] and e["elapsed"] / 1e9 > 5:
                save_cache()

            results.add(" ".join(e["answers"]))

            elapsed[lang].append(e["elapsed"] / 1e9)

        if len(results) > 1:
            line = f"{prefix} {RED}{BLINK}MISMATCH BETWEEN SOLUTIONS{RESET}"
            print(line)
            problems.append(line)

    nb_samples = set(len(t) for _, t in elapsed.items())
    assert len(nb_samples) == 1

    return dict((lang, sum(t) / len(t)) for lang, t in elapsed.items()), nb_samples.pop()


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("-l", "--language", type=str, metavar="LANG", help="filter by language")
    parser.add_argument("-r", "--refresh", action="store_true", help="relaunch solutions")
    parser.add_argument("n", type=int, nargs="*", help="filter by year or year/day")

    args = parser.parse_args()

    filter_year = 0 if len(args.n) == 0 else args.n.pop(0)
    filter_day = set(args.n)
    if args.language == "cpp":
        args.language = "c++"

    try:
        os.chdir(Path(__file__).parent.parent)

        problems = []
        stats_elapsed = dict()

        build_all()
        inputs, sols = load_data()

        for year in range(2015, 2024):
            if filter_year != 0 and year != filter_year:
                continue

            for day in range(1, 26):
                if filter_day and day not in filter_day:
                    continue

                for mday in list(Path(f"{year}").glob(f"day{day}")) + list(Path(f"{year}").glob(f"day{day}_*")):
                    mday = mday.name.removeprefix("day")

                    elapsed, nb_samples = run_day(year, day, mday, inputs, sols, problems, args.language, args.refresh)
                    save_cache()

                    if elapsed:
                        print(
                            "--> ",
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

    except Exception as e:
        print(f"{RED}ERROR {e}{RESET}")

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
                    f" : {GREEN}{t:7.3f}s{RESET} for {n:3} puzzles,"
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
                        f" (x {faster:4.1f} faster) on {n:3} puzzle{'s' if n>1 else ''}"
                    )

        if problems:
            print()
            print("LIST OF PROBLEMS:")
            for problem in problems:
                print(problem)


if __name__ == "__main__":
    main()
