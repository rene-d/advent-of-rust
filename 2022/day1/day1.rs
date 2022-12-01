//! [Day 1: Calorie Counting](https://adventofcode.com/2022/day/1)

/// ``main`` reads the puzzle input then solves part 1 and part 2
fn main() {
    let data = std::fs::read_to_string("input.txt").unwrap();

    let mut lines = data.split('\n').collect::<Vec<_>>();
    lines.push(""); // add ane empty line to flush the energy accumulator into the array

    let mut energy = 0;
    let mut reeinders = Vec::new();

    for line in lines {
        if line.is_empty() {
            reeinders.push(energy);
            energy = 0;
        } else {
            energy += line.parse::<u32>().unwrap();
        }
    }
    reeinders.sort_by(|a, b| b.cmp(a));

    println!("part1: {}", reeinders.first().unwrap());
    println!(
        "part2: {}",
        reeinders.get(0).unwrap() + reeinders.get(1).unwrap() + reeinders.get(2).unwrap()
    );
}
