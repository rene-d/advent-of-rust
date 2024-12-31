//! [Day 8: Resonant Collinearity](https://adventofcode.com/2024/day/8)

use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    ops::RangeInclusive,
};

struct Puzzle {
    antennas: HashMap<char, Vec<(i32, i32)>>,
    width: RangeInclusive<i32>,
    height: RangeInclusive<i32>,
}

impl Puzzle {
    fn new() -> Self {
        Self {
            antennas: HashMap::new(),
            width: 0..=0,
            height: 0..=0,
        }
    }

    /// Get the puzzle input.
    fn configure(&mut self, data: &str) {
        let mut max_x = 0;
        let mut max_y = 0;

        for (y, line) in data.lines().enumerate() {
            let y: i32 = y.try_into().unwrap();
            for (x, c) in line.chars().enumerate() {
                let x: i32 = x.try_into().unwrap();
                if c != '.' {
                    self.antennas.entry(c).or_default().push((x, y));
                }
                max_x = x;
            }
            max_y = y;
        }

        self.width = 0..=max_x;
        self.height = 0..=max_y;
    }

    /// Solve part one.
    fn part1(&self) -> usize {
        let mut uniq = HashSet::new();

        for positions in self.antennas.values() {
            for it in positions.iter().combinations(2) {
                let a = it[0];
                let b = it[1];

                let x = a.0 - (b.0 - a.0);
                let y = a.1 - (b.1 - a.1);
                if self.width.contains(&x) && self.height.contains(&y) {
                    uniq.insert((x, y));
                }

                let x = b.0 + (b.0 - a.0);
                let y = b.1 + (b.1 - a.1);
                if self.width.contains(&x) && self.height.contains(&y) {
                    uniq.insert((x, y));
                }
            }
        }

        uniq.len()
    }

    /// Solve part two.
    fn part2(&self) -> usize {
        let mut uniq = HashSet::new();

        for positions in self.antennas.values() {
            for it in positions.iter().combinations(2) {
                let a = it[0];
                let b = it[1];
                let dx = b.0 - a.0;
                let dy = b.1 - a.1;

                for n in 0.. {
                    let x = a.0 - n * dx;
                    let y = a.1 - n * dy;
                    if !(self.width.contains(&x) && self.height.contains(&y)) {
                        break;
                    }
                    uniq.insert((x, y));
                }

                for n in 0.. {
                    let x = b.0 + n * dx;
                    let y = b.1 + n * dy;
                    if !(self.width.contains(&x) && self.height.contains(&y)) {
                        break;
                    }
                    uniq.insert((x, y));
                }
            }
        }

        uniq.len()
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
        let data = aoc::load_input_data("test.txt");
        puzzle.configure(&data);
        assert_eq!(puzzle.part1(), 14);
    }

    #[test]
    fn test02() {
        let mut puzzle = Puzzle::new();
        let data = aoc::load_input_data("test.txt");
        puzzle.configure(&data);
        assert_eq!(puzzle.part2(), 34);
    }
}
