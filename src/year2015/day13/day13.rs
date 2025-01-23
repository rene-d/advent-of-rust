//! [Day 13: Knights of the Dinner Table](https://adventofcode.com/2015/day/13)

use itertools::Itertools;
use regex::Regex;
use rustc_hash::{FxHashMap, FxHashSet};

fn calc<'a>(names: &FxHashSet<&'a str>, happiness: &FxHashMap<(&'a str, &'a str), i32>) -> i32 {
    let length = names.len();

    names
        .iter()
        .permutations(names.len())
        .map(|permutated| {
            (0..length)
                .map(|i| {
                    let n1 = permutated[i];
                    let n2 = permutated[(i + 1) % length];

                    happiness.get(&(n1, n2)).unwrap() + happiness.get(&(n2, n1)).unwrap()
                })
                .sum()
        })
        .max()
        .unwrap()
}

/// # Panics
/// over malformed input
#[must_use]
pub fn solve(data: &str) -> (i32, i32) {
    let mut names: FxHashSet<&str> = FxHashSet::default();
    let mut happiness: FxHashMap<(&str, &str), i32> = FxHashMap::default();

    let re =
        Regex::new(r"^(.+) would (gain|lose) (\d+) happiness units by sitting next to (.+)\.$")
            .unwrap();

    for line in data.lines() {
        if let Some(op) = re.captures(line) {
            let name = op.get(1).unwrap().as_str();
            let neighbor = op.get(4).unwrap().as_str();

            names.insert(name);
            names.insert(neighbor);

            let mut gain: i32 = op[3].parse().unwrap();
            if &op[2] == "lose" {
                gain = -gain;
            }

            happiness.insert((name, neighbor), gain);
        }
    }

    // part 1
    let part1 = calc(&names, &happiness);

    // part 2
    for name in &names {
        happiness.insert((name, "me"), 0);
        happiness.insert(("me", name), 0);
    }
    names.insert("me");
    let part2 = calc(&names, &happiness);

    (part1, part2)
}

/// main function
pub fn main() {
    let args = aoc::parse_args();
    args.run(solve);
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = include_str!("test.txt");

    #[test]
    fn test1() {
        assert_eq!(solve(TEST_INPUT).0, 330);
    }
}
