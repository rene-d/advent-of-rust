//! [Day 8: Resonant Collinearity](https://adventofcode.com/2024/day/8)

use itertools::Itertools;
use rustc_hash::{FxHashMap, FxHashSet};
use std::ops::RangeInclusive;

struct Puzzle {
    antennas: FxHashMap<char, Vec<(i32, i32)>>,
    width: RangeInclusive<i32>,
    height: RangeInclusive<i32>,
}

impl Puzzle {
    fn new(data: &str) -> Self {
        let mut antennas: FxHashMap<char, Vec<(i32, i32)>> = FxHashMap::default();

        let mut max_x = 0;
        let mut max_y = 0;

        for (y, line) in data.lines().enumerate() {
            let y: i32 = y.try_into().unwrap();
            for (x, c) in line.chars().enumerate() {
                let x: i32 = x.try_into().unwrap();
                if c != '.' {
                    antennas.entry(c).or_default().push((x, y));
                }
                max_x = x;
            }
            max_y = y;
        }

        Self {
            antennas,
            width: 0..=max_x,
            height: 0..=max_y,
        }
    }

    /// Solve part one.
    fn part1(&self) -> usize {
        let mut uniq = FxHashSet::default();

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
        let mut uniq = FxHashSet::default();

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

/// # Panics
#[must_use]
pub fn solve(data: &str) -> (usize, usize) {
    let puzzle = Puzzle::new(data);
    (puzzle.part1(), puzzle.part2())
}

pub fn main() {
    let args = aoc::parse_args();
    args.run(solve);
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = include_str!("test.txt");

    #[test]
    fn test01() {
        let puzzle = Puzzle::new(TEST_INPUT);
        assert_eq!(puzzle.part1(), 14);
    }

    #[test]
    fn test02() {
        let puzzle = Puzzle::new(TEST_INPUT);
        assert_eq!(puzzle.part2(), 34);
    }
}
