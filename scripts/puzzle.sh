#!/usr/bin/env bash

set -euo pipefail

rootdir=$(realpath $(dirname $0)/..)

if [[ $(basename $PWD) =~ day* ]]; then
    day=$(basename $PWD)
    day=${day/day/}
    year=$(basename $(realpath $PWD/..))
else
    if [ $# -eq 0 ]; then
        echo "Usage: $0 [day]"
        exit
    fi

    year=$(basename $PWD)
    day=$1
    mkdir -p day$day
    cd day$day
fi


session=$(awk '/^[^#].*/{ if (! session) session='$day' } END{print session}' < $rootdir/session)

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

if [ -f day$day.py ]; then
    printf "\033[31mPython script already exists.\033[0m\n"
else
    cat <<EOF >day$day.py
#!/usr/bin/env python3
# https://adventofcode.com/$year/day/$day

from pathlib import Path
from copy import deepcopy
from collections import defaultdict, deque, namedtuple
import sys, re, math, itertools, time
from functools import reduce
import re

verbose = "-v" in sys.argv
if verbose:
    sys.argv.remove("-v")
filename = ("test.txt" if sys.argv[1] == "-t" else sys.argv[1]) if len(sys.argv) > 1 else "input.txt"
data = Path(filename).read_text().strip()
lines = data.splitlines()
EOF
    chmod a+x day$day.py

    printf "\033[32mPython script template created.\033[0m\n"
fi

if [ -f day$day.rs ]; then
    printf "\033[31mRust program already exists.\033[0m\n"
else
    cat <<EOF >day$day.rs
//! [Day $day: xxx](https://adventofcode.com/$year/day/$day)

use clap::Parser;

#[derive(Parser)]
struct Args {
    /// Puzzle input
    #[arg(default_value = "input.txt")]
    path: String,
}

struct Puzzle {
    data: String,
}

impl Puzzle {
    fn new() -> Puzzle {
        Puzzle {
            data: "".to_string(),
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

fn main() {
    let args = Args::parse();
    let mut puzzle = Puzzle::new();
    puzzle.configure(args.path.as_str());
    println!("{}", puzzle.part1());
    println!("{}", puzzle.part2());
}
EOF

    printf "\033[32mRust program template created.\033[0m\n"
fi

if [ ! -f Cargo.toml ]; then
    cat <<EOF >Cargo.toml
[package]
name = "day$day"
version = "0.1.0"
edition = "2021"

[dependencies]
clap = { version = "4.*", features = ["derive"] }

[[bin]]
name = "day$day"
path = "day$day.rs"
EOF
fi

if [[ $available ]]; then
    open "https://adventofcode.com/$year/day/$day"
    code --add . day$day.py
fi
