//! [Day 18: Many-Worlds Interpretation](https://adventofcode.com/2019/day/18)

use std::collections::{HashSet, VecDeque};

use aoc::{grid, grid::Grid};

use day18::mazette::Dijkstra;
use day18::multirobot::Multirobot;

struct Puzzle {
    maze: Grid<u8>,
}

impl Puzzle {
    fn new() -> Self {
        Self { maze: grid![] }
    }

    /// Get the puzzle input.
    fn configure(&mut self, path: &str) {
        let data = std::fs::read_to_string(path).unwrap();

        self.maze = aoc::grid::Grid::<u8>::parse(&data);
    }

    /// Solve part one.
    fn part1(&self) -> u32 {
        let mut entrance = (0, 0);
        let mut all_keys = 0u32;

        for (xy, c) in self.maze.iter() {
            match c {
                b'@' => entrance = xy,
                b'a'..=b'z' => all_keys |= 1 << u32::from(*c - b'a'),
                _ => (),
            }
        }

        // bfs

        let mut q = VecDeque::new();
        let mut seen = HashSet::new();

        q.push_back((entrance, 0, 0));
        while let Some((pos, mut keys, steps)) = q.pop_front() {
            if seen.contains(&(pos, keys)) {
                continue;
            }
            seen.insert((pos, keys));

            let c = self.maze[pos];

            match c {
                b'A'..=b'Z' => {
                    // it's a door

                    // have we the key ?
                    if keys & (1 << u32::from(c - b'A')) == 0 {
                        // no: we can't pass
                        continue;
                    }
                }

                b'a'..=b'z' => {
                    // it's a key

                    // collect it
                    keys |= 1 << u32::from(c - b'a');

                    // if we have all of them, it's over
                    if keys == all_keys {
                        return steps;
                    }
                }

                _ => (),
            }

            for np in self.maze.iter_directions(pos) {
                if self.maze[np] != b'#' {
                    q.push_back((np, keys, steps + 1));
                }
            }
        }

        0
    }

    /// Solve part two.
    fn part2(&mut self) -> usize {
        // Part 2 is much more complicated. All logics are implemented separately
        self.maze.update();

        self.maze.search()
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
        assert_eq!(puzzle.part1(), 8);
    }

    #[test]
    fn test02() {
        let mut puzzle = Puzzle::new();
        puzzle.configure("sample_4.txt");
        assert_eq!(puzzle.part1(), 86);
    }

    #[test]
    fn test03() {
        let mut puzzle = Puzzle::new();
        puzzle.configure("sample_9.txt");
        assert_eq!(puzzle.part1(), 132);
    }

    #[test]
    fn test04() {
        let mut puzzle = Puzzle::new();
        puzzle.configure("sample_10.txt");
        assert_eq!(puzzle.part1(), 136);
    }

    #[test]
    fn test05() {
        let mut puzzle = Puzzle::new();
        puzzle.configure("sample_11.txt");
        assert_eq!(puzzle.part1(), 81);
    }
}
