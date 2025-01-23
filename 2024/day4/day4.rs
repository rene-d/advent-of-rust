//! [Day 4: Ceres Search](https://adventofcode.com/2024/day/4)

use aoc::Coord;

type Grid = aoc::Grid<char>;

struct Puzzle {
    grid: Grid,
}

impl Puzzle {
    fn new(data: &str) -> Self {
        Self {
            grid: Grid::parse(data),
        }
    }

    /// Solve part one.
    fn part1(&self) -> u32 {
        let mut n = 0;
        let (sx, sy) = (self.grid.width(), self.grid.height());

        let mas = |i| match i {
            1 => 'M',
            2 => 'A',
            3 => 'S',
            _ => unreachable!(),
        };

        for (Coord { x, y }, _) in self.grid.iter().filter(|(_, p)| *p == &'X') {
            //
            if x <= sx - 4 && (1..4).all(|i| self.grid[(x + i, y)] == mas(i)) {
                n += 1;
            }
            if y <= sy - 4 && (1..4).all(|i| self.grid[(x, y + i)] == mas(i)) {
                n += 1;
            }

            if x >= 3 && (1..4).all(|i| self.grid[(x - i, y)] == mas(i)) {
                n += 1;
            }
            if y >= 3 && (1..4).all(|i| self.grid[(x, y - i)] == mas(i)) {
                n += 1;
            }

            if x <= sx - 4 && y <= sy - 4 && (1..4).all(|i| self.grid[(x + i, y + i)] == mas(i)) {
                n += 1;
            }
            if x >= 3 && y >= 3 && (1..4).all(|i| self.grid[(x - i, y - i)] == mas(i)) {
                n += 1;
            }

            if x <= sx - 4 && y >= 3 && (1..4).all(|i| self.grid[(x + i, y - i)] == mas(i)) {
                n += 1;
            }
            if x >= 3 && y <= sy - 4 && (1..4).all(|i| self.grid[(x - i, y + i)] == mas(i)) {
                n += 1;
            }
        }
        n
    }

    /// Solve part two.
    fn part2(&self) -> u32 {
        let mut n = 0;
        let (sx, sy) = (self.grid.width(), self.grid.height());
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
    const SAMPLE_2: &str = include_str!("sample_2.txt");
    const SAMPLE_3: &str = include_str!("sample_3.txt");
    const SAMPLE_4: &str = include_str!("sample_4.txt");
    const SAMPLE_5: &str = include_str!("sample_5.txt");

    #[test]
    fn test_p1_1() {
        let puzzle = Puzzle::new(SAMPLE_1);
        assert_eq!(puzzle.part1(), 4);
    }

    #[test]
    fn test_p1_2() {
        let puzzle = Puzzle::new(SAMPLE_2);
        assert_eq!(puzzle.part1(), 18);

        let puzzle = Puzzle::new(SAMPLE_3);
        assert_eq!(puzzle.part1(), 18);
    }

    #[test]
    fn test_p2_1() {
        let puzzle = Puzzle::new(SAMPLE_4);
        assert_eq!(puzzle.part2(), 1);
    }

    #[test]
    fn test_p2_2() {
        let puzzle = Puzzle::new(SAMPLE_2);
        assert_eq!(puzzle.part2(), 9);

        let puzzle = Puzzle::new(SAMPLE_5);
        assert_eq!(puzzle.part2(), 9);
    }
}
