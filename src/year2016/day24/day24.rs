//! [Day 24: Air Duct Spelunking](https://adventofcode.com/2016/day/24)

use itertools::Itertools;
use rustc_hash::FxHashSet;
use std::collections::VecDeque;

fn digit_usize(c: u8) -> Option<usize> {
    match c {
        b'0'..=b'9' => Some(usize::from(c - b'0')),
        _ => None,
    }
}

struct Puzzle {
    grid: aoc::GridU<u8>,
}

impl Puzzle {
    fn new(data: &str) -> Self {
        Self {
            grid: aoc::GridU::<u8>::parse(data),
        }
    }

    fn solve(&self) -> (usize, usize) {
        let mut points = vec![];

        for (xy, c) in self.grid.iter() {
            if c.is_ascii_digit() {
                points.push(xy);
            }
        }

        let n = points.len();
        let mut distances = aoc::GridU::<usize>::with_size(n, n);

        for &start in &points {
            let from = digit_usize(self.grid[start]).unwrap();

            let mut seen = FxHashSet::default();
            let mut q = VecDeque::new();
            q.push_back((start, 0));

            while let Some((point, steps)) = q.pop_front() {
                // distance from 'start' to current point
                if let Some(to) = digit_usize(self.grid[point]) {
                    distances[(from, to)] = steps;
                }

                // walk if possible north, east, south and west
                self.grid.iter_directions(point).for_each(|p| {
                    if self.grid[p] != b'#' && !seen.contains(&p) {
                        seen.insert(p);
                        q.push_back((p, steps + 1));
                    }
                });
            }
        }

        let mut part1 = usize::MAX;
        let mut part2 = usize::MAX;

        (1..n).permutations(n - 1).for_each(|path| {
            let first = distances[(0, path[0])];
            let middle: usize = path.windows(2).map(|x| distances[(x[0], x[1])]).sum();
            let last = distances[(path[n - 2], 0)];

            part1 = part1.min(first + middle);
            part2 = part2.min(first + middle + last);
        });

        (part1, part2)
    }

    /// Solve part one.
    fn part1(&self) -> usize {
        self.solve().0
    }

    /// Solve part two.
    fn part2(&self) -> usize {
        self.solve().1
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
