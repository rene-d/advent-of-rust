//! [Day 10: Hoof It](https://adventofcode.com/2024/day/10)

use aoc::{Coord, Grid};
use rustc_hash::FxHashSet;
use std::collections::VecDeque;

const BOTTOM: u8 = b'0';
const TOP: u8 = b'9';

struct Puzzle {
    grid: Grid<u8>,
}

impl Puzzle {
    fn new(data: &str) -> Self {
        Self {
            grid: Grid::<u8>::parse(data),
        }
    }

    fn bfs(&self, start: Coord) -> usize {
        let mut visited = FxHashSet::default();
        let mut height_9 = FxHashSet::default();
        let mut queue = VecDeque::new();

        queue.push_back((start, BOTTOM));

        while let Some((xy, height)) = queue.pop_front() {
            visited.insert(xy);

            if self.grid[xy] == TOP {
                height_9.insert(xy);
            }

            for (_, neigh) in self.grid.iter_directions(xy) {
                if self.grid[neigh] == height + 1 && !visited.contains(&neigh) {
                    queue.push_back((neigh, height + 1));
                }
            }
        }

        height_9.len()
    }

    fn dfs(&self, xy: Coord, height: u8) -> usize {
        if self.grid[xy] == TOP {
            1
        } else {
            self.grid
                .iter_directions(xy)
                .filter(|(_, neigh)| self.grid[*neigh] == height + 1)
                .map(|(_, neigh)| self.dfs(neigh, height + 1))
                .sum()
        }
    }

    /// Solve part one.
    fn part1(&self) -> usize {
        self.grid
            .iter()
            .filter(|c| c.1 == &BOTTOM)
            .map(|(xy, _)| self.bfs(xy))
            .sum()
    }

    /// Solve part two.
    fn part2(&self) -> usize {
        self.grid
            .iter()
            .filter(|(_, c)| *c == &BOTTOM)
            .map(|(xy, _)| self.dfs(xy, BOTTOM))
            .sum()
    }
}

fn solve(data: &str) -> (usize, usize) {
    let puzzle = Puzzle::new(data);
    (puzzle.part1(), puzzle.part2())
}

fn main() {
    let mut args = aoc::parse_args();
    args.run(solve);
}

/// Test from puzzle input
#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE_1: &str = include_str!("sample_1.txt");
    const SAMPLE_2: &str = include_str!("sample_2.txt");
    const SAMPLE_3: &str = include_str!("sample_3.txt");
    const SAMPLE_4: &str = include_str!("sample_4.txt");
    const SAMPLE_5: &str = include_str!("sample_5.txt");
    const SAMPLE_6: &str = include_str!("sample_6.txt");
    const SAMPLE_8: &str = include_str!("sample_8.txt");
    const SAMPLE_9: &str = include_str!("sample_9.txt");
    const SAMPLE_10: &str = include_str!("sample_10.txt");

    #[test]
    fn test01() {
        let puzzle = Puzzle::new(SAMPLE_1);
        assert_eq!(puzzle.part1(), 1);
    }

    #[test]
    fn test02() {
        let puzzle = Puzzle::new(SAMPLE_2);
        assert_eq!(puzzle.part1(), 2);
    }

    #[test]
    fn test03() {
        let puzzle = Puzzle::new(SAMPLE_3);
        assert_eq!(puzzle.part1(), 4);
    }

    #[test]
    fn test04() {
        let puzzle = Puzzle::new(SAMPLE_4);
        assert_eq!(puzzle.part1(), 1 + 2);
    }

    #[test]
    fn test05() {
        let puzzle = Puzzle::new(SAMPLE_5);
        assert_eq!(puzzle.part1(), 36);
    }

    #[test]
    fn test06() {
        let puzzle = Puzzle::new(SAMPLE_6);
        assert_eq!(puzzle.part2(), 3);
    }

    #[test]
    fn test08() {
        let puzzle = Puzzle::new(SAMPLE_8);
        assert_eq!(puzzle.part2(), 13);
    }

    #[test]
    fn test09() {
        let puzzle = Puzzle::new(SAMPLE_9);
        assert_eq!(puzzle.part2(), 227);
    }

    #[test]
    fn test10() {
        let puzzle = Puzzle::new(SAMPLE_10);
        assert_eq!(puzzle.part2(), 81);
    }
}
