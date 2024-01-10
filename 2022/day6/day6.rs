//! [Day 6: Tuning Trouble](https://adventofcode.com/2022/day/6)

#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use std::collections::HashSet;

struct Puzzle {
    /// Puzzle input
    data: String,
}

impl Puzzle {
    const fn new() -> Self {
        Self {
            data: String::new(),
        }
    }

    /// Loads data from input (one line)
    fn configure(&mut self, path: &str) {
        self.data = std::fs::read_to_string(path).unwrap();
        if let Some(stripped) = self.data.strip_suffix('\n') {
            self.data = stripped.to_string();
        }
    }

    // Solves part one
    fn part1(&self) -> usize {
        find_marker(&self.data, 4)
    }

    // Solve part two
    fn part2(&self) -> usize {
        find_marker(&self.data, 14)
    }
}

/// find the position of the first marker of the given length
fn find_marker(data: &str, length: usize) -> usize {
    if data.len() >= length {
        for i in 0..=(data.len() - length) {
            let mut marker = HashSet::new();
            for k in 0..length {
                marker.insert(data.chars().nth(i + k));
            }
            if marker.len() == length {
                return i + length;
            }
        }
    }
    0
}

/// main function
fn main() {
    let args = aoc::parse_args();
    let mut puzzle = Puzzle::new();
    puzzle.configure(&args.path);
    println!("{}", puzzle.part1());
    println!("{}", puzzle.part2());
}

#[test]
fn test01() {
    let mut puzzle = Puzzle::new();
    puzzle.configure("test.txt");
    assert_eq!(puzzle.part1(), 7);
    assert_eq!(puzzle.part2(), 19);
}

#[test]
fn test02() {
    assert_eq!(find_marker("bvwbjplbgvbhsrlpgdmjqwftvncz", 4), 5);
    assert_eq!(find_marker("nppdvjthqldpwncqszvftbrmjlhg", 4), 6);
    assert_eq!(find_marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4), 10);
    assert_eq!(find_marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4), 11);
}

#[test]
fn test03() {
    assert_eq!(find_marker("bvwbjplbgvbhsrlpgdmjqwftvncz", 14), 23);
    assert_eq!(find_marker("nppdvjthqldpwncqszvftbrmjlhg", 14), 23);
    assert_eq!(find_marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 14), 29);
    assert_eq!(find_marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 14), 26);
}
