//! [Day 3: Gear Ratios](https://adventofcode.com/2023/day/3)

#![allow(clippy::cast_sign_loss)]
#![allow(clippy::cast_possible_wrap)]
#![allow(clippy::cast_possible_truncation)]

use rustc_hash::FxHashMap;

struct Puzzle {
    sx: i32,
    sy: i32,
    grid: Vec<String>,
    sum_parts: u64,
    gears: FxHashMap<[i32; 2], Vec<u64>>,
}

impl Puzzle {
    fn new(data: &str) -> Self {
        let grid = data.lines().map(String::from).collect::<Vec<_>>();
        Self {
            sx: grid[0].len() as i32,
            sy: grid.len() as i32,
            grid,
            sum_parts: 0,
            gears: FxHashMap::default(),
        }
    }

    /// Access the engine schematic
    fn g(&self, x: i32, y: i32) -> char {
        if 0 <= x && x < self.sx && 0 <= y && y < self.sy {
            self.grid[y as usize].chars().nth(x as usize).unwrap()
        } else {
            '.'
        }
    }

    /// Read the schematic to find part numbers and gears
    fn parse(&mut self) {
        for y in 0..self.sy {
            let mut x = 0;

            while x < self.sx {
                let mut symbol = false;
                let mut gear = [0, 0];

                let mut n = 0;
                while let Some(d) = self.g(x, y).to_digit(10) {
                    n = n * 10 + u64::from(d);

                    for (ix, iy) in [
                        (-1, -1),
                        (-1, 0),
                        (-1, 1),
                        (0, -1),
                        (0, 1),
                        (1, -1),
                        (1, 0),
                        (1, 1),
                    ] {
                        let c = self.g(x + ix, y + iy);
                        if c != '.' && !c.is_ascii_digit() {
                            symbol = true;
                            if c == '*' {
                                // assert we have only one gear near a part number
                                assert!(!(gear != [0, 0] && gear != [x + ix, y + iy]),);
                                gear = [x + ix, y + iy];
                            }
                        }
                    }

                    x += 1;
                }

                if symbol {
                    self.sum_parts += n;
                }

                if gear != [0, 0] {
                    self.gears.entry(gear).or_default().push(n);
                }

                x += 1;
            }
        }
    }

    /// Solve part one.
    const fn part1(&self) -> u64 {
        self.sum_parts
    }

    /// Solve part two.
    fn part2(&self) -> u64 {
        let mut gear_ratios = 0;

        for parts in self.gears.values() {
            if parts.len() == 2 {
                gear_ratios += parts[0] * parts[1];
            }
        }

        gear_ratios
    }
}

/// # Panics
/// over malformed input
#[must_use]
pub fn solve(data: &str) -> (u64, u64) {
    let mut puzzle = Puzzle::new(data);
    puzzle.parse();
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
        let mut puzzle = Puzzle::new(TEST_INPUT);
        puzzle.parse();
        assert_eq!(puzzle.part1(), 4361);
    }

    #[test]
    fn test02() {
        let mut puzzle = Puzzle::new(TEST_INPUT);
        puzzle.parse();
        assert_eq!(puzzle.part2(), 467835);
    }
}
