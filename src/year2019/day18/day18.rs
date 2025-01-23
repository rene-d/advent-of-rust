//! [Day 18: Many-Worlds Interpretation](https://adventofcode.com/2019/day/18)

mod mazecell;
mod mazette;
mod multirobot;
mod path;
mod state;

use mazette::Dijkstra;
use multirobot::Multirobot;
use rustc_hash::FxHashSet;
use std::collections::VecDeque;

struct Puzzle {
    maze: aoc::GridU<u8>,
}

impl Puzzle {
    fn new(data: &str) -> Self {
        Self {
            maze: aoc::GridU::<u8>::parse(data),
        }
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
        let mut seen = FxHashSet::default();

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

/// # Panics
/// over malformed input
#[must_use]
pub fn solve(data: &str) -> (u32, usize) {
    let mut puzzle = Puzzle::new(data);
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
    const SAMPLE_4: &str = include_str!("sample_4.txt");
    const SAMPLE_9: &str = include_str!("sample_9.txt");
    const SAMPLE_10: &str = include_str!("sample_10.txt");
    const SAMPLE_11: &str = include_str!("sample_11.txt");

    #[test]
    fn test01() {
        let puzzle = Puzzle::new(SAMPLE_1);
        assert_eq!(puzzle.part1(), 8);
    }

    #[test]
    fn test02() {
        let puzzle = Puzzle::new(SAMPLE_4);
        assert_eq!(puzzle.part1(), 86);
    }

    #[test]
    fn test03() {
        let puzzle = Puzzle::new(SAMPLE_9);
        assert_eq!(puzzle.part1(), 132);
    }

    #[test]
    fn test04() {
        let puzzle = Puzzle::new(SAMPLE_10);
        assert_eq!(puzzle.part1(), 136);
    }

    #[test]
    fn test05() {
        let puzzle = Puzzle::new(SAMPLE_11);
        assert_eq!(puzzle.part1(), 81);
    }
}
