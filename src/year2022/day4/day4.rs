//! [Day 4: Camp Cleanup](https://adventofcode.com/2022/day/4)

use regex::Regex;
use std::cmp::{max, min};

/// # Panics
#[must_use]
pub fn solve(data: &str) -> (u32, u32) {
    let re = Regex::new(r"^(\d+)-(\d+),(\d+)-(\d+)").unwrap();

    let mut part1 = 0;
    let mut part2 = 0;

    for line in data.lines() {
        if let Some(m) = re.captures(line) {
            let a = m[1].parse::<i32>().unwrap();
            let b = m[2].parse::<i32>().unwrap();
            let c = m[3].parse::<i32>().unwrap();
            let d = m[4].parse::<i32>().unwrap();

            if (a <= c && c <= d && d <= b) || (c <= a && a <= b && b <= d) {
                part1 += 1;
            }

            if max(0, min(b, d) - max(a, c) + 1) != 0 {
                part2 += 1;
            }
        }
    }

    (part1, part2)
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
    fn test_puzzle() {
        let answers = solve(TEST_INPUT);
        assert_eq!(answers.0, 2);
        assert_eq!(answers.1, 4);
    }
}
