//! [Day 17: Spinlock](https://adventofcode.com/2017/day/17)

use clap::Parser;

#[derive(Parser)]
struct Args {
    /// Puzzle input
    #[arg(default_value = "input.txt")]
    path: String,
}

struct Puzzle {
    step: usize,
}

impl Puzzle {
    fn new() -> Puzzle {
        Puzzle { step: 0 }
    }

    /// Get the puzzle input.
    fn configure(&mut self, path: &str) {
        let data = std::fs::read_to_string(path).unwrap();

        self.step = data.trim().parse().unwrap();
    }

    /// Solve part one.
    fn part1(&self) -> u32 {
        let mut buf = vec![0];
        let mut pos = 0;

        for i in 1..=2017 {
            pos = (pos + self.step) % buf.len() + 1;
            buf.insert(pos, i);
        }

        buf[pos + 1]
    }

    /// Solve part two.
    fn part2(&self) -> usize {
        let mut buflen = 1;
        let mut pos = 0;
        let mut result = 0;

        for i in 1..50_000_000 {
            pos = (pos + self.step) % buflen + 1;
            buflen += 1;
            if pos == 1 {
                result = i;
            }
        }

        result
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
        puzzle.step = 3;
        assert_eq!(puzzle.part1(), 638);
    }
}
