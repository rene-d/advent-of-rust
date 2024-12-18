//! [Day 18: RAM Run](https://adventofcode.com/2024/day/18)

use std::collections::VecDeque;

use aoc::grid::Grid;

const CORRUPTED: u8 = 0xCC;

struct Puzzle {
    byte_positions: Vec<(usize, usize)>,
    mem_size: usize,
    num_corruptions: usize,
}

impl Puzzle {
    fn new() -> Puzzle {
        Puzzle {
            byte_positions: Vec::new(),
            mem_size: 71,
            num_corruptions: 1024,
        }
    }

    /// Get the puzzle input.
    fn configure(&mut self, path: &str) {
        let data = std::fs::read_to_string(path).unwrap();

        for line in data.lines() {
            let (x, y) = line.split_once(',').unwrap();

            let x: usize = x.parse().unwrap();
            let y: usize = y.parse().unwrap();

            self.byte_positions.push((x, y));
        }
    }

    fn find_path(&self, memory: &Grid<u8>) -> u32 {
        let mut queue = VecDeque::new();
        let mut seen = Grid::<bool>::with_size(self.mem_size, self.mem_size);
        // nota: direct access is much faster than using a hashset

        let start = (0, 0);
        let end = (self.mem_size - 1, self.mem_size - 1);

        queue.push_back((start, 0));
        seen[start] = true;

        while let Some((pos, steps)) = queue.pop_front() {
            if pos == end {
                return steps;
            }

            for new_pos in memory.iter_directions(pos) {
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
        let mut memory: Grid<u8> = Grid::<u8>::with_size(self.mem_size, self.mem_size);

        self.byte_positions
            .iter()
            .take(self.num_corruptions)
            .for_each(|&pos| memory[pos] = CORRUPTED);

        self.find_path(&memory)
    }

    /// Solve part two.
    fn part2(&self) -> String {
        let mut memory: Grid<u8> = Grid::<u8>::with_size(self.mem_size, self.mem_size);

        let start = (0, 0);
        let end = (self.mem_size - 1, self.mem_size - 1);

        for &pos in &self.byte_positions {
            memory[pos] = CORRUPTED;

            if memory[start] == CORRUPTED
                || memory[end] == CORRUPTED
                || self.find_path(&memory) == 0
            {
                let (x, y) = pos;
                return format!("{x},{y}");
            }
        }

        String::new()
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
        puzzle.num_corruptions = 12;
        puzzle.mem_size = 7;
        assert_eq!(puzzle.part1(), 22);
    }

    #[test]
    fn test02() {
        let mut puzzle = Puzzle::new();
        puzzle.configure("sample_1.txt");
        puzzle.mem_size = 7;
        assert_eq!(puzzle.part2(), "6,1");
    }
}
