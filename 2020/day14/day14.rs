//! [Day 14: Docking Data](https://adventofcode.com/2020/day/14)

use std::collections::HashMap;

struct Puzzle {
    data: String,
}

impl Puzzle {
    const fn new() -> Self {
        Self {
            data: String::new(),
        }
    }

    /// Get the puzzle input.
    fn configure(&mut self, data: &str) {
        self.data = data.to_string();
    }

    /// Solve part one.
    fn part1(&self) -> u64 {
        let mut mem = HashMap::new();
        let mut or_mask = 0;
        let mut and_mask = 0;

        for line in self.data.lines() {
            if let Some(mask) = line.strip_prefix("mask = ") {
                or_mask = u64::from_str_radix(mask.replace('X', "0").as_str(), 2).unwrap();
                and_mask = u64::from_str_radix(mask.replace('X', "1").as_str(), 2).unwrap();
            }

            if let Some(s) = line.strip_prefix("mem[") {
                let (addr, value) = s.split_once("] = ").unwrap();
                let value: u64 = value.parse().unwrap();
                mem.insert(addr, (value & and_mask) | or_mask);
            }
        }

        mem.values().sum()
    }

    /// Solve part two.
    fn part2(&self) -> u64 {
        let mut mem = HashMap::new();
        let mut and_mask = 0;
        let mut x_mask: &str = "";

        for line in self.data.lines() {
            if let Some(mask) = line.strip_prefix("mask = ") {
                x_mask = mask;

                // mask to cancel X bits
                and_mask =
                    u64::from_str_radix(mask.replace('0', "1").replace('X', "0").as_str(), 2)
                        .unwrap();
            }

            if let Some(s) = line.strip_prefix("mem[") {
                let (addr, value) = s.split_once("] = ").unwrap();
                let addr: u64 = addr.parse().unwrap();
                let value: u64 = value.parse().unwrap();

                let n = 1 << x_mask.chars().filter(|c| c == &'X').count();

                for mut i in 0..n {
                    let mut or_mask = 0;

                    for digit in x_mask.chars() {
                        or_mask = (or_mask << 1)
                            + if digit == 'X' {
                                let digit = i % 2;
                                i >>= 1;
                                digit
                            } else {
                                u64::from(digit.to_digit(2).unwrap())
                            };
                    }

                    mem.insert(or_mask | (addr & and_mask), value);
                }
            }
        }

        mem.values().sum()
    }
}

fn main() {
    let args = aoc::parse_args();
    let mut puzzle = Puzzle::new();
    puzzle.configure(&args.input);
    println!("{}", puzzle.part1());
    println!("{}", puzzle.part2());
}

/// Test from puzzle input
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let mut puzzle = Puzzle::new();
        puzzle.configure(&aoc::load_input_data("sample_1.txt"));
        assert_eq!(puzzle.part1(), 165);
    }

    #[test]
    fn test_part2() {
        let mut puzzle = Puzzle::new();
        puzzle.configure(&aoc::load_input_data("sample_5.txt"));
        assert_eq!(puzzle.part2(), 208);
    }
}
