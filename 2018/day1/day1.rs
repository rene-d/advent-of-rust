//! [Day 1: xxx](https://adventofcode.com/2018/day/1)

use std::collections::HashSet;

use clap::Parser;

#[derive(Parser)]
struct Args {
    /// Puzzle input
    #[arg(default_value = "input.txt")]
    path: String,
}

struct Puzzle {
    data: String,
}

impl Puzzle {
    fn new() -> Puzzle {
        Puzzle {
            data: String::new(),
        }
    }

    /// Get the puzzle input.
    fn configure(&mut self, path: &str) {
        let data = std::fs::read_to_string(path).unwrap();

        self.data = data;
    }

    /// Solve part one.
    fn part1(&self) -> i32 {
        self.data.lines().map(|x| x.parse::<i32>().unwrap()).sum()
    }

    /// Solve part two.
    fn part2(&self) -> i32 {
        let mut frequencies = HashSet::new();
        let mut sum = 0;
        loop {
            for i in self.data.lines() {
                sum += i.parse::<i32>().unwrap();
                if frequencies.contains(&sum) {
                    return sum;
                }
                frequencies.insert(sum);
            }
        }
    }
}

/// Test from puzzle input
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test01() {
        let mut puzzle = Puzzle::new();
        puzzle.configure("test1.txt");
        assert_eq!(puzzle.part1(), -6);
    }

    #[test]
    fn test02() {
        let mut puzzle = Puzzle::new();
        puzzle.configure("test2.txt");
        assert_eq!(puzzle.part2(), 14);
    }
}

fn main() {
    let args = Args::parse();
    let mut puzzle = Puzzle::new();
    puzzle.configure(args.path.as_str());
    println!("{}", puzzle.part1());
    println!("{}", puzzle.part2());
}
