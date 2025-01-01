//! [Day 1: Report Repair](https://adventofcode.com/2020/day/1)

// use std::collections::{FxHashMap,FxHashSet};

struct Puzzle {
    expenses: Vec<u64>,
}

impl Puzzle {
    const fn new() -> Self {
        Self { expenses: vec![] }
    }

    /// Get the puzzle input.
    fn configure(&mut self, data: &str) {
        self.expenses = data.lines().map(|s| s.parse().unwrap()).collect();
    }

    /// Solve part one.
    fn part1(&self) -> u64 {
        for i in &self.expenses {
            for j in &self.expenses {
                if i + j == 2020 {
                    return i * j;
                }
            }
        }
        0
    }

    /// Solve part two.
    fn part2(&self) -> u64 {
        for i in &self.expenses {
            for j in &self.expenses {
                for k in &self.expenses {
                    if i + j + k == 2020 {
                        return i * j * k;
                    }
                }
            }
        }
        0
    }
}

fn main() {
    let args = aoc::parse_args();
    let mut puzzle = Puzzle::new();
    puzzle.configure(&args.input);
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
        puzzle.configure(&aoc::load_input_data("test.txt"));
        assert_eq!(puzzle.part1(), 514579);
    }

    #[test]
    fn test02() {
        let mut puzzle = Puzzle::new();
        puzzle.configure(&aoc::load_input_data("test.txt"));
        assert_eq!(puzzle.part2(), 241861950);
    }
}
