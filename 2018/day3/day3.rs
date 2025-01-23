//! [Day 3: No Matter How You Slice It](https://adventofcode.com/2018/day/3)

use regex::Regex;
use rustc_hash::{FxHashMap, FxHashSet};

struct Puzzle<'a> {
    data: &'a str,
}

impl<'a> Puzzle<'a> {
    const fn new(data: &'a str) -> Self {
        Self { data }
    }

    /// Solve part one.
    fn part1(&self) -> usize {
        let re = Regex::new(r"^#([0-9]+) @ ([0-9]+),([0-9]+): ([0-9]+)x([0-9]+)$").unwrap();

        let mut squares = FxHashMap::default();

        for line in self.data.lines() {
            if let Some(caps) = re.captures(line) {
                let s = caps
                    .iter()
                    .skip(2)
                    .map(|c| c.unwrap().as_str().parse::<u32>().unwrap())
                    .collect::<Vec<_>>();

                let x = s[0];
                let y = s[1];

                let width = s[2];
                let height = s[3];

                for i in x..(x + width) {
                    for j in y..(y + height) {
                        *squares.entry((i, j)).or_insert(0) += 1u32;
                    }
                }
            }
        }

        squares.values().filter(|&&v| v > 1).count()
    }

    /// Solve part two.
    fn part2(&self) -> u32 {
        let re = Regex::new(r"^#([0-9]+) @ ([0-9]+),([0-9]+): ([0-9]+)x([0-9]+)$").unwrap();

        let mut squares_id: FxHashMap<(u32, u32), u32> = FxHashMap::default();
        let mut intact = FxHashSet::default();

        for line in self.data.lines() {
            if let Some(caps) = re.captures(line) {
                let s = caps
                    .iter()
                    .skip(1)
                    .map(|c| c.unwrap().as_str().parse::<u32>().unwrap())
                    .collect::<Vec<_>>();

                let id = s[0];

                let x = s[1];
                let y = s[2];

                let width = s[3];
                let height = s[4];

                intact.insert(id);

                for i in x..(x + width) {
                    for j in y..(y + height) {
                        let key = (i, j);

                        if let std::collections::hash_map::Entry::Vacant(e) = squares_id.entry(key)
                        {
                            e.insert(id);
                        } else {
                            intact.remove(&id);

                            let o = squares_id[&key];
                            intact.remove(&o);
                        }
                    }
                }
            }
        }

        *intact.iter().next().unwrap()
    }
}

/// # Panics
/// over malformed input
#[must_use]
pub fn solve(data: &str) -> (usize, u32) {
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

    const TEST_INPUT: &str = include_str!("test.txt");

    #[test]
    fn test01() {
        let puzzle = Puzzle::new(TEST_INPUT);
        assert_eq!(puzzle.part1(), 4);
    }

    #[test]
    fn test02() {
        let puzzle = Puzzle::new(TEST_INPUT);
        assert_eq!(puzzle.part2(), 3);
    }
}
