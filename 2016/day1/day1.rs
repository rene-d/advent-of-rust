//! [Day 1: No Time for a Taxicab](https://adventofcode.com/2016/day/1)

use std::collections::HashSet;

/// ``main`` reads the puzzle input then solves part 1 and part 2
fn main() {
    let data = aoc::load_input_data(1);

    let mut x = 0_i32;
    let mut y = 0_i32;
    let mut angle = 0;

    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    let mut twice = false;
    let mut part2 = 0;

    for op2 in data.split(',') {
        let op = op2.trim();

        let direction = op.chars().next().unwrap();
        let distance = op[1..].parse::<i32>().unwrap();

        match direction {
            'L' => angle = (angle + 90) % 360,
            'R' => angle = (angle + 270) % 360,
            _ => panic!("unknown direction: {direction}"),
        }

        for _ in 1..=distance {
            match angle {
                0 => y += 1,
                90 => x += 1,
                180 => y -= 1,
                270 => x -= 1,
                _ => panic!("unknown angle: {angle}"),
            }

            if !twice && visited.contains(&(x, y)) {
                // println!("twice: {} (part 2)", x.abs() + y.abs());
                part2 = x.abs() + y.abs();
                twice = true;
            } else {
                visited.insert((x, y));
            }
        }
    }

    //println!("Easter Bunny HQ: {} (part 1)", x.abs() + y.abs());

    println!("{}", x.abs() + y.abs());
    println!("{part2}");
}
