//! [Day 9: All in a Single Night](https://adventofcode.com/2015/day/9)

use itertools::Itertools;
use regex::Regex;
use rustc_hash::{FxHashMap, FxHashSet};

/// # Panics
/// over malformed input
pub fn main() {
    let args = aoc::parse_args();
    args.run(solve);
}

/// # Panics
/// over malformed input
#[must_use]
pub fn solve(data: &str) -> (u32, u32) {
    let mut places: FxHashSet<String> = FxHashSet::default();
    let mut distances: FxHashMap<(String, String), u32> = FxHashMap::default();

    let re = Regex::new(r"^(.+) to (.+) = (\d+)$").unwrap();

    for line in data.lines() {
        if let Some(op) = re.captures(line) {
            places.insert(op[1].to_string());
            places.insert(op[2].to_string());

            distances.insert(
                (op[1].to_string(), op[2].to_string()),
                op[3].parse().unwrap(),
            );
            distances.insert(
                (op[2].to_string(), op[1].to_string()),
                op[3].parse().unwrap(),
            );
        }
    }

    let mut min_distance = u32::MAX;
    let mut max_distance = u32::MIN;

    for permutated in places.iter().permutations(places.len()) {
        let mut distance = 0;
        for i in 0..permutated.len() - 1 {
            let from = permutated[i];
            let to = permutated[i + 1];

            distance += distances.get(&(from.to_string(), to.to_string())).unwrap();
        }

        if distance < min_distance {
            min_distance = distance;
        }
        if distance > max_distance {
            max_distance = distance;
        }
    }

    (min_distance, max_distance)
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = include_str!("test.txt");

    #[test]
    fn test1() {
        assert_eq!(solve(TEST_INPUT), (605, 982));
    }
}
