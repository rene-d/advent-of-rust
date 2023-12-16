//! [Day 9: Mirage Maintenance](https://adventofcode.com/2023/day/9)

use clap::Parser;

#[derive(Parser)]
struct Args {
    /// Puzzle input
    #[arg(default_value = "input.txt")]
    path: String,
}

struct Puzzle {
    histories: Vec<Vec<i32>>,
}

impl Puzzle {
    fn new() -> Puzzle {
        Puzzle { histories: vec![] }
    }

    /// Get the puzzle input.
    fn configure(&mut self, path: &str) {
        let data = std::fs::read_to_string(path).unwrap();

        for line in data.lines() {
            let history: Vec<_> = line
                .split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect();
            self.histories.push(history);
        }
    }

    fn history_diffs(history: &[i32]) -> Vec<Vec<i32>> {
        let mut diffs = vec![];

        let mut history = history.to_vec();

        loop {
            // stop when the sequence of differences has only zeros
            if history.iter().min().unwrap() == &0 && history.iter().max().unwrap() == &0 {
                return diffs;
            }

            diffs.push(history.clone());

            history = history
                .iter()
                .zip(history.iter().skip(1))
                .map(|(a, b)| b - a)
                .collect();
        }
    }

    /// Solve part one.
    fn part1(&self) -> i32 {
        let mut result = 0;

        for history in &self.histories {
            let diffs = Self::history_diffs(history);

            // sum of numbers at the right
            let n = diffs.iter().fold(0, |acc, x| acc + x.last().unwrap());

            result += n;
        }

        result
    }

    /// Solve part two.
    fn part2(&self) -> i32 {
        self.histories
            .iter()
            .map(|history| {
                Self::history_diffs(history)
                    .iter()
                    .rev()
                    .fold(0, |acc, x| x.first().unwrap() - acc)
            })
            .sum::<i32>()
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
        assert_eq!(puzzle.part1(), 114);
    }

    #[test]
    fn test02() {
        let mut puzzle = Puzzle::new();
        puzzle.configure("test.txt");
        assert_eq!(puzzle.part2(), 2);
    }
}

fn main() {
    let args = Args::parse();
    let mut puzzle = Puzzle::new();
    puzzle.configure(args.path.as_str());
    println!("{}", puzzle.part1());
    println!("{}", puzzle.part2());
}
