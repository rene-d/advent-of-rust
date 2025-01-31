//! [Day 3: Toboggan Trajectory](https://adventofcode.com/2020/day/3)

type Grid = aoc::GridU<u8>;

struct Puzzle {
    grid: Grid,
}

impl Puzzle {
    fn new(data: &str) -> Self {
        Self {
            grid: Grid::parse(data),
        }
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

/// # Panics
#[must_use]
pub fn solve(data: &str) -> (u64, u64) {
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
    fn test01() {
        let puzzle = Puzzle::new(TEST_INPUT);
        assert_eq!(puzzle.part1(), 7);
    }

    #[test]
    fn test02() {
        let puzzle = Puzzle::new(TEST_INPUT);
        assert_eq!(puzzle.part2(), 336);
    }
}
