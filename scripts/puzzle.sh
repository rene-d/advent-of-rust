#!/usr/bin/env bash

set -euo pipefail

rootdir=$(realpath $(dirname $0)/..)
declare -a features=
day=
year=
available=

usage()
{
    echo "Usage: $0 [day]"
    exit
}

parse_args()
{
    declare -a args=
    for i ; do
        case $i in
            -h|--help) usage ;;
            -r|--rs|--rust) features+=("rust") ;;
            -p|--py|--python) features+=("python") ;;
            -t|--test) features+=("test_samples") ;;
            *) args+=$i ;;
        esac
    done
    set -- "${args[@]}"

    if [[ $(basename $PWD) =~ day* ]]; then
        day=$(basename $PWD)
        day=${day##day}
        year=$(basename $(realpath $PWD/..))
        year=${year##year}
    else
        if [ $# -eq 0 ]; then
            usage
        fi

        year=$(basename $PWD)
        day=$1
        mkdir -p day$day
        cd day$day
    fi
}

has_feature()
{
    [[ -z "${features[@]}" ]] || [[ "${features[@]}" =~ "$1" ]]
}

fetch_input()
{
    local session
    local now
    local opening
    local waiting

    session=$(awk '/^[^#].*/{ if (! session) session=$1 } END{print session}' < $rootdir/.session)

    now=$(date -u +%Y%m%d%H%M%S)
    local ts=$(printf "%04d%02d%02d%02d%02d%02d" $year 12 $day 5 0 0)
    if [[ $now == $ts ]] || [[ $now > $ts ]] ; then
        curl "https://adventofcode.com/$year/day/$day/input" \
            -H "Cookie: session=$session" -o input.txt
        head input.txt
        wc -l input.txt
        available=1
    else
        opening=$(date -v${year}y -v12m -v${day}d -v5H -v0M -v0S -u +%s)
        now=$(date -u +%s)
        waiting=$(($opening-$now))
        printf "\033[5;93m"
        printf "Puzzle unavailable: please wait "
        if [[ $(($waiting / 3600)) != 0 ]]; then printf "$(($waiting / 3600)) hours, "; fi
        printf "$(( $(($waiting / 60)) % 60)) minutes and $(($waiting % 60)) seconds."
        printf "\033[0m\n"
        available=
    fi
}

fetch_samples()
{
    local session

    [[ $available ]] || return 0

    session=$(awk '/^[^#].*/{ if (! session) session=$1 } END{print session}' < $rootdir/.session)

    curl -s "https://adventofcode.com/$year/day/$day" \
        -H "Cookie: session=$session" | python3 -EB -c '
import re, sys, pathlib
for i, m in enumerate(re.finditer(r"<pre><code>(.*?)</code></pre>", sys.stdin.read(), re.DOTALL), 1):
    sample = m[1]
    sample = re.sub(r"<em>(.*?)</em>", r"\1", sample)
    sample = sample.replace("&gt;", ">")
    sample = sample.replace("&lt;", "<")
    print(f"\033[32mextracting sample {i} ({len(sample)} bytes)\033[0m")
    pathlib.Path(f"sample_{i}.txt").write_text(sample)
'
}

create_python()
{
    has_feature python || return 0

    if [ -f day$day.py ]; then
        printf "\033[31mPython script already exists.\033[0m\n"
        return
    fi

    local title=$($rootdir/scripts/answers.py --get-title --year $year --day $day)

    cat <<EOF >day$day.py
#!/usr/bin/env python3
# $title

from pathlib import Path
from argparse import ArgumentParser
from copy import deepcopy
from collections import defaultdict, deque, namedtuple, Counter
from functools import reduce
from operator import mul
import sys, re, math, itertools, time, atexit, re

parser = ArgumentParser()
parser.add_argument("-v", "--verbose", action="store_true")
parser.add_argument("-t", "--test", action="store_true")
parser.add_argument("--elapsed", action="store_true")
parser.add_argument("filename", nargs="?", type=Path, default="input.txt")
args = parser.parse_args()
if args.test:
    args.filename = Path("test.txt")

data = args.filename.read_text()

if args.elapsed:
    start_time_ns = time.time_ns()
    atexit.register(lambda: print(f"elapsed: {(time.time_ns() - start_time_ns) / 1_000_000}ms"))

EOF
    chmod a+x day$day.py

    printf "\033[32mPython script template created.\033[0m\n"
}

create_rust()
{
    has_feature rust || return 0

    if [ -f day$day.rs ]; then
        printf "\033[31mRust program already exists.\033[0m\n"
        return
    fi

    local title=$($rootdir/scripts/answers.py --get-title --year $year --day $day)

    cat <<EOF >day$day.rs
//! $title

struct Puzzle {
    //
}

impl Puzzle {
    /// Initialize from the puzzle input.
    const fn new(data: &str) -> Self {
        Self { }
    }

    /// Solve part one.
    fn part1(&self) -> i32 {
        0
    }

    /// Solve part two.
    fn part2(&self) -> i32 {
        0
    }
}

/// # Panics
#[must_use]
pub fn solve(data: &str) -> (i32, i32) {
    let puzzle = Puzzle::new(data);
    (puzzle.part1(), puzzle.part2())
}

pub fn main() {
    let args = aoc::parse_args();
    args.run(solve);
}

#[cfg(test)]
mod test {
    use super::*;

    // const TEST_INPUT: &str = include_str!("test.txt");
EOF

    for sample in sample_*.txt ; do
        local sample_name=$(echo ${sample%.txt} | tr "[:lower:]" "[:upper:]")
        cat <<EOF >>day$day.rs
    // const $sample_name: &str = include_str!("$sample");
EOF
    done

    cat <<EOF >>day$day.rs

    // #[test]
    // fn part1() {
    //     let puzzle = Puzzle::new(SAMPLE_1);
    //     assert_eq!(puzzle.part1(), 0);
    // }

    // #[test]
    // fn part2() {
    //     let puzzle = Puzzle::new(TEST_INPUT);
    //     assert_eq!(puzzle.part2(), 0);
    // }
}
EOF

    printf "\033[32mRust program template created.\033[0m\n"

    if [ ! -f Cargo.toml ]; then
        cat <<EOF >Cargo.toml
[package]
name = "day$day"
version = "0.1.0"
edition = "2021"

[dependencies]
aoc = { path = "../../../crates/aoc" }
rustc-hash = "*"
itertools = "*"

[[bin]]
name = "day$day"
path = "day$day.rs"
EOF
    fi
}



parse_args "$@"
fetch_input
fetch_samples
create_python
create_rust
