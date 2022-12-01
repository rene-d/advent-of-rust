//! [Day 1: Calorie Counting](https://adventofcode.com/2022/day/1)

/// ``main`` reads the puzzle input then solves part 1 and part 2
fn main() {
    let data = std::fs::read_to_string("input.txt").unwrap();

    let mut lines = data.split('\n').collect::<Vec<_>>();
    lines.push(""); // add ane empty line to flush the energy accumulator into the array

    let mut energy = 0;
    let mut calories = Vec::new();

    for line in lines {
        if line.is_empty() {
            calories.push(energy);
            energy = 0;
        } else {
            energy += line.parse::<u32>().unwrap();
        }
    }
    calories.sort_by(|a, b| b.cmp(a));

    println!("part1: {}", calories.first().unwrap());
    println!(
        "part2: {}",
        calories.get(0).unwrap() + calories.get(1).unwrap() + calories.get(2).unwrap()
    );
}
