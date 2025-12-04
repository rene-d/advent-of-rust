//! [Day 4: Printing Department](https://adventofcode.com/2025/day/4)

use rustc_hash::FxHashSet;

const PAPER_ROLL: u8 = b'@';
const EMPTY: u8 = b'.';

struct Puzzle {
    grid: aoc::Grid<u8>,
}

impl Puzzle {
    /// Initialize from the puzzle input.
    fn new(data: &str) -> Self {
        Self {
            grid: aoc::Grid::<u8>::parse(data),
        }
    }

    /// Solve part one.
    fn part1(&self) -> i32 {
        let mut accessible = 0;

        for (pos, ch) in &self.grid {
            if ch == &PAPER_ROLL {
                let rolls = self
                    .grid
                    .iter_neighbors(pos)
                    .filter(|neigh| self.grid[(neigh.x, neigh.y)] == PAPER_ROLL)
                    .count();

                if rolls < 4 {
                    accessible += 1;
                }
            }
        }

        accessible
    }

    /// Solve part two.
    fn part2(&self) -> usize {
        let mut grid = self.grid.clone();
        let mut removed = 0;

        loop {
            let mut accessible = FxHashSet::default();

            for (pos, ch) in &grid {
                if ch == &PAPER_ROLL {
                    let rolls = grid
                        .iter_neighbors(pos)
                        .filter(|neigh| grid[(neigh.x, neigh.y)] == PAPER_ROLL)
                        .count();

                    if rolls < 4 {
                        accessible.insert(pos);
                    }
                }
            }

            if accessible.is_empty() {
                break;
            }

            removed += accessible.len();

            for pos in accessible {
                grid[(pos.x, pos.y)] = EMPTY;
            }
        }

        removed
    }
}

/// # Panics
#[must_use]
pub fn solve(data: &str) -> (i32, usize) {
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
        assert_eq!(puzzle.part1(), 13);
    }

    #[test]
    fn part2() {
        let puzzle = Puzzle::new(TEST_INPUT);
        assert_eq!(puzzle.part2(), 43);
    }
}
