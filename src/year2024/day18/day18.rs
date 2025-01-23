//! [Day 18: RAM Run](https://adventofcode.com/2024/day/18)

use std::collections::VecDeque;

use aoc::{Coord, Grid};

const CORRUPTED: u8 = 0xCC;

struct Puzzle {
    byte_positions: Vec<Coord>,
    mem_size: i32,
    num_corruptions: usize,
}

impl Puzzle {
    fn new(data: &str) -> Self {
        let mut byte_positions = Vec::new();

        for line in data.lines() {
            let (x, y) = line.split_once(',').unwrap();

            let x: i32 = x.parse().unwrap();
            let y: i32 = y.parse().unwrap();

            byte_positions.push(Coord { x, y });
        }

        Self {
            byte_positions,
            mem_size: 71,
            num_corruptions: 1024,
        }
    }

    fn find_path(&self, memory: &Grid<u8>) -> u32 {
        let mut queue = VecDeque::new();
        let mut seen = Grid::<bool>::with_size(self.mem_size, self.mem_size, false, false);
        // nota: direct access is much faster than using a hashset

        let start = Coord::new(0, 0);
        let end = Coord::new(self.mem_size - 1, self.mem_size - 1);

        queue.push_back((start, 0));
        seen[start] = true;

        while let Some((pos, steps)) = queue.pop_front() {
            if pos == end {
                return steps;
            }

            for (_, new_pos) in memory.iter_directions(pos) {
                if memory[new_pos] != CORRUPTED && !seen[new_pos] {
                    queue.push_back((new_pos, steps + 1));
                    seen[new_pos] = true;
                }
            }
        }

        0
    }

    /// Solve part one.
    fn part1(&self) -> u32 {
        let mut memory = Grid::<u8>::with_size(self.mem_size, self.mem_size, b' ', b'#');

        self.byte_positions
            .iter()
            .take(self.num_corruptions)
            .for_each(|&pos| memory[pos] = CORRUPTED);

        self.find_path(&memory)
    }

    /// Solve part two.
    fn part2(&self) -> String {
        let mut a = 0;
        let mut b = self.byte_positions.len() - 1;

        while a + 1 < b {
            let mut memory = Grid::<u8>::with_size(self.mem_size, self.mem_size, b' ', b'#');

            let m = (a + b) / 2;

            // corrupt the first m bytes
            for &pos in self.byte_positions.iter().take(m) {
                memory[pos] = CORRUPTED;
            }

            if self.find_path(&memory) == 0 {
                b = m;
            } else {
                a = m;
            }
        }

        format!("{},{}", self.byte_positions[a].x, self.byte_positions[a].y)
    }
}

/// # Panics
/// over malformed input
#[must_use]
pub fn solve(data: &str) -> (u32, String) {
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

    #[test]
    fn test01() {
        let mut puzzle = Puzzle::new(SAMPLE_1);
        puzzle.num_corruptions = 12;
        puzzle.mem_size = 7;
        assert_eq!(puzzle.part1(), 22);
    }

    #[test]
    fn test02() {
        let mut puzzle = Puzzle::new(SAMPLE_1);
        puzzle.mem_size = 7;
        assert_eq!(puzzle.part2(), "6,1");
    }
}
