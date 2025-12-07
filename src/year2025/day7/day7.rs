//! [Day 7: Laboratories](https://adventofcode.com/2025/day/7)

use rustc_hash::{FxHashMap, FxHashSet};

const START: u8 = b'S';
const SPLITTER: u8 = b'^';

struct Puzzle {
    grid: aoc::Grid<u8>,
    start: aoc::Coord,
}

impl Puzzle {
    /// Initialize from the puzzle input.
    fn new(data: &str) -> Self {
        let grid = aoc::Grid::<u8>::parse(data);

        let start = grid.iter().find(|&(_, v)| v == &START).unwrap().0;

        Self { grid, start }
    }

    /// Solve part one.
    fn part1(&self) -> u64 {
        let mut splits = 0;

        let mut beams = FxHashSet::default();

        beams.insert(self.start.x);

        for y in self.start.y..self.grid.height() {
            let mut next_beams = FxHashSet::default();

            for x in beams {
                if self.grid[(x, y)] == SPLITTER {
                    splits += 1;

                    if x > 0 {
                        next_beams.insert(x - 1);
                    }

                    if x < self.grid.width() - 1 {
                        next_beams.insert(x + 1);
                    }
                } else {
                    next_beams.insert(x);
                }
            }

            beams = next_beams;
        }

        splits
    }

    /// Solve part two.
    fn part2(&self) -> u64 {
        let mut timelines = FxHashMap::default();

        timelines.insert(self.start.x, 1);

        for y in self.start.y..self.grid.height() {
            let mut next_timelines = FxHashMap::default();

            for (&x, ways) in &timelines {
                if self.grid[(x, y)] == SPLITTER {
                    if x > 0 {
                        *next_timelines.entry(x - 1).or_insert(0) += ways;
                    }

                    if x < self.grid.width() - 1 {
                        *next_timelines.entry(x + 1).or_insert(0) += ways;
                    }
                } else {
                    *next_timelines.entry(x).or_insert(0) += ways;
                }
            }

            timelines = next_timelines;
        }

        timelines.into_values().sum()
    }
}

/// # Panics
#[must_use]
pub fn solve(data: &str) -> (u64, u64) {
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
    fn part1() {
        let puzzle = Puzzle::new(TEST_INPUT);
        assert_eq!(puzzle.part1(), 21);
    }

    #[test]
    fn part2() {
        let puzzle = Puzzle::new(TEST_INPUT);
        assert_eq!(puzzle.part2(), 40);
    }
}
