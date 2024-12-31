//! [Day 16: Aunt Sue](https://adventofcode.com/2015/day/16)

use regex::Regex;
use std::collections::HashMap;

/// main function
fn main() {
    let args = aoc::parse_args();
    let data = &args.input;

    let re = Regex::new(r"Sue (\d+): (\w+): (\d+), (\w+): (\d+), (\w+): (\d+)").unwrap();

    let mut aunts: HashMap<u32, HashMap<String, u32>> = HashMap::new();

    for line in data.lines() {
        let m = re.captures(line).unwrap();
        let sue = m.get(1).unwrap().as_str().parse::<u32>().unwrap();

        for i in (2..=6).step_by(2) {
            let key = m.get(i).unwrap().as_str();
            let value = m.get(i + 1).unwrap().as_str().parse::<u32>().unwrap();
            aunts.entry(sue).or_default().insert(key.to_string(), value);
        }
    }

    // part 1
    for (sue, aunt) in &aunts {
        if aunt.get("children").unwrap_or(&3) == &3
            && aunt.get("cats").unwrap_or(&7) == &7
            && aunt.get("samoyeds").unwrap_or(&2) == &2
            && aunt.get("pomeranians").unwrap_or(&3) == &3
            && aunt.get("akitas").unwrap_or(&0) == &0
            && aunt.get("vizslas").unwrap_or(&0) == &0
            && aunt.get("goldfish").unwrap_or(&5) == &5
            && aunt.get("trees").unwrap_or(&3) == &3
            && aunt.get("cars").unwrap_or(&2) == &2
            && aunt.get("perfumes").unwrap_or(&1) == &1
        {
            println!("{sue}");
        }
    }

    // part 2
    for (sue, aunt) in &aunts {
        if aunt.get("children").unwrap_or(&3) == &3
            && aunt.get("cats").unwrap_or(&8) > &7          // should be greater than
            && aunt.get("samoyeds").unwrap_or(&2) == &2
            && aunt.get("pomeranians").unwrap_or(&2) < &3   // should be fewer than
            && aunt.get("akitas").unwrap_or(&0) == &0
            && aunt.get("vizslas").unwrap_or(&0) == &0
            && aunt.get("goldfish").unwrap_or(&4) < &5      // should be fewer than
            && aunt.get("trees").unwrap_or(&4) > &3         // should be greater than
            && aunt.get("cars").unwrap_or(&2) == &2
            && aunt.get("perfumes").unwrap_or(&1) == &1
        {
            println!("{sue}");
        }
    }
}
