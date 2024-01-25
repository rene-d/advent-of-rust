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
            -c|--c) features+=("c") ;;
            *) args+=$i ;;
        esac
    done
    set -- "${args[@]}"

    if [[ $(basename $PWD) =~ day* ]]; then
        day=$(basename $PWD)
        day=${day/day/}
        year=$(basename $(realpath $PWD/..))
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

    session=$(awk '/^[^#].*/{ if (! session) session=$1 } END{print session}' < $rootdir/session)

    now=$(date -u +%Y%m%d%H%M%S)
    if [[ $now == ${year}12${day}050000 ]] || [[ $now > ${year}12${day}050000 ]] ; then
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

    session=$(awk '/^[^#].*/{ if (! session) session=$1 } END{print session}' < $rootdir/session)

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
from copy import deepcopy
from collections import defaultdict, deque, namedtuple, Counter
import sys, re, math, itertools, time
from functools import reduce
import re
import unittest

verbose = "-v" in sys.argv
if verbose:
    sys.argv.remove("-v")
self_tests = "-T" in sys.argv
if self_tests:
    sys.argv.remove("-T")
filename = ("test.txt" if sys.argv[1] == "-t" else sys.argv[1]) if len(sys.argv) > 1 else "input.txt"
data = Path(filename).read_text().strip()
lines = data.splitlines()
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

// use std::collections::{HashMap,HashSet};

struct Puzzle {
    data: String,
}

impl Puzzle {
    fn new() -> Puzzle {
        Puzzle {
            data: String::new(),
        }
    }

    /// Get the puzzle input.
    fn configure(&mut self, path: &str) {
        let data = std::fs::read_to_string(path).unwrap();

        self.data = data;
    }

    /// Solve part one.
    fn part1(&self) -> u32 {
        0
    }

    /// Solve part two.
    fn part2(&self) -> u32 {
        0
    }
}

fn main() {
    let args = aoc::parse_args();
    let mut puzzle = Puzzle::new();
    puzzle.configure(args.path.as_str());
    println!("{}", puzzle.part1());
    println!("{}", puzzle.part2());
}

/// Test from puzzle input
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test01() {
        let mut puzzle = Puzzle::new();
        puzzle.configure("test.txt");
        assert_eq!(puzzle.part1(), 0);
    }

    #[test]
    fn test02() {
        let mut puzzle = Puzzle::new();
        puzzle.configure("test.txt");
        assert_eq!(puzzle.part2(), 0);
    }
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
aoc = { path = "../../aoc" }

[[bin]]
name = "day$day"
path = "day$day.rs"
EOF
    fi
}

open_if_available()
{
    if [[ $available ]]; then
        if [[ -f day$day.py ]]; then
            code -n . day$day.py
        else
            code -n .
        fi
        open "https://adventofcode.com/$year/day/$day"
    fi
}


parse_args "$@"
fetch_input
fetch_samples
create_python
create_rust
# open_if_available
