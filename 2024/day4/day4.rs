//! [Day 4: Ceres Search](https://adventofcode.com/2024/day/4)

// use std::collections::{HashMap,HashSet};
use aoc::{grid, grid::Grid};

struct Puzzle {
    grid: Grid<char>,
}

impl Puzzle {
    fn new() -> Puzzle {
        Puzzle { grid: grid![] }
    }

    /// Get the puzzle input.
    fn configure(&mut self, path: &str) {
        let data = std::fs::read_to_string(path).unwrap();

        self.grid = aoc::grid::Grid::<char>::parse(&data);
    }

    /// Solve part one.
    fn part1(&self) -> u32 {
        let mut n = 0;
        let (sx, sy) = self.grid.size();
        for ((x, y), _p) in self.grid.iter().filter(|(_, p)| *p == &'X') {
            //
            if x <= sx - 4 && (1..4).all(|i| self.grid[(x + i, y)] == ['M', 'A', 'S'][i - 1]) {
                n += 1;
            }
            if y <= sy - 4 && (1..4).all(|i| self.grid[(x, y + i)] == ['M', 'A', 'S'][i - 1]) {
                n += 1;
            }

            if x >= 3 && (1..4).all(|i| self.grid[(x - i, y)] == ['M', 'A', 'S'][i - 1]) {
                n += 1;
            }
            if y >= 3 && (1..4).all(|i| self.grid[(x, y - i)] == ['M', 'A', 'S'][i - 1]) {
                n += 1;
            }

            if x <= sx - 4
                && y <= sy - 4
                && (1..4).all(|i| self.grid[(x + i, y + i)] == ['M', 'A', 'S'][i - 1])
            {
                n += 1;
            }
            if x >= 3
                && y >= 3
                && (1..4).all(|i| self.grid[(x - i, y - i)] == ['M', 'A', 'S'][i - 1])
            {
                n += 1;
            }

            if x <= sx - 4
                && y >= 3
                && (1..4).all(|i| self.grid[(x + i, y - i)] == ['M', 'A', 'S'][i - 1])
            {
                n += 1;
            }
            if x >= 3
                && y <= sy - 4
                && (1..4).all(|i| self.grid[(x - i, y + i)] == ['M', 'A', 'S'][i - 1])
            {
                n += 1;
            }
        }
        n
    }

    /// Solve part two.
    fn part2(&self) -> u32 {
        let mut n = 0;
        let (sx, sy) = self.grid.size();
        for x in 1..(sx - 1) {
            for y in 1..(sy - 1) {
                if self.grid[(x, y)] == 'A' {
                    let ul = self.grid[(x - 1, y - 1)];
                    let ur = self.grid[(x + 1, y - 1)];
                    let bl = self.grid[(x - 1, y + 1)];
                    let br = self.grid[(x + 1, y + 1)];

                    if ((ul == 'M' && br == 'S') || (ul == 'S' && br == 'M'))
                        && ((ur == 'M' && bl == 'S') || (ur == 'S' && bl == 'M'))
                    {
                        n += 1;
                    }
                }
            }
        }
        n
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
    fn test_p1_1() {
        let mut puzzle = Puzzle::new();
        puzzle.configure("sample_1.txt");
        assert_eq!(puzzle.part1(), 4);
    }

    #[test]
    fn test_p1_2() {
        let mut puzzle = Puzzle::new();

        puzzle.configure("sample_2.txt");
        assert_eq!(puzzle.part1(), 18);

        puzzle.configure("sample_3.txt");
        assert_eq!(puzzle.part1(), 18);
    }

    #[test]
    fn test_p2_1() {
        let mut puzzle = Puzzle::new();
        puzzle.configure("sample_4.txt");
        assert_eq!(puzzle.part2(), 1);
    }

    #[test]
    fn test_p2_2() {
        let mut puzzle = Puzzle::new();

        puzzle.configure("sample_2.txt");
        assert_eq!(puzzle.part2(), 9);

        puzzle.configure("sample_5.txt");
        assert_eq!(puzzle.part2(), 9);
    }
}
