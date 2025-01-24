//! [Day 19: A Series of Tubes](https://adventofcode.com/2017/day/19)

use aoc::Coord;

struct Puzzle {
    path: String,
    steps: u32,
}

impl Puzzle {
    /// Initialize from the puzzle input.
    fn new(data: &str) -> Self {
        let   grid = aoc::Grid::<char>::parse(data, ' ');


        // find start
        let start = (0..grid.width())
            .find(|&x| grid[(x, 0)] == '|')
            .map_or(0, |x| x);

        let mut path = String::new();
        let mut steps = 0;

        let mut xy = Coord::new(start, 0);
        let mut dir = Coord::SOUTH;

        loop {
            xy += dir;
            steps += 1;

            let c = grid[xy];
            match c {
                ' ' => break,   // outside
                '-' | '|' => {} // no change of direction
                'A'..='Z' => {
                    // letter
                    path.push(c);
                }
                '+' => {
                    // if we can turn clockwise on a line, continue
                    if "-|ABCDEFGHIJKLMNOPQRSTUVWXYZ".contains(grid[xy + dir.clockwise()]) {
                        dir = dir.clockwise();
                    } else {
                        // actually we should test if we have a line to continue on
                        // but I suppose that the puzzle input is correct
                        dir = dir.counter_clockwise();
                    }
                }
                _ => panic!(),
            }
        }

        Self { path, steps }
    }

    /// Solve part one.
    fn part1(&self) -> String {
        self.path.clone()
    }

    /// Solve part two.
    const fn part2(&self) -> u32 {
        self.steps
    }
}

/// # Panics
#[must_use]
pub fn solve(data: &str) -> (String, u32) {
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
    fn part1() {
        let puzzle = Puzzle::new(TEST_INPUT);
        assert_eq!(puzzle.part1(), "ABCDEF");
    }

    #[test]
    fn part2() {
        let puzzle = Puzzle::new(TEST_INPUT);
        assert_eq!(puzzle.part2(), 38);
    }
}
