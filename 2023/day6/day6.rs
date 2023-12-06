//! [Day 6: xxx](https://adventofcode.com/2023/day/6)

use clap::Parser;

#[derive(Parser)]
struct Args {
    /// Puzzle input
    #[arg(default_value = "input.txt")]
    path: String,
}

struct Puzzle {
    time: String,
    distance: String,
}

impl Puzzle {
    fn new() -> Puzzle {
        Puzzle {
            time: "".to_string(),
            distance: "".to_string(),
        }
    }

    /// Get the puzzle input.
    fn configure(&mut self, path: &str) {
        let data = std::fs::read_to_string(path).unwrap();

        let mut lines = data.lines();
        self.time = lines
            .next()
            .unwrap()
            .strip_prefix("Time:")
            .unwrap()
            .to_string();
        self.distance = lines
            .next()
            .unwrap()
            .strip_prefix("Distance:")
            .unwrap()
            .to_string();
    }

    fn win(&self, t: u64, d: u64) -> u64 {
        let mut win = 0;
        for hold in 1..t {
            let speed = hold;
            let remaining = t - hold;
            let travelled = speed * remaining;
            if travelled > d {
                win += 1;
            }
        }
        win
    }

    /// Solve part one.
    fn part1(&self) -> u64 {
        let time: Vec<u64> = self
            .time
            .split_whitespace()
            .map(|x| x.parse::<u64>().unwrap())
            .collect();
        let distance: Vec<u64> = self
            .distance
            .split_whitespace()
            .map(|x| x.parse::<u64>().unwrap())
            .collect();

        let mut result = 1;
        for (&t, &d) in time.iter().zip(distance.iter()) {
            result *= self.win(t, d);
        }
        result
    }

    /// Solve part two.
    fn part2(&self) -> u64 {
        let time = self.time.replace(" ", "").parse::<u64>().unwrap();
        let distance = self.distance.replace(" ", "").parse::<u64>().unwrap();

        self.win(time, distance)
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
        assert_eq!(puzzle.part1(), 288);
    }

    #[test]
    fn test02() {
        let mut puzzle = Puzzle::new();
        puzzle.configure("test.txt");
        assert_eq!(puzzle.part2(), 71503);
    }
}

fn main() {
    let args = Args::parse();
    let mut puzzle = Puzzle::new();
    puzzle.configure(args.path.as_str());
    println!("{}", puzzle.part1());
    println!("{}", puzzle.part2());
}
