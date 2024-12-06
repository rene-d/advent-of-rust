//! [Day 6: Guard Gallivant](https://adventofcode.com/2024/day/6)

use aoc::{grid, grid::Direction, grid::Grid};
use std::collections::HashSet;

struct Puzzle {
    grid: Grid<char>,
    start: (usize, usize),
}

impl Puzzle {
    fn new() -> Puzzle {
        Puzzle {
            grid: grid![],
            start: (0, 0),
        }
    }

    /// Get the puzzle input.
    fn configure(&mut self, path: &str) {
        let data = std::fs::read_to_string(path).unwrap();

        self.grid = aoc::grid::Grid::<char>::parse(&data);

        for (x, y, p) in self.grid.iter() {
            if p == &'^' {
                self.start = (x, y);
                break;
            }
        }
    }

    fn move_guard(
        &self,
        x: &mut usize,
        y: &mut usize,
        direction: &mut grid::Direction,
        obstruction: (usize, usize),
    ) -> bool {
        match direction {
            Direction::East => {
                if *x == 0 {
                    return true;
                } else if self.grid[(*x - 1, *y)] == '#' || (*x - 1, *y) == obstruction {
                    *direction = Direction::North;
                } else {
                    *x -= 1;
                }
            }
            Direction::West => {
                if *x == self.grid.size().0 - 1 {
                    return true;
                } else if self.grid[(*x + 1, *y)] == '#' || (*x + 1, *y) == obstruction {
                    *direction = Direction::South;
                } else {
                    *x += 1;
                }
            }
            Direction::North => {
                if *y == 0 {
                    return true;
                } else if self.grid[(*x, *y - 1)] == '#' || (*x, *y - 1) == obstruction {
                    *direction = Direction::West;
                } else {
                    *y -= 1;
                }
            }
            Direction::South => {
                if *y == self.grid.size().1 - 1 {
                    return true;
                } else if self.grid[(*x, *y + 1)] == '#' || (*x, *y + 1) == obstruction {
                    *direction = Direction::East;
                } else {
                    *y += 1;
                }
            }
        };

        false
    }

    /// Solve part one.
    fn part1(&self) -> usize {
        let (mut x, mut y) = self.start;
        let mut direction = grid::Direction::North;
        let mut leave = false;

        let mut seen = HashSet::new();

        let obstruction = (usize::MAX, usize::MAX);

        while !leave {
            seen.insert((x, y));

            leave = self.move_guard(&mut x, &mut y, &mut direction, obstruction);
        }

        seen.len()
    }

    /// Solve part two.
    fn part2(&self) -> u32 {
        // repeat part 1 to eliminate positions that are never visited
        let (mut x, mut y) = self.start;
        let mut direction = grid::Direction::North;
        let mut leave = false;
        let obstruction = (usize::MAX, usize::MAX);

        let mut normal_walk = HashSet::new();

        while !leave {
            normal_walk.insert((x, y));
            leave = self.move_guard(&mut x, &mut y, &mut direction, obstruction);
        }

        let mut stuck = 0;

        for (ox, oy, c) in self.grid.iter() {
            // optimization: if the guard never walks to this position,
            // an obstruction cannot deviate him
            if !normal_walk.contains(&(ox, oy)) {
                continue;
            }

            if c == &'.' {
                // can choose this position for the obstruction

                let obstruction = (ox, oy);

                let (mut x, mut y) = self.start;
                let mut direction = grid::Direction::North;
                let mut leave = false;
                let mut seen: HashSet<(usize, usize, Direction)> = HashSet::new();

                while !leave {
                    if seen.contains(&(x, y, direction)) {
                        // same pos, same direction : the guard is stuck
                        stuck += 1;
                        break;
                    }
                    seen.insert((x, y, direction));

                    leave = self.move_guard(&mut x, &mut y, &mut direction, obstruction);
                }
            }
        }

        stuck
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
        assert_eq!(puzzle.part1(), 41);
    }

    #[test]
    fn test02() {
        let mut puzzle = Puzzle::new();
        puzzle.configure("test.txt");
        assert_eq!(puzzle.part2(), 6);
    }
}
