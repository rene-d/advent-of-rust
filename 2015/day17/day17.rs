//! [Day 17: No Such Thing as Too Much](https://adventofcode.com/2015/day/17)

use permutator::LargeCombinationIterator;
use rustc_hash::FxHashMap;

/// main function
fn main() {
    let args = aoc::parse_args();
    let data = &args.input;

    // read data into an array
    let values = data
        .lines()
        .filter(|line| !line.is_empty())
        .map(|s| s.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    let mut part1 = 0;
    let mut part2: FxHashMap<usize, usize> = FxHashMap::default();

    for i in 1..=values.len() {
        // try all the combinations with i containers
        let combinator = LargeCombinationIterator::new(&values, i);
        for combination in combinator {
            let sum: u32 = combination.iter().copied().sum();
            if sum == 150 {
                // part 1: count solutions
                part1 += 1;

                // part 2: count solutions by number of container
                *part2.entry(combination.len()).or_insert(0) += 1;
            }
        }
    }
    println!("{part1}");

    let min_container = part2.keys().min().unwrap();
    println!("{}", part2[min_container]);
}
