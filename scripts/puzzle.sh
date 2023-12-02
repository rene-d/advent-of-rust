#!/usr/bin/env bash

set -euo pipefail

if [ $# -eq 0 ]; then
    echo "Usage: $0 [day]"
    exit
fi

year=$(basename $PWD)
session=$(awk '/^[^#].*/{ if (! session) session=$1 } END{print session}' < $(dirname $0)/../session)

mkdir -p day$1
cd day$1

curl "https://adventofcode.com/$year/day/$1/input" \
    -H "Cookie: session=$session" -o input.txt
head input.txt
wc -l input.txt

if [ ! -f day$1.py ]; then
    cat <<EOF >day$1.py
#!/usr/bin/env python3
# https://adventofcode.com/$year/day/$1

from pathlib import Path
from copy import deepcopy
from collections import defaultdict, deque, namedtuple
import sys, re, math, itertools, time
from functools import reduce
import re

filename = ("test.txt" if sys.argv[1] == "-t" else sys.argv[1]) if len(sys.argv) > 1 else "input.txt"
data = Path(filename).read_text().strip()
lines = data.splitlines()
EOF
    chmod a+x day$1.py
fi

if [ ! -f day$1.rs ]; then
    cat <<EOF >day$1.rs
//! [Day $1: xxx](https://adventofcode.com/$year/day/$1)

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
fi

if [ ! -f Cargo.toml ]; then
    cat <<EOF >Cargo.toml
[package]
name = "day$1"
version = "0.1.0"
edition = "2021"

[dependencies]
clap = { version = "4.*", features = ["derive"] }

[[bin]]
name = "day$1"
path = "day$1.rs"
EOF
fi

code --add . day$1.py
