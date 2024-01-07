//! [Day 4: Scratchcards](https://adventofcode.com/2023/day/4)

use std::collections::HashSet;

use clap::Parser;

#[derive(Parser)]
struct Args {
    /// Puzzle input
    #[arg(default_value = "input.txt")]
    path: String,
}

struct Puzzle {
    matching_cards: Vec<usize>,
}

impl Puzzle {
    fn new() -> Puzzle {
        Puzzle {
            matching_cards: vec![],
        }
    }

    /// Get the puzzle input.
    fn configure(&mut self, path: &str) {
        let data = std::fs::read_to_string(path).unwrap();

        for line in data.lines() {
            let line = line[line.find(':').unwrap() + 1..]
                .split('|')
                .collect::<Vec<_>>();

            let winning = line[0]
                .split_whitespace()
                .map(|x| x.parse::<u32>().unwrap())
                .collect::<HashSet<_>>();

            let nums = line[1]
                .split_whitespace()
                .map(|x| x.parse::<u32>().unwrap())
                .collect::<HashSet<_>>();

            let result = winning.intersection(&nums).count();

            self.matching_cards.push(result);
        }
    }

    /// Solve part one.
    fn part1(&self) -> usize {
        let mut sum = 0;
        for n in &self.matching_cards {
            if n >= &1 {
                sum += 2usize.pow(u32::try_from(*n).unwrap() - 1);
            }
        }
        sum
    }

    /// Solve part two.
    fn part2(&self) -> usize {
        let mut copies = vec![0usize; self.matching_cards.len()];

        for i in 0..self.matching_cards.len() {
            copies[i] += 1;

            let m = self.matching_cards[i];
            for j in (i + 1)..(i + 1 + m) {
                copies[j] += copies[i];
            }
        }

        copies.iter().sum()
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
        assert_eq!(puzzle.part1(), 13);
    }

    #[test]
    fn test02() {
        let mut puzzle = Puzzle::new();
        puzzle.configure("test.txt");
        assert_eq!(puzzle.part2(), 30);
    }
}

fn main() {
    let args = Args::parse();
    let mut puzzle = Puzzle::new();
    puzzle.configure(args.path.as_str());
    println!("{}", puzzle.part1());
    println!("{}", puzzle.part2());
}
