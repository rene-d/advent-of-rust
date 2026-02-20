//! [Day 16: Reindeer Maze](https://adventofcode.com/2024/day/16)

use rustc_hash::FxHashSet;
use std::collections::BinaryHeap;

const EAST: usize = 0;
const SOUTH: usize = 1;
const WEST: usize = 2;
const NORTH: usize = 3;

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
struct Coord {
    x: usize,
    y: usize,
}

impl Coord {
    const ZERO: Self = Self { x: 0, y: 0 };

    const fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    fn add(&self, dir: usize) -> Self {
        match dir {
            EAST => Self {
                x: self.x + 1,
                y: self.y,
            },
            SOUTH => Self {
                x: self.x,
                y: self.y + 1,
            },
            WEST => Self {
                x: self.x - 1,
                y: self.y,
            },
            NORTH => Self {
                x: self.x,
                y: self.y - 1,
            },
            _ => unreachable!(),
        }
    }
}

struct Cost1 {
    cost: u32,
    pos: Coord,
    dir: usize,
}

impl Cost1 {
    const fn new(cost: u32, pos: Coord, dir: usize) -> Self {
        Self { cost, pos, dir }
    }
}

impl Ord for Cost1 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Cost1 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Cost1 {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}

impl Eq for Cost1 {}

struct Puzzle {
    start: Coord,
    end: Coord,
    maze: FxHashSet<Coord>,
    width: usize,
    height: usize,
}

impl Puzzle {
    fn new(data: &str) -> Self {
        let mut start = Coord::ZERO;
        let mut end = Coord::ZERO;
        let mut maze = FxHashSet::default();
        let mut height = 0;
        let mut width = 0;

        for (line, y) in data.lines().zip(0..) {
            for (c, x) in line.chars().zip(0..) {
                if c == '#' {
                    continue;
                }

                if c == 'S' {
                    start = Coord::new(x, y);
                } else if c == 'E' {
                    end = Coord::new(x, y);
                }

                maze.insert(Coord { x, y });
                width = x + 1;
            }

            height = y + 1;
        }

        Self {
            start,
            end,
            maze,
            width,
            height,
        }
    }

    /// Solve part one.
    fn part1(&self) -> u32 {
        let num_states = self.width * self.height * 4;
        let mut dist = vec![u32::MAX; num_states];
        let mut heap = BinaryHeap::new();

        let start_idx = (self.start.y * self.width + self.start.x) * 4 + EAST;
        dist[start_idx] = 0;
        heap.push(Cost1::new(0, self.start, EAST));

        while let Some(Cost1 { cost, pos, dir }) = heap.pop() {
            let current_idx = (pos.y * self.width + pos.x) * 4 + dir;
            if cost > dist[current_idx] {
                continue;
            }

            if pos == self.end {
                return cost;
            }

            // Left turn (counter-clockwise): (idx + 3) % 4
            // Right turn (clockwise): (idx + 1) % 4
            // Straight: idx

            let moves = [
                (cost + 1, dir),              // Straight
                (cost + 1001, (dir + 3) % 4), // Left turn + move
                (cost + 1001, (dir + 1) % 4), // Right turn + move
            ];

            for (new_cost, new_dir) in moves {
                let new_pos = pos.add(new_dir);

                if self.maze.contains(&new_pos) {
                    let new_idx = (new_pos.y * self.width + new_pos.x) * 4 + new_dir;
                    if new_cost < dist[new_idx] {
                        dist[new_idx] = new_cost;
                        heap.push(Cost1::new(new_cost, new_pos, new_dir));
                    }
                }
            }
        }

        0
    }

    /// Solve part two.
    fn part2(&self) -> usize {
        let num_states = self.width * self.height * 4;

        let mut dist = vec![u32::MAX; num_states];
        let mut preds: Vec<Vec<_>> = vec![Vec::new(); num_states];
        let mut heap = BinaryHeap::new();

        let start_idx = (self.start.y * self.width + self.start.x) * 4 + EAST;
        dist[start_idx] = 0;
        heap.push(Cost1::new(0, self.start, EAST));

        let mut best_cost = u32::MAX;
        let mut end_states = Vec::new();

        while let Some(Cost1 { cost, pos, dir }) = heap.pop() {
            let current_idx = (pos.y * self.width + pos.x) * 4 + dir;

            if cost > dist[current_idx] {
                continue;
            }

            // Optimization: if we exceeded the best cost found so far, we can stop
            // But we need to be careful to find ALL paths.
            // Actually, Dijkstra guarantees we visit in order.
            // If we found an end state, that cost is the best cost (first time).
            // Any subsequent pop with cost > best_cost means we are done effectively for finding *optimal* paths.
            if cost > best_cost {
                break;
            }

            if pos == self.end {
                if cost < best_cost {
                    best_cost = cost;
                    end_states.clear();
                    end_states.push(current_idx);
                } else if cost == best_cost {
                    end_states.push(current_idx);
                }
                continue;
            }

            let moves = [
                (cost + 1, dir),              // Straight
                (cost + 1001, (dir + 3) % 4), // Left turn + move
                (cost + 1001, (dir + 1) % 4), // Right turn + move
            ];

            for (new_cost, new_dir) in moves {
                let new_pos = pos.add(new_dir);

                if self.maze.contains(&new_pos) {
                    let new_idx = (new_pos.y * self.width + new_pos.x) * 4 + new_dir;

                    if new_cost < dist[new_idx] {
                        dist[new_idx] = new_cost;
                        preds[new_idx].clear();
                        preds[new_idx].push(current_idx);
                        heap.push(Cost1::new(new_cost, new_pos, new_dir));
                    } else if new_cost == dist[new_idx] {
                        preds[new_idx].push(current_idx);
                    }
                }
            }
        }

        // Backtracking BFS
        let mut seen_idx = FxHashSet::default();
        let mut queue = std::collections::VecDeque::new();
        let mut unique_tiles = FxHashSet::default();

        for &idx in &end_states {
            if seen_idx.insert(idx) {
                queue.push_back(idx);
            }
        }

        while let Some(idx) = queue.pop_front() {
            // Recover coord from idx
            let y = (idx / 4) / self.width;
            let x = (idx / 4) % self.width;
            unique_tiles.insert(Coord { x, y });

            for &pred_idx in &preds[idx] {
                if seen_idx.insert(pred_idx) {
                    queue.push_back(pred_idx);
                }
            }
        }

        unique_tiles.len()
    }
}

/// # Panics
#[must_use]
pub fn solve(data: &str) -> (u32, usize) {
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

    const SAMPLE_1: &str = include_str!("sample_1.txt");
    const SAMPLE_3: &str = include_str!("sample_3.txt");

    #[test]
    fn test01() {
        let puzzle = Puzzle::new(SAMPLE_1);
        assert_eq!(puzzle.part1(), 7036);
    }

    #[test]
    fn test02() {
        let puzzle = Puzzle::new(SAMPLE_3);
        assert_eq!(puzzle.part1(), 11048);
    }

    #[test]
    fn test03() {
        let puzzle = Puzzle::new(SAMPLE_1);
        assert_eq!(puzzle.part2(), 45);
    }

    #[test]
    fn test04() {
        let puzzle = Puzzle::new(SAMPLE_3);
        assert_eq!(puzzle.part2(), 64);
    }
}
