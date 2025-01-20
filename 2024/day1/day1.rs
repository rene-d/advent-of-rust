//! [Day 1: Historian Hysteria](https://adventofcode.com/2024/day/1)

use rustc_hash::FxHashMap;

struct Puzzle {
    left: Vec<i32>,
    right: Vec<i32>,
}

impl Puzzle {
    fn new(data: &str) -> Self {
        let mut left = Vec::new();
        let mut right = Vec::new();

        for line in data.lines() {
            let parts: Vec<i32> = line
                .split_whitespace()
                .filter_map(|s| s.parse().ok())
                .collect();

            if parts.len() == 2 {
                left.push(parts[0]);
                right.push(parts[1]);
            }
        }

        // Sort both arrays
        left.sort_unstable();
        right.sort_unstable();

        Self { left, right }
    }

    /// Solve part one.
    fn part1(&self) -> i32 {
        self.left
            .iter()
            .zip(self.right.iter())
            .map(|(a, b)| (a - b).abs())
            .sum()
    }

    /// Solve part two.
    fn part2(&self) -> i32 {
        let mut right_count = FxHashMap::default();
        for &num in &self.right {
            *right_count.entry(num).or_insert(0) += 1;
        }

        self.left
            .iter()
            .map(|&a| a * right_count.get(&a).unwrap_or(&0))
            .sum()
    }
}

fn solve(data: &str) -> (i32, i32) {
    let puzzle = Puzzle::new(data);
    (puzzle.part1(), puzzle.part2())
}

fn main() {
    let mut args = aoc::parse_args();
    args.run(solve);
}

/// Test from puzzle input
#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = include_str!("test.txt");

    #[test]
    fn test01() {
        let puzzle = Puzzle::new(TEST_INPUT);
        assert_eq!(puzzle.part1(), 11);
    }

    #[test]
    fn test02() {
        let puzzle = Puzzle::new(TEST_INPUT);
        assert_eq!(puzzle.part2(), 31);
    }
}
