//! [Day 22: Monkey Market](https://adventofcode.com/2024/day/22)

use std::collections::{HashMap, HashSet};

const fn next_secret(secret: i64) -> i64 {
    let secret = (secret ^ (secret * 64)) % 16_777_216;
    let secret = (secret ^ (secret / 32)) % 16_777_216;
    (secret ^ (secret * 2048)) % 16_777_216
}

struct Puzzle {
    initial_secrets: Vec<i64>,
}

impl Puzzle {
    const fn new() -> Self {
        Self {
            initial_secrets: Vec::new(),
        }
    }

    /// Get the puzzle input.
    fn configure(&mut self, path: &str) {
        let data = std::fs::read_to_string(path).unwrap();

        self.initial_secrets
            .extend(data.lines().map_while(|s| s.parse::<i64>().ok()));
    }

    /// Solve part one.
    fn part1(&self) -> i64 {
        self.initial_secrets
            .iter()
            .map(|&initial_secret| (0..2000).fold(initial_secret, |secret, _| next_secret(secret)))
            .sum()
    }

    /// Solve part two.
    fn part2(&self) -> i64 {
        let mut bananas = HashMap::new();

        for &initial_secret in &self.initial_secrets {
            let mut prices = Vec::new();

            let mut secret = initial_secret;
            prices.push(secret % 10);
            for _ in 0..2000 {
                secret = next_secret(secret);
                prices.push(secret % 10);
            }

            let mut seen = HashSet::new();
            for p in prices.windows(5) {
                let sequence = [p[1] - p[0], p[2] - p[1], p[3] - p[2], p[4] - p[3]];

                if seen.insert(sequence) {
                    *bananas.entry(sequence).or_default() += p[4];
                }
            }
        }

        *bananas.values().max().unwrap()
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
        assert_eq!(puzzle.part1(), 37327623);
    }

    #[test]
    fn test02() {
        let mut puzzle = Puzzle::new();
        puzzle.configure("test.txt");
        assert_eq!(puzzle.part2(), 23 + 1);
    }
}
