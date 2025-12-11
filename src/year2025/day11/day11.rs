//! [Day 11: Reactor](https://adventofcode.com/2025/day/11)

use rustc_hash::FxHashMap;

struct Puzzle<'a> {
    outputs: FxHashMap<&'a str, Vec<&'a str>>,
}

impl<'a> Puzzle<'a> {
    /// Initialize from the puzzle input.
    fn new(data: &'a str) -> Self {
        Self {
            outputs: data
                .lines()
                .map(|line| {
                    let (device, s) = line.split_once(": ").unwrap();

                    (device, s.split_ascii_whitespace().collect())
                })
                .collect(),
        }
    }

    /// Solve part one.
    fn part1(&self) -> u64 {
        fn count<'b>(
            key: &'b str,
            f: &FxHashMap<&'b str, Vec<&'b str>>,
            memo: &mut FxHashMap<&'b str, u64>,
        ) -> u64 {
            if let Some(&v) = memo.get(key) {
                return v;
            }
            if key == "out" {
                return 1;
            }
            let sum = f[key].iter().map(|child| count(child, f, memo)).sum();
            memo.insert(key, sum);
            sum
        }

        let mut memo = FxHashMap::default();
        count("you", &self.outputs, &mut memo)
    }

    /// Solve part two.
    fn part2(&self) -> u64 {
        fn count<'b>(
            key: &'b str,
            f: &FxHashMap<&'b str, Vec<&'b str>>,
            vdac: bool,
            vfft: bool,
            memo: &mut FxHashMap<(&'b str, bool, bool), u64>,
        ) -> u64 {
            let memo_key = (key, vdac, vfft);

            if let Some(&v) = memo.get(&memo_key) {
                return v;
            }

            if key == "out" {
                let v = u64::from(vdac && vfft);
                memo.insert(memo_key, v);
                return v;
            }

            let mut new_vdac = vdac;
            let mut new_vfft = vfft;

            if key == "dac" {
                new_vdac = true;
            }
            if key == "fft" {
                new_vfft = true;
            }

            let sum = f[key]
                .iter()
                .map(|child| count(child, f, new_vdac, new_vfft, memo))
                .sum();

            memo.insert(memo_key, sum);
            sum
        }

        let mut memo = FxHashMap::default();
        count("svr", &self.outputs, false, false, &mut memo)
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
    const SAMPLE_2: &str = include_str!("sample_2.txt");

    #[test]
    fn part1() {
        let puzzle = Puzzle::new(SAMPLE_1);
        assert_eq!(puzzle.part1(), 5);
    }

    #[test]
    fn part2() {
        let puzzle = Puzzle::new(SAMPLE_2);
        assert_eq!(puzzle.part2(), 2);
    }
}
