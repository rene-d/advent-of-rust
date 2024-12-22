//! [Day 4: Secure Container](https://adventofcode.com/2019/day/4)

// use std::collections::{HashMap,HashSet};

struct Puzzle {
    a: u32,
    b: u32,
}

impl Puzzle {
    const fn new() -> Self {
        Self { a: 0, b: 0 }
    }

    /// Get the puzzle input.
    fn configure(&mut self, path: &str) {
        let data = std::fs::read_to_string(path).unwrap();

        let (a, b) = data.trim_ascii().split_once('-').unwrap();

        self.a = a.parse().unwrap();
        self.b = b.parse().unwrap();
    }

    /// Solve part one.
    fn part1(&self) -> u32 {
        let mut result = 0;

        for n in self.a..=self.b {
            let s = n.to_string();
            let mut ok = true;
            let mut same_adj = false;
            for (i, j) in s.chars().zip(s.chars().skip(1)) {
                if j < i {
                    ok = false;
                    break;
                }
                if i == j {
                    same_adj = true;
                }
            }
            if same_adj && ok {
                result += 1;
            }
        }
        result
    }

    /// Solve part two.
    fn part2(&self) -> u32 {
        let mut result = 0;

        for n in self.a..=self.b {
            let s = n.to_string();
            let mut ok = true;
            let mut freq = [0; 10];
            for (i, j) in s.chars().zip(s.chars().skip(1)) {
                if j < i {
                    ok = false;
                    break;
                }
            }
            if ok {
                for c in s.chars() {
                    freq[c.to_digit(10).unwrap() as usize] += 1;
                }
                if freq.iter().any(|&x| x == 2) {
                    result += 1;
                }
            }
        }
        result
    }
}

fn main() {
    let args = aoc::parse_args();
    let mut puzzle = Puzzle::new();
    puzzle.configure(args.path.as_str());
    println!("{}", puzzle.part1());
    println!("{}", puzzle.part2());
}
