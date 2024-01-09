//! [Day 10: Knot Hash](https://adventofcode.com/2017/day/10)

use aoc::knot::*;
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

        self.data = data.trim().to_owned();
    }

    /// Solve part one.
    fn part1(&self) -> u32 {
        let lengths: Vec<_> = self
            .data
            .split(',')
            .filter_map(|s| s.parse().ok())
            .collect();

        let mut skip = 0;
        let mut pos = 0;
        let mut sparse: Vec<u8> = (0..=255).collect();

        knot_tie(&lengths, &mut sparse, &mut skip, &mut pos);

        (sparse[0] as u32) * (sparse[1] as u32)
    }

    /// Solve part two.
    fn part2(&self) -> String {
        knot_hash(&self.data)
    }
}

fn main() {
    let args = Args::parse();
    let mut puzzle = Puzzle::new();
    puzzle.configure(args.path.as_str());
    println!("{}", puzzle.part1());
    println!("{}", puzzle.part2());
}
