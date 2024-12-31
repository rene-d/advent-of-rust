//! [Day 13: Packet Scanners](https://adventofcode.com/2017/day/13)

use std::collections::HashMap;

struct Puzzle {
    heights: HashMap<u32, u32>,
}

impl Puzzle {
    fn new() -> Self {
        Self {
            heights: HashMap::new(),
        }
    }

    /// Get the puzzle input.
    fn configure(&mut self, data: &str) {
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

fn main() {
    let args = aoc::parse_args();
    let mut puzzle = Puzzle::new();
    puzzle.configure(&args.input);
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
        puzzle.configure(&aoc::load_input_data("test.txt"));
        assert_eq!(puzzle.part1(), 24);
    }

    #[test]
    fn test02() {
        let mut puzzle = Puzzle::new();
        puzzle.configure(&aoc::load_input_data("test.txt"));
        assert_eq!(puzzle.part2(), 10);
    }
}
