//! [Day 1: Historian Hysteria](https://adventofcode.com/2024/day/1)

use std::collections::HashMap;

struct Puzzle {
    left: Vec<i32>,
    right: Vec<i32>,
}

impl Puzzle {
    fn new() -> Puzzle {
        Puzzle {
            left: vec![],
            right: vec![],
        }
    }

    /// Get the puzzle input.
    fn configure(&mut self, path: &str) {
        let data = std::fs::read_to_string(path).unwrap();

        for line in data.lines() {
            let parts: Vec<i32> = line
                .split_whitespace()
                .filter_map(|s| s.parse().ok())
                .collect();

            if parts.len() == 2 {
                self.left.push(parts[0]);
                self.right.push(parts[1]);
            }
        }

        // Sort both arrays
        self.left.sort_unstable();
        self.right.sort_unstable();
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
        let mut right_count = HashMap::new();
        for &num in &self.right {
            *right_count.entry(num).or_insert(0) += 1;
        }

        self.left
            .iter()
            .map(|&a| a * right_count.get(&a).unwrap_or(&0))
            .sum()
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
        assert_eq!(puzzle.part1(), 11);
    }

    #[test]
    fn test02() {
        let mut puzzle = Puzzle::new();
        puzzle.configure("test.txt");
        assert_eq!(puzzle.part2(), 31);
    }
}
