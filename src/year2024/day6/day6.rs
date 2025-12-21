//! [Day 6: Guard Gallivant](https://adventofcode.com/2024/day/6)

use aoc::Coord;
use rayon::prelude::*;
use rustc_hash::FxHashSet;

type Grid = aoc::Grid<u8>;

struct Puzzle {
    grid: Grid,
    start: Coord,
}

impl Puzzle {
    fn new(data: &str) -> Self {
        let grid = Grid::parse(data);
        let start = grid.iter().find(|(_, c)| c == &&b'^').unwrap().0;

        Self { grid, start }
    }

    /// Solve part one.
    fn part1(&self) -> usize {
        let width = self.grid.width();
        let height = self.grid.height();
        let Coord { mut x, mut y } = self.start;
        let mut dir = 0;

        let mut seen = FxHashSet::default();

        loop {
            seen.insert((x, y));

            let (next_x, next_y) = match dir {
                0 => (x, y - 1), // North
                1 => (x + 1, y), // East
                2 => (x, y + 1), // South
                3 => (x - 1, y), // West
                _ => unreachable!(),
            };

            if next_x < 0 || next_x >= width || next_y < 0 || next_y >= height {
                break;
            }

            if self.grid[(next_x, next_y)] == b'#' {
                dir = (dir + 1) % 4;
            } else {
                x = next_x;
                y = next_y;
            }
        }

        seen.len()
    }

    /// Solve part two.
    fn part2(&self) -> u32 {
        let width = self.grid.width();
        let height = self.grid.height();
        let Coord {
            x: start_x,
            y: start_y,
        } = self.start;

        // Trace original path to find candidates
        let mut candidates = Vec::new();

        let mut x = start_x;
        let mut y = start_y;
        let mut dir = 0;
        let mut seen = FxHashSet::default();

        #[allow(clippy::cast_sign_loss)]
        let visited_idx =
            |x: i32, y: i32, dir: i32| -> usize { ((y * width + x) * 4 + dir) as usize };
        let visited_size = visited_idx(0, height, 0);

        loop {
            if seen.insert((x, y)) {
                // Add as candidate if it's not the start position and valid
                if (x, y) != (start_x, start_y) {
                    candidates.push((x, y));
                }
            }

            let (next_x, next_y) = match dir {
                0 => (x, y - 1), // North
                1 => (x + 1, y), // East
                2 => (x, y + 1), // South
                3 => (x - 1, y), // West
                _ => unreachable!(),
            };

            if next_x < 0 || next_x >= width || next_y < 0 || next_y >= height {
                break;
            }

            if self.grid[(next_x, next_y)] == b'#' {
                dir = (dir + 1) % 4;
            } else {
                x = next_x;
                y = next_y;
            }
        }

        // Parallel optimization:
        // Use map_init to create a thread-local visited buffer and run_id.
        // This avoids reallocating the buffer for every candidate.
        candidates
            .par_iter()
            .map_init(
                || (vec![0u16; visited_size], 0u16),
                |(visited, run_id), &(ox, oy)| {
                    *run_id = run_id.wrapping_add(1);
                    if *run_id == 0 {
                        visited.fill(0);
                        *run_id = 1;
                    }
                    let current_run_id = *run_id;

                    let mut x = start_x;
                    let mut y = start_y;
                    let mut dir = 0; // North

                    loop {
                        let idx = visited_idx(x, y, dir);
                        if visited[idx] == current_run_id {
                            return 1; // Stuck
                        }
                        visited[idx] = current_run_id;

                        let (next_x, next_y) = match dir {
                            0 => (x, y - 1), // North
                            1 => (x + 1, y), // East
                            2 => (x, y + 1), // South
                            3 => (x - 1, y), // West
                            _ => unreachable!(),
                        };

                        if next_x < 0 || next_x >= width || next_y < 0 || next_y >= height {
                            return 0; // Outside of map
                        }

                        if self.grid[(next_x, next_y)] == b'#' || (next_x == ox && next_y == oy) {
                            dir = (dir + 1) % 4;
                        } else {
                            x = next_x;
                            y = next_y;
                        }
                    }
                },
            )
            .sum()
    }
}

/// # Panics
#[must_use]
pub fn solve(data: &str) -> (usize, u32) {
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
        assert_eq!(puzzle.part1(), 41);
    }

    #[test]
    fn test02() {
        let puzzle = Puzzle::new(TEST_INPUT);
        assert_eq!(puzzle.part2(), 6);
    }
}
