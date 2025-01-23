//! [Day 12: Hill Climbing Algorithm](https://adventofcode.com/2022/day/12)

#![allow(clippy::cast_possible_wrap)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_sign_loss)]

use rustc_hash::FxHashSet;
use std::collections::VecDeque;

#[derive(Debug)]
struct Node {
    pos: (usize, usize),
    steps: usize,
}

struct Puzzle {
    grid: Vec<Vec<u8>>,
    nx: usize,
    ny: usize,
    start: (usize, usize),
    end: (usize, usize),
}

impl Puzzle {
    fn new(data: &str) -> Self {
        let mut puzzle = Self {
            grid: vec![],
            start: (0, 0),
            end: (0, 0),
            nx: 0,
            ny: 0,
        };

        let lines = data.split('\n').collect::<Vec<_>>();

        puzzle.nx = lines.first().unwrap().len();
        let mut y = 0;
        for line in lines {
            if line.is_empty() {
                continue;
            }
            let mut row = vec![0; puzzle.nx];
            for (x, pos) in line.chars().enumerate() {
                if pos == 'S' {
                    puzzle.start = (x, y);
                }
                row[x] = match pos {
                    'S' => {
                        // start position: height 0
                        puzzle.start = (x, y);
                        1
                    }
                    'E' => {
                        // end position: height 26 (like 'z')
                        puzzle.end = (x, y);
                        26
                    }
                    _ => (pos as u8) - 96, // other locations: height 1 to 26
                };
            }
            puzzle.grid.push(row);
            y += 1;
        }
        puzzle.ny = puzzle.grid.len();

        puzzle
    }

    /// Breadth-first search
    fn bfs(&self, part: u8) -> usize {
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
            for y in 0..self.ny {
                for x in 0..self.nx {
                    if self.grid[y][x] == 1 {
                        let n = Node {
                            pos: (x, y),
                            steps: 0,
                        };
                        q.push_back(n);
                    }
                }
            }
        }

        while !q.is_empty() {
            let n = q.pop_front().unwrap();
            let pos = n.pos;

            if pos == self.end {
                return n.steps;
            }

            if seen.contains(&pos) {
                continue;
            }
            seen.insert(pos);

            // Nota: need to figure out the best way to write this in Rust without ugly casts
            for step in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                let pos2 = (pos.0 as isize + step.0, pos.1 as isize + step.1);

                if 0 <= pos2.0
                    && pos2.0 < self.nx as isize
                    && 0 <= pos2.1
                    && pos2.1 < self.ny as isize
                {
                    let new_pos = (pos2.0 as usize, pos2.1 as usize);
                    if self.grid[new_pos.1][new_pos.0] <= 1 + self.grid[pos.1][pos.0] {
                        q.push_back(Node {
                            pos: new_pos,
                            steps: n.steps + 1,
                        });
                    }
                }
            }
        }

        0
    }

    // Solves part one
    fn part1(&self) -> usize {
        self.bfs(1)
    }

    // Solve part two
    fn part2(&self) -> usize {
        self.bfs(2)
    }
}

#[must_use]
pub fn solve(data: &str) -> (usize, usize) {
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
