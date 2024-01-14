#!/usr/bin/env python3

import argparse
import json
import os
import subprocess
import time
import typing as t
from collections import defaultdict
from copy import deepcopy
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


def run(key: str, prog: Path, file: Path, solution: t.List):
    if not prog.is_file():
        return

    prog = prog.absolute()
    if e := check_cache(key, prog):
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
            subprocess.check_call(["cargo", "build", "--manifest-path", m, "--release", "--quiet"])

        for day in range(1, 26):
            src = year / f"day{day}" / f"day{day}.c"
            make(year, src, f"day{day}_c", "cc -std=c11")

            src = year / f"day{day}" / f"day{day}.cpp"
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


def run_day(year: int, day: int, inputs: t.Dict, sols: t.Dict, problems: t.Set, filter_lang):
    for crc, file in inputs[year, day].items():
        input_name = file.parent.parent.name.removeprefix("tmp-")[:16]
        prefix = f"[{year}-{day:02d}] {input_name:<16}"

        if day % 2 == 1:
            prefix = f"{BLUE}{prefix}{RESET}"
        else:
            prefix = f"{CYAN}{prefix}{RESET}"

        results = set()

        for lang, pattern in LANGUAGES.items():
            if filter_lang and lang.lower() != filter_lang.lower():
                continue

            prog = Path(pattern.format(year=year, day=day))
            key = ":".join(map(str, (year, day, crc, prog, lang.lower())))

            e = run(key, prog, file, sols[year, day].get(crc))

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
                f" {GRAY}{'cache' if e['cache'] else '':<5}"
                f" {status_color}{str(e['answers']):<40}{RESET}"
                f"{info}"
            )
            print(line)

            if e["status"] == "fail":
                problems.append(line)

            if not e["cache"] and e["elapsed"] / 1e9 > 5:
                save_cache()

            results.add(" ".join(e["answers"]))

        if len(results) > 1:
            line = f"{prefix} {RED}{BLINK}MISMATCH BETWEEN SOLUTIONS{RESET}"
            print(line)
            problems.append(line)


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("-l", "--language", type=str, metavar="LANG", help="filter by language")
    parser.add_argument("n", type=int, nargs="*", help="filter by year or year/day")

    args = parser.parse_args()
    if len(args.n) > 2:
        parser.error("too many args")

    filter_year = 0 if len(args.n) == 0 else args.n[0]
    filter_day = 0 if len(args.n) <= 1 else args.n[1]
    if args.language == "cpp":
        args.language = "c++"

    try:
        os.chdir(Path(__file__).parent.parent)

        build_all()
        inputs, sols = load_data()

        problems = []

        for year in range(2015, 2024):
            if filter_year != 0 and year != filter_year:
                continue

            for day in range(1, 26):
                if filter_day != 0 and day != filter_day:
                    continue

                run_day(year, day, inputs, sols, problems, args.language)
                save_cache()

            if filter_year == 0:
                print(
                    "=========================="  # prefix
                    " ==============================="  # language, status
                    " ========================================"  # answers
                    " =================================="  # input path
                )

    except KeyboardInterrupt:
        pass

    finally:
        if problems:
            print()
            print("LIST OF PROBLEMS:")
            for problem in problems:
                print(problem)


if __name__ == "__main__":
    main()
