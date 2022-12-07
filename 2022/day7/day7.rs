//! [Day 7: No Space Left On Device](https://adventofcode.com/2022/day/7)

use clap::Parser;

#[derive(Parser)]
struct Args {
    /// Puzzle input
    #[arg(default_value = "input.txt")]
    path: String,
}

struct Puzzle {
    /// Puzzle input
    data: String,
}

impl Puzzle {
    const fn new() -> Self {
        Self {
            data: String::new(),
        }
    }

    /// Loads data from input (one line)
    fn configure(&mut self, path: &str) {
        self.data = std::fs::read_to_string(path).unwrap();
    }

    // Solves part one
    fn part1(&self) -> usize {
        0
    }

    // Solve part two
    fn part2(&self) -> usize {
        0
    }
}

/// main function
fn main() {
    let args = Args::parse();
    let mut puzzle = Puzzle::new();
    puzzle.configure(&args.path);
    println!("{}", puzzle.part1());
    println!("{}", puzzle.part2());
}

#[test]
fn test01() {
    let mut puzzle = Puzzle::new();
    puzzle.configure("test.txt");
    assert_eq!(puzzle.part1(), 0);
    assert_eq!(puzzle.part2(), 0);
}
