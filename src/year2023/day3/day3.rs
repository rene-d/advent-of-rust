//! [Day 3: Gear Ratios](https://adventofcode.com/2023/day/3)

use rustc_hash::FxHashMap;

struct Puzzle {
    grid: aoc::Grid<char>,
    sum_parts: u64,
    gears: FxHashMap<[i32; 2], Vec<u64>>,
}

impl Puzzle {
    fn new(data: &str) -> Self {
        Self {
            grid: aoc::Grid::<char>::parse(data, '.'),
            sum_parts: 0,
            gears: FxHashMap::default(),
        }
    }

    /// Read the schematic to find part numbers and gears
    fn parse(&mut self) {
        for y in 0..self.grid.height() {
            let mut x = 0;

            while x < self.grid.width() {
                let mut symbol = false;
                let mut gear = [0, 0];

                let mut n = 0;
                while let Some(d) = self.grid[(x, y)].to_digit(10) {
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
                        let c = self.grid[(x + ix, y + iy)];
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
