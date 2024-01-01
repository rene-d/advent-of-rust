//! [Day 10: xxx](https://adventofcode.com/2017/day/10)

use clap::Parser;
use day10::HexDisplayExt;

#[derive(Parser)]
struct Args {
    /// Puzzle input
    #[arg(default_value = "input.txt")]
    path: String,
}

fn knot_tie(values: &[usize], sparse: &mut [u8], skip: &mut usize, pos: &mut usize) {
    let n = sparse.len();

    for length in values {
        for k in 0..(length / 2) {
            sparse.swap((*pos + k) % n, (*pos + length - 1 - k) % n);
        }

        *pos = (*pos + length + *skip) % n;
        *skip += 1;
    }
}

fn knot_hash(text: &str) -> String {
    let mut lengths: Vec<_> = text.chars().map(u32::from).map(|u| u as usize).collect();

    lengths.extend_from_slice(&[17, 31, 73, 47, 23]);

    let mut skip = 0;
    let mut pos = 0;
    let mut sparse: Vec<u8> = (0..=255).collect();

    for _ in 0..64 {
        knot_tie(&lengths, &mut sparse, &mut skip, &mut pos);
    }

    let dense: Vec<_> = sparse
        .chunks(16)
        .map(|chunk| chunk.iter().copied().reduce(|x, y| (x ^ y)).unwrap())
        .collect();

    dense.hex_display().to_string()
}

struct Puzzle {
    data: String,
}

impl Puzzle {
    fn new() -> Puzzle {
        Puzzle {
            data: String::new(),
        }
    }

    /// Get the puzzle input.
    fn configure(&mut self, path: &str) {
        let data = std::fs::read_to_string(path).unwrap();

        self.data = data.trim().to_owned();
    }

    /// Solve part one.
    fn part1(&self) -> u32 {
        let lengths: Vec<_> = self
            .data
            .split(',')
            .filter_map(|s| s.parse().ok())
            .collect();

        let mut skip = 0;
        let mut pos = 0;
        let mut sparse: Vec<u8> = (0..=255).collect();

        knot_tie(&lengths, &mut sparse, &mut skip, &mut pos);

        (sparse[0] as u32) * (sparse[1] as u32)
    }

    /// Solve part two.
    fn part2(&self) -> String {
        knot_hash(&self.data)
    }
}

fn main() {
    let args = Args::parse();
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
        let mut skip = 0;
        let mut pos = 0;
        let mut sparse: Vec<u8> = (0..=4).collect();

        knot_tie(&[3, 4, 1, 5], &mut sparse, &mut skip, &mut pos);

        assert_eq!(sparse, &[3, 4, 2, 1, 0]);
    }

    #[test]
    fn test02() {
        assert_eq!(knot_hash(""), "a2582a3a0e66e6e86e3812dcb672a272");
    }

    #[test]
    fn test03() {
        assert_eq!(knot_hash("AoC 2017"), "33efeb34ea91902bb2f59c9920caa6cd");
    }

    #[test]
    fn test04() {
        assert_eq!(knot_hash("1,2,3"), "3efbe78a8d82f29979031a4aa0b16a9d");
    }

    #[test]
    fn test05() {
        assert_eq!(knot_hash("1,2,4"), "63960835bcdc130f0b66d7ff4f6a5a8e");
    }
}
