//! [Day 3: Toboggan Trajectory](https://adventofcode.com/2020/day/3)

use aoc::grid;

type Grid = aoc::grid::Grid<u8>;

struct Puzzle {
    grid: Grid,
}

impl Puzzle {
    fn new() -> Puzzle {
        Puzzle { grid: grid!() }
    }

    /// Get the puzzle input.
    fn configure(&mut self, path: &str) {
        let data = std::fs::read_to_string(path).unwrap();

        self.grid = Grid::parse(&data);
    }

    fn trees(&self, slope_x: usize, slope_y: usize) -> u64 {
        let mut x = 0;
        let mut y = 0;
        let mut n = 0;

        while y < self.grid.size().1 {
            if self.grid[(x, y)] == b'#' {
                n += 1;
            }

            x = (x + slope_x) % self.grid.size().0;
            y += slope_y;
        }

        n
    }

    /// Solve part one.
    fn part1(&self) -> u64 {
        self.trees(3, 1)
    }

    /// Solve part two.
    fn part2(&self) -> u64 {
        self.trees(1, 1) * self.trees(3, 1) * self.trees(5, 1) * self.trees(7, 1) * self.trees(1, 2)
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
        puzzle.configure("test.txt");
        assert_eq!(puzzle.part1(), 7);
    }

    #[test]
    fn test02() {
        let mut puzzle = Puzzle::new();
        puzzle.configure("test.txt");
        assert_eq!(puzzle.part2(), 336);
    }
}
