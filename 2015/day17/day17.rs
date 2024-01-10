//! [Day 17: No Such Thing as Too Much](https://adventofcode.com/2015/day/17)

use permutator::LargeCombinationIterator;
use std::collections::HashMap;

/// main function
fn main() {
    let data = aoc::load_input_data(17);

    // read data into an array
    let values = data
        .lines()
        .map(|s| s.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    let mut part1 = 0;
    let mut part2: HashMap<usize, usize> = HashMap::new();

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
