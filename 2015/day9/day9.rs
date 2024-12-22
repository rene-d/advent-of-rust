//! [Day 9: All in a Single Night](https://adventofcode.com/2015/day/9)

use permutator::HeapPermutationIterator;
use regex::Regex;
use std::collections::{HashMap, HashSet};

/// main function
fn main() {
    let data = aoc::load_input_data_vec(9);

    let mut places: HashSet<String> = HashSet::new();
    let mut distances: HashMap<(String, String), u32> = HashMap::new();

    let re = Regex::new(r"^(.+) to (.+) = (\d+)$").unwrap();

    for line in &data {
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

    let perm_places = &mut places.iter().collect::<Vec<&String>>();
    let permutator = HeapPermutationIterator::new(perm_places);

    let mut min_distance = u32::MAX;
    let mut max_distance = u32::MIN;

    for permutated in permutator {
        let mut distance = 0;
        for i in 0..permutated.len() - 1 {
            let from = permutated[i];
            let to = permutated[i + 1];

            distance += distances.get(&(from.to_string(), to.to_string())).unwrap();
        }

        // println!("{:?}  ->  {}", permutated, distance);

        if distance < min_distance {
            min_distance = distance;
        }
        if distance > max_distance {
            max_distance = distance;
        }
    }

    println!("{min_distance}");
    println!("{max_distance}");
}
