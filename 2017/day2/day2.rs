//! [Day 2: Corruption Checksum](https://adventofcode.com/2017/day/2)

use clap::Parser;

#[derive(Parser)]
struct Args {
    /// Puzzle input
    #[arg(default_value = "input.txt")]
    path: String,
}

struct Puzzle {
    data: Vec<Vec<u32>>,
}

impl Puzzle {
    fn new() -> Puzzle {
        Puzzle { data: vec![] }
    }

    /// Get the puzzle input.
    fn configure(&mut self, path: &str) {
        let data = std::fs::read_to_string(path).unwrap();

        for line in data.lines() {
            self.data.push(
                line.split_ascii_whitespace()
                    .map(|x| x.parse().unwrap())
                    .collect(),
            );
        }
    }

    /// Solve part one.
    fn part1(&self) -> u32 {
        self.data
            .iter()
            .map(|row| row.iter().max().unwrap() - row.iter().min().unwrap())
            .sum()
    }

    /// Solve part two.
    fn part2(&self) -> u32 {
        self.data
            .iter()
            .map(|row| {
                for a in row {
                    for b in row {
                        if a > b && a % b == 0 {
                            return a / b;
                        }
                    }
                }
                0
            })
            .sum()
    }
}

fn main() {
    let args = Args::parse();
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
        puzzle.configure("sample_1.txt");
        assert_eq!(puzzle.part1(), 18);
    }

    #[test]
    fn test02() {
        let mut puzzle = Puzzle::new();
        puzzle.configure("sample_2.txt");
        assert_eq!(puzzle.part2(), 9);
    }
}
