//! [Day 14: Docking Data](https://adventofcode.com/2020/day/14)

use rustc_hash::FxHashMap;

struct Puzzle<'a> {
    data: &'a str,
}

impl<'a> Puzzle<'a> {
    const fn new(data: &'a str) -> Self {
        Self { data }
    }

    /// Solve part one.
    fn part1(&self) -> u64 {
        let mut mem = FxHashMap::default();
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
        let mut mem = FxHashMap::default();
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

/// # Panics
#[must_use]
pub fn solve(data: &str) -> (u64, u64) {
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
    const SAMPLE_5: &str = include_str!("sample_5.txt");

    #[test]
    fn test_part1() {
        let puzzle = Puzzle::new(SAMPLE_1);
        assert_eq!(puzzle.part1(), 165);
    }

    #[test]
    fn test_part2() {
        let puzzle = Puzzle::new(SAMPLE_5);
        assert_eq!(puzzle.part2(), 208);
    }
}
