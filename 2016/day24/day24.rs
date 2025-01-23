//! [Day 24: Air Duct Spelunking](https://adventofcode.com/2016/day/24)

#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_possible_wrap)]

use itertools::Itertools;
use rustc_hash::FxHashSet;
use std::collections::VecDeque;

struct Puzzle {
    grid: aoc::Grid<char>,
}

impl Puzzle {
    fn new(data: &str) -> Self {
        Self {
            grid: aoc::Grid::<char>::parse(data),
        }
    }

    fn solve(&self) -> (u32, u32) {
        let mut points = vec![];

        for (xy, c) in &self.grid {
            if c.is_ascii_digit() {
                points.push(xy);
            }
        }

        let n = points.len();
        let mut distances = aoc::Grid::<u32>::with_size(n as i32, n as i32, 0, 0);

        for &start in &points {
            let from = self.grid[start].to_digit(10).unwrap();

            let mut seen = FxHashSet::default();
            let mut q = VecDeque::new();
            q.push_back((start, 0u32));

            while let Some((point, steps)) = q.pop_front() {
                // distance from 'start' to current point
                if let Some(to) = self.grid[point].to_digit(10) {
                    distances[(from as i32, to as i32)] = steps;
                }

                // walk if possible north, east, south and west
                self.grid.iter_directions(point).for_each(|(_, p)| {
                    if self.grid[p] != '#' && !seen.contains(&p) {
                        seen.insert(p);
                        q.push_back((p, steps + 1));
                    }
                });
            }
        }

        let mut part1 = u32::MAX;
        let mut part2 = u32::MAX;

        (1..n).permutations(n - 1).for_each(|path| {
            let first = distances[(0, path[0] as i32)];
            let middle: u32 = path.windows(2).map(|x| distances[(x[0] as i32, x[1] as i32)]).sum();
            let last = distances[(path[n - 2] as i32, 0)];

            part1 = part1.min(first + middle);
            part2 = part2.min(first + middle + last);
        });

        (part1, part2)
    }

    /// Solve part one.
    fn part1(&self) -> u32 {
        self.solve().0
    }

    /// Solve part two.
    fn part2(&self) -> u32 {
        self.solve().1
    }
}

/// # Panics
/// over malformed input
#[must_use]
pub fn solve(data: &str) -> (u32, u32) {
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

    const SAMPLE_1: &str = include_str!("sample_1.txt");

    #[test]
    fn test01() {
        let puzzle = Puzzle::new(SAMPLE_1);
        assert_eq!(puzzle.part1(), 14);
    }

    #[test]
    fn test02() {
        let puzzle = Puzzle::new(SAMPLE_1);
        assert_eq!(puzzle.part2(), 20);
    }
}
