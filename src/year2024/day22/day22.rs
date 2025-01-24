//! [Day 22: Monkey Market](https://adventofcode.com/2024/day/22)

use rustc_hash::{FxHashMap, FxHashSet};

const fn next_secret(secret: i64) -> i64 {
    let secret = (secret ^ (secret * 64)) % 16_777_216;
    let secret = (secret ^ (secret / 32)) % 16_777_216;
    (secret ^ (secret * 2048)) % 16_777_216
}

struct Puzzle {
    initial_secrets: Vec<i64>,
}

impl Puzzle {
    fn new(data: &str) -> Self {
        Self {
            initial_secrets: data.lines().map_while(|s| s.parse::<i64>().ok()).collect(),
        }
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
        let mut bananas = FxHashMap::default();

        for &initial_secret in &self.initial_secrets {
            let mut prices = Vec::new();

            let mut secret = initial_secret;
            prices.push(secret % 10);
            for _ in 0..2000 {
                secret = next_secret(secret);
                prices.push(secret % 10);
            }

            let mut seen = FxHashSet::default();
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

/// # Panics
#[must_use]
pub fn solve(data: &str) -> (i64, i64) {
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

    const TEST_INPUT: &str = include_str!("test.txt");

    #[test]
    fn test_part1() {
        let puzzle = Puzzle::new(TEST_INPUT);
        assert_eq!(puzzle.part1(), 37327623);
    }

    #[test]
    fn test_part2() {
        let puzzle = Puzzle::new(TEST_INPUT);
        assert_eq!(puzzle.part2(), 23 + 1);
    }
}
