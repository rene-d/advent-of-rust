//! [Day 4: Camp Cleanup](https://adventofcode.com/2022/day/4)
use regex::Regex;
use std::cmp::{max, min};

struct Puzzle {
    part1: i32,
    part2: i32,
}

impl Puzzle {
    const fn new() -> Self {
        Self { part1: 0, part2: 0 }
    }

    fn configure(&mut self, data: &str) {
        let re = Regex::new(r"^(\d+)-(\d+),(\d+)-(\d+)").unwrap();

        self.part1 = 0;
        self.part2 = 0;

        for line in data.split('\n') {
            if let Some(m) = re.captures(line) {
                let a = m[1].parse::<i32>().unwrap();
                let b = m[2].parse::<i32>().unwrap();
                let c = m[3].parse::<i32>().unwrap();
                let d = m[4].parse::<i32>().unwrap();

                if (a <= c && c <= d && d <= b) || (c <= a && a <= b && b <= d) {
                    self.part1 += 1;
                }

                if max(0, min(b, d) - max(a, c) + 1) != 0 {
                    self.part2 += 1;
                }
            }
        }
    }
}

fn main() {
    let args = aoc::parse_args();
    let mut puzzle = Puzzle::new();
    puzzle.configure(&args.input);
    println!("{}", puzzle.part1);
    println!("{}", puzzle.part2);
}

#[test]
fn test_puzzle() {
    let mut puzzle = Puzzle::new();
    puzzle.configure(&aoc::load_input_data("test.txt"));
    assert_eq!(puzzle.part1, 2);
    assert_eq!(puzzle.part2, 4);
}
