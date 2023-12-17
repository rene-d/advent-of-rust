//! [Day 13: xxx](https://adventofcode.com/2017/day/13)

use clap::Parser;
use std::collections::HashMap;

#[derive(Parser)]
struct Args {
    /// Puzzle input
    #[arg(default_value = "input.txt")]
    path: String,
}

struct Puzzle {
    heights: HashMap<u32, u32>,
}

impl Puzzle {
    fn new() -> Puzzle {
        Puzzle {
            heights: HashMap::new(),
        }
    }

    /// Get the puzzle input.
    fn configure(&mut self, path: &str) {
        let data = std::fs::read_to_string(path).unwrap();

        for line in data.lines() {
            let mut line = line.split(": ");

            let pos: u32 = line.next().unwrap().parse().unwrap();
            let height: u32 = line.next().unwrap().parse().unwrap();

            self.heights.insert(pos, height);
        }
    }

    /// Solve part one.
    fn part1(&self) -> u32 {
        self.heights
            .iter()
            .filter(|&(&pos, &height)| pos % (2 * (height - 1)) == 0)
            .map(|(&pos, &height)| pos * height)
            .sum()
    }

    /// Solve part two.
    fn part2(&self) -> u32 {
        (0..10_000_000)
            .find(|wait| {
                !self
                    .heights
                    .iter()
                    .any(|(&pos, &height)| (wait + pos) % (2 * (height - 1)) == 0)
            })
            .unwrap()
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
        assert_eq!(puzzle.part1(), 24);
    }

    #[test]
    fn test02() {
        let mut puzzle = Puzzle::new();
        puzzle.configure("test.txt");
        assert_eq!(puzzle.part2(), 10);
    }
}

fn main() {
    let args = Args::parse();
    let mut puzzle = Puzzle::new();
    puzzle.configure(args.path.as_str());
    println!("{}", puzzle.part1());
    println!("{}", puzzle.part2());
}