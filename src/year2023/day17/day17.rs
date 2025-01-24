//! [Day 17: Clumsy Crucible](https://adventofcode.com/2023/day/17)

use rustc_hash::FxHashSet;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
enum Direction {
    Unknown,
    North,
    East,
    South,
    West,
}

impl Direction {
    /// Move one step in the given direction, if possible.
    fn step(self, x: usize, y: usize, sx: usize, sy: usize) -> Option<(usize, usize)> {
        if self == Self::North && y > 0 {
            Some((x, y - 1))
        } else if self == Self::East && x < sx - 1 {
            Some((x + 1, y))
        } else if self == Self::South && y < sy - 1 {
            Some((x, y + 1))
        } else if self == Self::West && x > 0 {
            Some((x - 1, y))
        } else {
            None
        }
    }

    /// Indicate if two directions are opposite.
    fn is_opposite(self, other: Self) -> bool {
        self == Self::North && other == Self::South
            || self == Self::South && other == Self::North
            || self == Self::East && other == Self::West
            || self == Self::West && other == Self::East
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct State {
    heat_loss: u32,
    x: usize,
    y: usize,
    direction: Direction,
    same_direction_count: u8,
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other.heat_loss.cmp(&self.heat_loss)
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct Puzzle {
    grid: Vec<Vec<u32>>,
    sx: usize,
    sy: usize,
}

impl Puzzle {
    fn new(data: &str) -> Self {
        let grid: Vec<Vec<u32>> = data
            .lines()
            .map(|line| line.chars().filter_map(|c| c.to_digit(10)).collect())
            .collect();

        let sx = grid[0].len();
        let sy = grid.len();

        Self { grid, sx, sy }
    }

    fn dijkstra(&self, ultra_crucibles: bool) -> u32 {
        let mut heap = BinaryHeap::new();
        let mut seen = FxHashSet::default();
        let mut min_heat_loss = u32::MAX;

        heap.push(State {
            heat_loss: 0,
            x: 0,
            y: 0,
            direction: Direction::Unknown,
            same_direction_count: u8::MAX, // initial value, anything greater than 4
        });

        while let Some(State {
            heat_loss,
            x,
            y,
            direction: course,
            same_direction_count,
        }) = heap.pop()
        {
            if x == self.sx - 1 && y == self.sy - 1 {
                // a minimum of four blocks in that direction before it can stop at the end
                if !ultra_crucibles || same_direction_count >= 4 {
                    min_heat_loss = min_heat_loss.min(heat_loss);
                }
                continue;
            }

            let key = (x, y, course, same_direction_count);
            if seen.contains(&key) {
                continue;
            }
            seen.insert(key);

            for new_direction in [
                Direction::North,
                Direction::East,
                Direction::South,
                Direction::West,
            ] {
                // do not go back
                if new_direction.is_opposite(course) {
                    continue;
                }

                if let Some((nx, ny)) = new_direction.step(x, y, self.sx, self.sy) {
                    // update the number of steps in the same direction
                    let new_direction_count = if new_direction == course {
                        same_direction_count + 1
                    } else {
                        1
                    };

                    if ultra_crucibles {
                        // 10 consecutive blocks without turning
                        if new_direction_count > 10 {
                            continue;
                        }

                        // a minimum of four blocks in that direction (or start position)
                        if new_direction != course && same_direction_count < 4 {
                            continue;
                        }
                    } else {
                        // at most three blocks in the same direction
                        if new_direction_count > 3 {
                            continue;
                        }
                    }

                    heap.push(State {
                        heat_loss: heat_loss + self.grid[ny][nx],
                        x: nx,
                        y: ny,
                        direction: new_direction,
                        same_direction_count: new_direction_count,
                    });
                }
            }
        }

        min_heat_loss
    }

    /// Solve part one.
    fn part1(&self) -> u32 {
        self.dijkstra(false)
    }

    /// Solve part two.
    fn part2(&self) -> u32 {
        self.dijkstra(true)
    }
}

/// # Panics
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

    const TEST_INPUT: &str = include_str!("test.txt");
    const TEST_INPUT_2: &str = include_str!("test2.txt");

    #[test]
    fn test01() {
        let puzzle = Puzzle::new(TEST_INPUT);
        assert_eq!(puzzle.part1(), 102);
    }

    #[test]
    fn test02() {
        let puzzle = Puzzle::new(TEST_INPUT);
        assert_eq!(puzzle.part2(), 94);
    }

    #[test]
    fn test03() {
        let puzzle = Puzzle::new(TEST_INPUT_2);
        assert_eq!(puzzle.part2(), 71);
    }
}
