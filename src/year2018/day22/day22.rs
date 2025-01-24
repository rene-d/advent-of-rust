//! [Day 22: Mode Maze](https://adventofcode.com/2018/day/22)

use aoc::Direction;
use rustc_hash::{FxHashMap, FxHashSet};
use std::collections::BinaryHeap;

const ROCKY: u32 = 0;
const WET: u32 = 1;
const NARROW: u32 = 2;

const TORCH: u8 = 0;
const CLIMBING_GEAR: u8 = 1;
const NEITHER: u8 = 2;

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct Node {
    x: u32,
    y: u32,
    item: u8,
    time: u32,
    heuristic: u32,
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.heuristic.cmp(&self.heuristic)
    }
}

impl Node {
    /// Construct the initial node.
    /// Start at position (0,0) with a torch
    const fn init(target: (u32, u32)) -> Self {
        Self {
            x: 0,
            y: 0,
            item: TORCH,
            time: 0,
            heuristic: manhattan(0, 0, target),
        }
    }

    /// Switch item, with a cost of 7 seconds.
    const fn switch_item(&self, target: (u32, u32), region: u32) -> Self {
        let new_item = match (region, self.item) {
            (ROCKY, TORCH) | (WET, NEITHER) => CLIMBING_GEAR,
            (WET, CLIMBING_GEAR) | (NARROW, TORCH) => NEITHER,
            (ROCKY, CLIMBING_GEAR) | (NARROW, NEITHER) => TORCH,
            _ => panic!(),
        };

        let new_time = self.time + 7;

        Self {
            x: self.x,
            y: self.y,
            item: new_item,
            time: new_time,
            heuristic: manhattan(self.x, self.y, target) + new_time,
        }
    }

    /// Move to a position with a cost of 1 second.
    /// No verification that the movement is legal or not
    const fn move_to(&self, target: (u32, u32), x: u32, y: u32) -> Self {
        Self {
            x,
            y,
            item: self.item,
            time: self.time + 1,
            heuristic: manhattan(x, y, target) + self.time + 1,
        }
    }
}

/// Compute Manhattan distance to target.
const fn manhattan(x: u32, y: u32, target: (u32, u32)) -> u32 {
    x.abs_diff(target.0) + y.abs_diff(target.1)
}

/// Returns an iterator over the all four directions, within the limits of the grid.
fn iter_directions(
    x: u32,
    y: u32,
    width: u32,
    height: u32,
) -> impl Iterator<Item = (u32, u32, Direction)> {
    [
        Direction::North,
        Direction::East,
        Direction::South,
        Direction::West,
    ]
    .iter()
    .filter_map(move |&d| {
        if d == Direction::North && y > 0 {
            Some((x, y - 1, d))
        } else if d == Direction::South && y < height - 1 {
            Some((x, y + 1, d))
        } else if d == Direction::East && x < width - 1 {
            Some((x + 1, y, d))
        } else if d == Direction::West && x > 0 {
            Some((x - 1, y, d))
        } else {
            None
        }
    })
}

struct Puzzle {
    depth: u32,
    target: (u32, u32),

    lru: FxHashMap<(u32, u32), u32>,
}

impl Puzzle {
    fn new(data: &str) -> Self {
        let mut depth = 0;
        let mut target = (0, 0);

        for line in data.lines() {
            if let Some(s) = line.strip_prefix("target: ") {
                let (x, y) = s.split_once(',').unwrap();
                target = (x.parse().unwrap(), y.parse().unwrap());
            } else if let Some(x) = line.strip_prefix("depth: ") {
                depth = x.parse().unwrap();
            }
        }

        Self {
            depth,
            target,
            lru: FxHashMap::default(),
        }
    }

    fn geologic_index(&mut self, x: u32, y: u32) -> u32 {
        if let Some(index) = self.lru.get(&(x, y)) {
            *index
        } else {
            let index = if (x, y) == (0, 0) || (x, y) == self.target {
                0
            } else if y == 0 {
                x * 16807
            } else if x == 0 {
                y * 48271
            } else {
                self.erosion_level(x - 1, y) * self.erosion_level(x, y - 1)
            };

            self.lru.insert((x, y), index);
            index
        }
    }

    fn erosion_level(&mut self, x: u32, y: u32) -> u32 {
        (self.geologic_index(x, y) + self.depth) % 20183
    }

    fn region(&mut self, x: u32, y: u32) -> u32 {
        self.erosion_level(x, y) % 3
    }

    fn show(&mut self) {
        for y in 0..16 {
            for x in 0..16 {
                let c = if (x, y) == (0, 0) {
                    'M'
                } else if (x, y) == self.target {
                    'T'
                } else {
                    match self.region(x, y) {
                        ROCKY => '.',
                        WET => '=',
                        NARROW => '|',
                        _ => panic!(),
                    }
                };

                print!("{c}");
            }

            println!();
        }
    }
}

impl Puzzle {
    /// Solve part one.
    fn part1(&mut self) -> u32 {
        (0..=self.target.0)
            .map(|x| (0..=self.target.1).map(|y| self.region(x, y)).sum::<u32>())
            .sum()
    }

    /// Solve part two.
    fn part2(&mut self) -> u32 {
        // A star algorithm
        let mut open_set = BinaryHeap::new();
        let mut closed_list = FxHashSet::default();

        open_set.push(Node::init(self.target));

        while let Some(e) = open_set.pop() {
            // if current node is at goal (target and torch)
            // we have found our path
            if (e.x, e.y) == self.target && e.item == TORCH {
                return e.time;
            }

            if closed_list.contains(&(e.x, e.y, e.item)) {
                continue;
            }
            closed_list.insert((e.x, e.y, e.item));

            // switch item
            let n = e.switch_item(self.target, self.region(e.x, e.y));
            open_set.push(n);

            // move
            for (nx, ny, _) in iter_directions(e.x, e.y, u32::MAX, u32::MAX) {
                let can_move = match self.region(nx, ny) {
                    ROCKY => e.item != NEITHER,
                    WET => e.item != TORCH,
                    NARROW => e.item != CLIMBING_GEAR,
                    _ => panic!(),
                };

                if can_move && !closed_list.contains(&(nx, ny, e.item)) {
                    open_set.push(e.move_to(self.target, nx, ny));
                }
            }
        }
        0
    }
}

/// # Panics
#[must_use]
pub fn solve(data: &str) -> (u32, u32) {
    let mut puzzle = Puzzle::new(data);
    (puzzle.part1(), puzzle.part2())
}

pub fn main() {
    let args = aoc::parse_args();

    if args.verbose {
        Puzzle::new(&args.input).show();
        std::process::exit(0);
    }

    args.run(solve);
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = include_str!("test.txt");

    #[test]
    fn test01() {
        let mut puzzle = Puzzle::new(TEST_INPUT);
        assert_eq!(puzzle.part1(), 114);
    }

    #[test]
    fn test02() {
        let mut puzzle = Puzzle::new(TEST_INPUT);
        assert_eq!(puzzle.part2(), 45);
    }
}
