//! [Day 1: Report Repair](https://adventofcode.com/2020/day/1)

// use std::collections::{HashMap,HashSet};

struct Puzzle {
    expenses: Vec<u64>,
}

impl Puzzle {
    fn new() -> Puzzle {
        Puzzle { expenses: vec![] }
    }

    /// Get the puzzle input.
    fn configure(&mut self, path: &str) {
        let data = std::fs::read_to_string(path).unwrap();

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
        assert_eq!(puzzle.part1(), 514579);
    }

    #[test]
    fn test02() {
        let mut puzzle = Puzzle::new();
        puzzle.configure("test.txt");
        assert_eq!(puzzle.part2(), 241861950);
    }
}
