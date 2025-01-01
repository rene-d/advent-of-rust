//! [Day 6: Guard Gallivant](https://adventofcode.com/2024/day/6)

use aoc::Coord;
use rustc_hash::FxHashSet;

type Grid = aoc::Grid<u8>;

struct Puzzle {
    grid: Grid,
    start: Coord,
}

impl Puzzle {
    fn new() -> Self {
        Self {
            grid: Grid::default(),
            start: Coord::ZERO,
        }
    }

    /// Get the puzzle input.
    fn configure(&mut self, data: &str) {
        self.grid = Grid::parse(data);

        for (xy, p) in &self.grid {
            if p == &b'^' {
                self.start = xy;
                break;
            }
        }
    }

    fn move_guard(
        &self,
        x: &mut i32,
        y: &mut i32,
        direction: &mut Coord,
        obstruction: (i32, i32),
    ) -> bool {
        match *direction {
            Coord::EAST => {
                if *x == 0 {
                    return true;
                } else if self.grid[(*x - 1, *y)] == b'#' || (*x - 1, *y) == obstruction {
                    *direction = Coord::NORTH;
                } else {
                    *x -= 1;
                }
            }
            Coord::WEST => {
                if *x == self.grid.width() - 1 {
                    return true;
                } else if self.grid[(*x + 1, *y)] == b'#' || (*x + 1, *y) == obstruction {
                    *direction = Coord::SOUTH;
                } else {
                    *x += 1;
                }
            }
            Coord::NORTH => {
                if *y == 0 {
                    return true;
                } else if self.grid[(*x, *y - 1)] == b'#' || (*x, *y - 1) == obstruction {
                    *direction = Coord::WEST;
                } else {
                    *y -= 1;
                }
            }
            Coord::SOUTH => {
                if *y == self.grid.height() - 1 {
                    return true;
                } else if self.grid[(*x, *y + 1)] == b'#' || (*x, *y + 1) == obstruction {
                    *direction = Coord::EAST;
                } else {
                    *y += 1;
                }
            }
            _ => unreachable!(),
        };

        false
    }

    /// Solve part one.
    fn part1(&self) -> usize {
        let Coord { mut x, mut y } = self.start;
        let mut direction = Coord::NORTH;
        let mut leave = false;

        let mut seen = FxHashSet::default();

        let obstruction = (i32::MAX, i32::MAX);

        while !leave {
            seen.insert((x, y));

            leave = self.move_guard(&mut x, &mut y, &mut direction, obstruction);
        }

        seen.len()
    }

    /// Solve part two.
    fn part2(&self) -> u32 {
        // repeat part 1 to eliminate positions that are never visited
        let mut xy = self.start;
        let mut direction = Coord::NORTH;
        let mut leave = false;
        let obstruction = (i32::MAX, i32::MAX);

        let mut normal_walk = FxHashSet::default();

        while !leave {
            normal_walk.insert(xy);
            leave = self.move_guard(&mut xy.x, &mut xy.y, &mut direction, obstruction);
        }

        let mut stuck = 0;

        for (xy, c) in &self.grid {
            // optimization: if the guard never walks to this position,
            // an obstruction cannot deviate him
            if !normal_walk.contains(&xy) {
                continue;
            }

            if c == &b'.' {
                // can choose this position for the obstruction

                let obstruction = (xy.x, xy.y);

                let mut xy = self.start;
                let mut direction = Coord::NORTH;
                let mut leave = false;
                let mut seen: FxHashSet<(Coord, Coord)> = FxHashSet::default();

                while !leave {
                    if seen.contains(&(xy, direction)) {
                        // same pos, same direction : the guard is stuck
                        stuck += 1;
                        break;
                    }
                    seen.insert((xy, direction));

                    leave = self.move_guard(&mut xy.x, &mut xy.y, &mut direction, obstruction);
                }
            }
        }

        stuck
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
    fn test01() {
        let mut puzzle = Puzzle::new();
        let data = aoc::load_input_data("test.txt");
        puzzle.configure(&data);
        assert_eq!(puzzle.part1(), 41);
    }

    #[test]
    fn test02() {
        let mut puzzle = Puzzle::new();
        let data = aoc::load_input_data("test.txt");
        puzzle.configure(&data);
        assert_eq!(puzzle.part2(), 6);
    }
}
