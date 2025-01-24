//! [Day 3: Mull It Over](https://adventofcode.com/2024/day/3)

use regex::Regex;

/// Compute valid `mul()` operations.
/// if part2 is true, take care of `do()`/`don't()` statements.
fn calc(data: &str, part: u8) -> i32 {
    let mut enabled = true;
    let mut total_sum = 0;

    let pattern = Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)").unwrap();

    for m in pattern.captures_iter(data) {
        if m.get(0).unwrap().as_str() == "do()" {
            enabled = true;
        } else if m.get(0).unwrap().as_str() == "don't()" {
            enabled = false;
        } else if enabled || part == 1 {
            let x = m[1].parse::<i32>().unwrap();
            let y = m[2].parse::<i32>().unwrap();
            total_sum += x * y;
        }
    }

    total_sum
}

/// # Panics
#[must_use]
pub fn solve(data: &str) -> (i32, i32) {
    (calc(data, 1), calc(data, 2))
}

pub fn main() {
    let args = aoc::parse_args();
    args.run(solve);
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE_1: &str = include_str!("sample_1.txt");
    const SAMPLE_2: &str = include_str!("sample_2.txt");

    #[test]
    fn test01() {
        assert_eq!(calc(SAMPLE_1, 1), 161);
        assert_eq!(calc(SAMPLE_1, 2), 161);
    }

    #[test]
    fn test02() {
        assert_eq!(calc(SAMPLE_2, 1), 161);
        assert_eq!(calc(SAMPLE_2, 2), 48);
    }
}
