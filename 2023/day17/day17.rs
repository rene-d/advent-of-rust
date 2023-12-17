//! [Day 17: Clumsy Crucible](https://adventofcode.com/2023/day/17)

use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};

use clap::Parser;

#[derive(Parser)]
struct Args {
    /// Puzzle input
    #[arg(default_value = "input.txt")]
    path: String,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
enum Direction {
    Unknown,
    North,
    East,
    South,
    West,
}

impl Direction {
    fn step(self, x: usize, y: usize, sx: usize, sy: usize) -> Option<(usize, usize)> {
        if self == Direction::North && y > 0 {
            Some((x, y - 1))
        } else if self == Direction::East && x < sx - 1 {
            Some((x + 1, y))
        } else if self == Direction::South && y < sy - 1 {
            Some((x, y + 1))
        } else if self == Direction::West && x > 0 {
            Some((x - 1, y))
        } else {
            None
        }
    }

    fn is_inverse(self, other: Direction) -> bool {
        self == Direction::North && other == Direction::South
            || self == Direction::South && other == Direction::North
            || self == Direction::East && other == Direction::West
            || self == Direction::West && other == Direction::East
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct State {
    cost: u32,
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
        other.cost.cmp(&self.cost)
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
    fn new() -> Puzzle {
        Puzzle {
            grid: vec![],
            sx: 0,
            sy: 0,
        }
    }

    /// Get the puzzle input.
    fn configure(&mut self, path: &str) {
        let data = std::fs::read_to_string(path).unwrap();

        for line in data.lines() {
            let row = line.chars().filter_map(|c| c.to_digit(10)).collect();

            self.grid.push(row);
        }

        self.sx = self.grid[0].len();
        self.sy = self.grid.len();
    }

    fn dijkstra(&self, ultra_crucibles: bool) -> u32 {
        let mut heap = BinaryHeap::new();
        let mut seen = HashSet::new();
        let mut min_cost = u32::MAX;

        heap.push(State {
            cost: 0,
            x: 0,
            y: 0,
            direction: Direction::Unknown,
            same_direction_count: 255,
        });

        while let Some(State {
            cost,
            x,
            y,
            direction: course,
            same_direction_count,
        }) = heap.pop()
        {
            if x == self.sx - 1 && y == self.sy - 1 {
                // a minimum of four blocks in that direction before it can stop at the end
                if !ultra_crucibles || same_direction_count >= 4 {
                    min_cost = min_cost.min(cost);
                }
                continue;
            }

            let key = (x, y, course, same_direction_count);
            if seen.contains(&key) {
                continue;
            }
            seen.insert(key);

            for new_course in [
                Direction::North,
                Direction::East,
                Direction::South,
                Direction::West,
            ] {
                // do not go back
                if new_course.is_inverse(course) {
                    continue;
                }

                if let Some((nx, ny)) = new_course.step(x, y, self.sx, self.sy) {
                    // update the number of steps in the same direction
                    let new_course_count = if new_course == course {
                        same_direction_count + 1
                    } else {
                        1
                    };

                    if ultra_crucibles {
                        // 10 consecutive blocks without turning
                        if new_course_count > 10 {
                            continue;
                        }

                        // a minimum of four blocks in that direction (or start)
                        if new_course != course
                            && same_direction_count < 4
                            && same_direction_count != 255
                        {
                            continue;
                        }
                    } else {
                        // at most three blocks in a single direction
                        if new_course_count > 3 {
                            continue;
                        }
                    }

                    heap.push(State {
                        cost: cost + self.grid[ny][nx],
                        x: nx,
                        y: ny,
                        direction: new_course,
                        same_direction_count: new_course_count,
                    });
                }
            }
        }

        min_cost
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

/// Test from puzzle input
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test01() {
        let mut puzzle = Puzzle::new();
        puzzle.configure("test.txt");
        assert_eq!(puzzle.part1(), 102);
    }

    #[test]
    fn test02() {
        let mut puzzle = Puzzle::new();
        puzzle.configure("test.txt");
        assert_eq!(puzzle.part2(), 94);
    }

    #[test]
    fn test03() {
        let mut puzzle = Puzzle::new();
        puzzle.configure("test2.txt");
        assert_eq!(puzzle.part2(), 71);
    }
}

fn main() {
    let args = Args::parse();
    let mut puzzle = Puzzle::new();
    puzzle.configure(args.path.as_str());
    println!("{}", puzzle.part1());
    println!("{}", puzzle.part2());
}
