//! [Day 17: No Such Thing as Too Much](https://adventofcode.com/2015/day/17)

use itertools::Itertools;
use rustc_hash::FxHashMap;

/// main function
fn main() {
    let mut args = aoc::parse_args();
    args.run(solve);
}

fn solve(data: &str) -> (i32, usize) {
    solve_eggnot(data, 150)
}

fn solve_eggnot(data: &str, eggnot: u32) -> (i32, usize) {
    // read data into an array
    let values = data
        .lines()
        .filter_map(|line| line.parse().ok())
        .collect::<Vec<u32>>();

    let mut part1 = 0;
    let mut part2 = FxHashMap::default();

    for k in 1..=values.len() {
        // try all the combinations with i containers
        for combination in values.iter().combinations(k) {
            let sum: u32 = combination.iter().copied().sum();
            if sum == eggnot {
                // part 1: count solutions
                part1 += 1;

                // part 2: count solutions by number of container
                *part2.entry(k).or_insert(0) += 1;
            }
        }
    }

    let min_container = part2.keys().min().unwrap();

    (part1, part2[min_container])
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = include_str!("test.txt");

    #[test]
    fn test_solve() {
        assert_eq!(solve_eggnot(TEST_INPUT, 25), (4, 3));
    }
}
