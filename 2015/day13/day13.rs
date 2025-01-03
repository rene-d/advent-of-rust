//! [Day 13: Knights of the Dinner Table](https://adventofcode.com/2015/day/13)

use permutator::HeapPermutationIterator;
use regex::Regex;
use rustc_hash::{FxHashMap, FxHashSet};

fn calc(names: &FxHashSet<String>, happiness: &FxHashMap<(String, String), i32>) -> i32 {
    let perm_names = &mut names.iter().collect::<Vec<&String>>();
    let permutator = HeapPermutationIterator::new(perm_names);

    let mut happiness_max = i32::MIN;

    for permutated in permutator {
        let mut happiness_sum = 0;
        for i in 0..permutated.len() {
            let n1 = permutated[i];
            let n2 = permutated[(i + 1) % permutated.len()];

            happiness_sum += happiness.get(&(n1.to_string(), n2.to_string())).unwrap();
            happiness_sum += happiness.get(&(n2.to_string(), n1.to_string())).unwrap();
        }

        if happiness_max < happiness_sum {
            happiness_max = happiness_sum;
        }
    }
    happiness_max
}

/// main function
fn main() {
    let args = aoc::parse_args();
    let data = args
        .input
        .lines()
        .map(std::string::ToString::to_string)
        .collect::<Vec<String>>();

    let mut names: FxHashSet<String> = FxHashSet::default();
    let mut happiness: FxHashMap<(String, String), i32> = FxHashMap::default();

    let re =
        Regex::new(r"^(.+) would (gain|lose) (\d+) happiness units by sitting next to (.+)\.$")
            .unwrap();

    for line in &data {
        if let Some(op) = re.captures(line) {
            names.insert(op[1].to_string());
            names.insert(op[4].to_string());

            let mut gain: i32 = op[3].parse().unwrap();
            if op[2].to_string() == "lose" {
                gain = -gain;
            }

            happiness.insert((op[1].to_string(), op[4].to_string()), gain);
        }
    }

    // part 1
    println!("{}", calc(&names, &happiness));

    // part 2
    for name in &names {
        happiness.insert((name.to_string(), "me".to_string()), 0);
        happiness.insert(("me".to_string(), name.to_string()), 0);
    }
    names.insert("me".to_string());

    println!("{}", calc(&names, &happiness));
}
