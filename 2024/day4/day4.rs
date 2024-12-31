//! [Day 4: Ceres Search](https://adventofcode.com/2024/day/4)

use aoc::Coord;

type Grid = aoc::Grid<char>;

struct Puzzle {
    grid: Grid,
}

impl Puzzle {
    fn new() -> Self {
        Self {
            grid: Grid::default(),
        }
    }

    /// Get the puzzle input.
    fn configure(&mut self, data: &str) {
        self.grid = Grid::parse(data);
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

fn main() {
    let args = aoc::parse_args();
    let mut puzzle = Puzzle::new();
    puzzle.configure(&args.input);
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
        let data = aoc::load_input_data("sample_1.txt");
        puzzle.configure(&data);
        assert_eq!(puzzle.part1(), 4);
    }

    #[test]
    fn test_p1_2() {
        let mut puzzle = Puzzle::new();

        puzzle.configure(&aoc::load_input_data("sample_2.txt"));
        assert_eq!(puzzle.part1(), 18);

        puzzle.configure(&aoc::load_input_data("sample_3.txt"));
        assert_eq!(puzzle.part1(), 18);
    }

    #[test]
    fn test_p2_1() {
        let mut puzzle = Puzzle::new();
        puzzle.configure(&aoc::load_input_data("sample_4.txt"));
        assert_eq!(puzzle.part2(), 1);
    }

    #[test]
    fn test_p2_2() {
        let mut puzzle = Puzzle::new();

        puzzle.configure(&aoc::load_input_data("sample_2.txt"));
        assert_eq!(puzzle.part2(), 9);

        puzzle.configure(&aoc::load_input_data("sample_5.txt"));
        assert_eq!(puzzle.part2(), 9);
    }
}
