//! [Day 4: High-Entropy Passphrases](https://adventofcode.com/2017/day/4)

use std::collections::HashSet;

struct Puzzle {
    lines: Vec<String>,
}

impl Puzzle {
    const fn new() -> Self {
        Self { lines: vec![] }
    }

    /// Get the puzzle input.
    fn configure(&mut self, data: &str) {
        self.lines = data.lines().map(str::to_string).collect();
    }

    /// Solve part one.
    fn part1(&self) -> usize {
        self.lines
            .iter()
            .filter(|line| {
                let a = line.split_ascii_whitespace();
                let n = line.split_ascii_whitespace().count();

                a.collect::<HashSet<_>>().len() == n
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

                a.iter().collect::<HashSet<_>>().len() == a.len()
            })
            .count()
    }
}

fn main() {
    let args = aoc::parse_args();
    let mut puzzle = Puzzle::new();
    puzzle.configure(&args.input);
    println!("{}", puzzle.part1());
    println!("{}", puzzle.part2());
}
