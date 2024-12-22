//! [Day 24: Air Duct Spelunking](https://adventofcode.com/2016/day/24)

use aoc::{grid, grid::Grid};

use itertools::Itertools;
use std::collections::{HashSet, VecDeque};

struct Puzzle {
    grid: Grid<char>,
}

impl Puzzle {
    fn new() -> Self {
        Self { grid: grid![] }
    }

    /// Get the puzzle input.
    fn configure(&mut self, path: &str) {
        let data = std::fs::read_to_string(path).unwrap();

        self.grid = aoc::grid::Grid::<char>::parse(&data);
    }

    fn solve(&self) -> (u32, u32) {
        let mut points = vec![];

        for (xy, c) in self.grid.iter() {
            if c.is_ascii_digit() {
                points.push(xy);
            }
        }

        let n = points.len();
        let mut distances = Grid::<u32>::with_size(n, n);

        for &start in &points {
            let from = self.grid[start].to_digit(10).unwrap();

            let mut seen = HashSet::new();
            let mut q = VecDeque::new();
            q.push_back((start, 0u32));

            while let Some((point, steps)) = q.pop_front() {
                // distance from 'start' to current point
                if let Some(to) = self.grid[point].to_digit(10) {
                    distances[(from as usize, to as usize)] = steps;
                }

                // walk if possible north, east, south and west
                self.grid.iter_directions(point).for_each(|p| {
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
            let first = distances[(0, path[0])];
            let middle: u32 = path.windows(2).map(|x| distances[(x[0], x[1])]).sum();
            let last = distances[(path[n - 2], 0)];

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

fn main() {
    let args = aoc::parse_args();
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
        puzzle.configure("sample_1.txt");
        assert_eq!(puzzle.part1(), 14);
    }

    #[test]
    fn test02() {
        let mut puzzle = Puzzle::new();
        puzzle.configure("sample_1.txt");
        assert_eq!(puzzle.part2(), 20);
    }
}
