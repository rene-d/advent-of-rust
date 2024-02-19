//! [Day 3: Spiral Memory](https://adventofcode.com/2017/day/3)

struct Puzzle {
    n: i32,
}

impl Puzzle {
    fn new() -> Puzzle {
        Puzzle { n: 0 }
    }

    /// Get the puzzle input.
    fn configure(&mut self, path: &str) {
        let data = std::fs::read_to_string(path).unwrap();

        self.n = data.trim().parse().unwrap();
    }

    /// Solve part one.
    fn part1(&self) -> i32 {
        let mut x = 0;
        let mut y = 0;
        let mut m = 1;
        let mut dx = 1;
        let mut dy = 0;

        for i in 1..self.n {
            if i == (m * 2 + 1) * (m * 2 + 1) {
                x += 1;
                m += 1;
            } else {
                x += dx;
                y += dy;

                if y + dy > m {
                    dx = 1;
                    dy = 0;
                } else if x + dx < -m {
                    dx = 0;
                    dy = 1;
                } else if y + dy < -m {
                    dx = -1;
                    dy = 0;
                } else if x + dx > m {
                    dx = 0;
                    dy = -1;
                }
            }
        }

        x.abs() + y.abs()
    }

    /// Solve part two.
    fn part2(&self) -> i32 {
        let mut grid = [[0; 11]; 11];

        let offset = |x: i32| usize::try_from(x + 5).unwrap();

        let mut x = 0;
        let mut y = 0;
        let mut m = 1;
        let mut dx = 1;
        let mut dy = 0;

        grid[offset(y)][offset(x)] = 1;

        for i in 1.. {
            let value = if x == 0 && y == 0 {
                // initial value
                1
            } else {
                // sum of the values in all adjacent squares
                [
                    (-1, -1),
                    (-1, 0),
                    (-1, 1),
                    (0, -1),
                    (0, 1),
                    (1, -1),
                    (1, 0),
                    (1, 1),
                ]
                .iter()
                .map(|(ix, iy)| grid[offset(y + iy)][offset(x + ix)])
                .sum()
            };

            if value >= self.n {
                return value;
            }

            grid[offset(y)][offset(x)] = value;

            if i == (m * 2 + 1) * (m * 2 + 1) {
                x += 1;
                m += 1;
            } else {
                x += dx;
                y += dy;

                if y + dy > m {
                    dx = 1;
                    dy = 0;
                } else if x + dx < -m {
                    dx = 0;
                    dy = 1;
                } else if y + dy < -m {
                    dx = -1;
                    dy = 0;
                } else if x + dx > m {
                    dx = 0;
                    dy = -1;
                }
            }
        }

        unreachable!();
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

        puzzle.n = 1;
        assert_eq!(puzzle.part1(), 0);

        puzzle.n = 12;
        assert_eq!(puzzle.part1(), 3);

        puzzle.n = 23;
        assert_eq!(puzzle.part1(), 2);

        puzzle.n = 1024;
        assert_eq!(puzzle.part1(), 31);
    }
}
