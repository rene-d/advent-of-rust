//! [Day 12: Hill Climbing Algorithm](https://adventofcode.com/2022/day/12)

use rustc_hash::FxHashSet;
use std::collections::VecDeque;

#[derive(Debug)]
struct Node {
    pos: (usize, usize),
    steps: u32,
}

struct Puzzle {
    grid: aoc::GridU<u8>,
    start: (usize, usize),
    end: (usize, usize),
}

impl Puzzle {
    fn new(data: &str) -> Self {
        let mut puzzle = Self {
            grid: aoc::GridU::<u8>::parse(data.trim_ascii()),
            start: (0, 0),
            end: (0, 0),
        };

        for (pos, c) in puzzle.grid.iter_mut() {
            *c = match c {
                b'S' => {
                    puzzle.start = pos; // start position: height 1
                    1
                }
                b'E' => {
                    puzzle.end = pos; // end position: height 26 (like 'z')}
                    26
                }
                b'a'..=b'z' => *c - b'a' + 1, // other locations: height 1 to 26
                _ => panic!("unknown elevation {c}"),
            };
        }

        puzzle
    }

    /// Breadth-first search
    fn bfs(&self, part: u8) -> u32 {
        let mut q: VecDeque<Node> = VecDeque::new();
        let mut seen: FxHashSet<(usize, usize)> = FxHashSet::default();

        // set up the start positions:
        // 'S' if part 1
        // 'S' or any 'a' if part 2 (height 1)
        if part == 1 {
            q.push_back(Node {
                pos: self.start,
                steps: 0,
            });
        } else {
            for (pos, c) in self.grid.iter() {
                if *c == 1 {
                    let n = Node { pos, steps: 0 };
                    q.push_back(n);
                }
            }
        }

        while !q.is_empty() {
            let n = q.pop_front().unwrap();
            let pos = n.pos;

            if pos == self.end {
                return n.steps;
            }

            if !seen.insert(pos) {
                continue;
            }

            for new_pos in self.grid.iter_directions(pos) {
                if self.grid[new_pos] <= 1 + self.grid[pos] {
                    q.push_back(Node {
                        pos: new_pos,
                        steps: n.steps + 1,
                    });
                }
            }
        }

        0
    }

    // Solves part one
    fn part1(&self) -> u32 {
        self.bfs(1)
    }

    // Solve part two
    fn part2(&self) -> u32 {
        self.bfs(2)
    }
}

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

    #[test]
    fn test01() {
        let puzzle = Puzzle::new(TEST_INPUT);
        assert_eq!(puzzle.part1(), 31);
        assert_eq!(puzzle.part2(), 29);
    }
}
