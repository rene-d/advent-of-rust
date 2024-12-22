//! [Day 10: Hoof It](https://adventofcode.com/2024/day/10)

use aoc::{grid, grid::Grid};
use std::collections::{HashSet, VecDeque};

const BOTTOM: u8 = b'0';
const TOP: u8 = b'9';

struct Puzzle {
    grid: Grid<u8>,
}

impl Puzzle {
    fn new() -> Self {
        Self { grid: grid![] }
    }

    /// Get the puzzle input.
    fn configure(&mut self, path: &str) {
        let data = std::fs::read_to_string(path).unwrap();

        self.grid = aoc::grid::Grid::<u8>::parse(&data);
    }

    fn bfs(&self, start: (usize, usize)) -> usize {
        let mut visited = HashSet::new();
        let mut height_9 = HashSet::new();
        let mut queue = VecDeque::new();

        queue.push_back((start, BOTTOM));

        while let Some((xy, height)) = queue.pop_front() {
            visited.insert(xy);

            if self.grid[xy] == TOP {
                height_9.insert(xy);
            }

            for neigh in self.grid.iter_directions(xy) {
                if self.grid[neigh] == height + 1 && !visited.contains(&neigh) {
                    queue.push_back((neigh, height + 1));
                }
            }
        }

        height_9.len()
    }

    fn dfs(&self, xy: (usize, usize), height: u8) -> usize {
        if self.grid[xy] == TOP {
            1
        } else {
            self.grid
                .iter_directions(xy)
                .filter(|neigh| self.grid[*neigh] == height + 1)
                .map(|neigh| self.dfs(neigh, height + 1))
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
        puzzle.configure("sample_1.txt");
        assert_eq!(puzzle.part1(), 1);
    }

    #[test]
    fn test02() {
        let mut puzzle = Puzzle::new();
        puzzle.configure("sample_2.txt");
        assert_eq!(puzzle.part1(), 2);
    }

    #[test]
    fn test03() {
        let mut puzzle = Puzzle::new();
        puzzle.configure("sample_3.txt");
        assert_eq!(puzzle.part1(), 4);
    }

    #[test]
    fn test04() {
        let mut puzzle = Puzzle::new();
        puzzle.configure("sample_4.txt");
        assert_eq!(puzzle.part1(), 1 + 2);
    }

    #[test]
    fn test05() {
        let mut puzzle = Puzzle::new();
        puzzle.configure("sample_5.txt");
        assert_eq!(puzzle.part1(), 36);
    }

    #[test]
    fn test06() {
        let mut puzzle = Puzzle::new();
        puzzle.configure("sample_6.txt");
        assert_eq!(puzzle.part2(), 3);
    }

    #[test]
    fn test08() {
        let mut puzzle = Puzzle::new();
        puzzle.configure("sample_8.txt");
        assert_eq!(puzzle.part2(), 13);
    }

    #[test]
    fn test09() {
        let mut puzzle = Puzzle::new();
        puzzle.configure("sample_9.txt");
        assert_eq!(puzzle.part2(), 227);
    }

    #[test]
    fn test10() {
        let mut puzzle = Puzzle::new();
        puzzle.configure("sample_10.txt");
        assert_eq!(puzzle.part2(), 81);
    }
}
