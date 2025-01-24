//! [Day 4: High-Entropy Passphrases](https://adventofcode.com/2017/day/4)

use rustc_hash::FxHashSet;

struct Puzzle {
    lines: Vec<String>,
}

impl Puzzle {
    fn new(data: &str) -> Self {
        Self {
            lines: data.lines().map(str::to_string).collect(),
        }
    }

    /// Solve part one.
    fn part1(&self) -> usize {
        self.lines
            .iter()
            .filter(|line| {
                let a = line.split_ascii_whitespace();
                let n = line.split_ascii_whitespace().count();

                a.collect::<FxHashSet<_>>().len() == n
            })
            .count()
    }

    /// Solve part two.
    fn part2(&self) -> usize {
        self.lines
            .iter()
            .filter(|line| {
                let a: Vec<_> = line
                    .split_ascii_whitespace()
                    .map(|x| {
                        let mut x: Vec<_> = x.chars().collect();
                        x.sort_unstable();
                        x
                    })
                    .collect();

                a.iter().collect::<FxHashSet<_>>().len() == a.len()
            })
            .count()
    }
}

/// # Panics
#[must_use]
pub fn solve(data: &str) -> (usize, usize) {
    let puzzle = Puzzle::new(data);
    (puzzle.part1(), puzzle.part2())
}

pub fn main() {
    let args = aoc::parse_args();
    args.run(solve);
}
