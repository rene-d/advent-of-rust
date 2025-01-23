//! [Day 12: Digital Plumber](https://adventofcode.com/2017/day/12)

use rustc_hash::{FxHashMap, FxHashSet};
use std::collections::VecDeque;

struct Puzzle {
    links: FxHashMap<u32, Vec<u32>>,
}

impl Puzzle {
    /// Initialize from the puzzle input.
    fn new(data: &str) -> Self {
        let mut links = FxHashMap::default();

        for line in data.lines() {
            let (src, dests) = line.split_once(" <-> ").unwrap();

            links.insert(
                src.parse().unwrap(),
                dests.split(", ").map_while(|s| s.parse().ok()).collect(),
            );
        }
        Self { links }
    }

    fn walk(&self, id: u32) -> FxHashSet<u32> {
        let mut seen = FxHashSet::default();
        let mut queue = VecDeque::new();

        queue.push_front(id);
        while let Some(id) = queue.pop_back() {
            seen.insert(id);
            for dest in &self.links[&id] {
                if !seen.contains(dest) {
                    queue.push_front(*dest);
                }
            }
        }

        seen
    }

    /// Solve part one.
    fn part1(&self) -> usize {
        self.walk(0).len()
    }

    /// Solve part two.
    fn part2(&self) -> u64 {
        let mut programs: FxHashSet<u32> = self.links.keys().copied().collect();
        let mut groups = 0;
        while let Some(id) = programs.iter().next() {
            groups += 1;
            let connected = self.walk(*id);
            programs = programs.difference(&connected).copied().collect();
        }
        groups
    }
}

/// # Panics
/// over malformed input
#[must_use]
pub fn solve(data: &str) -> (usize, u64) {
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
        assert_eq!(puzzle.part1(), 6);
    }

    #[test]
    fn part2() {
        let puzzle = Puzzle::new(TEST_INPUT);
        assert_eq!(puzzle.part2(), 2);
    }
}
