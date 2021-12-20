// Day 1: No Time for a Taxicab
// https://adventofcode.com/2016/day/1

use std::collections::HashSet;

fn main() {
    let data = std::fs::read_to_string("input.txt").unwrap();

    let mut x = 0i32;
    let mut y = 0i32;
    let mut angle = 0;

    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    let mut twice = false;

    for op2 in data.split(',') {
        let op = op2.trim();

        let direction = op.chars().next().unwrap();
        let distance = op[1..].parse::<i32>().unwrap();

        match direction {
            'L' => angle = (angle + 90) % 360,
            'R' => angle = (angle + 270) % 360,
            _ => panic!("unknown direction: {}", direction),
        }

        for _ in 1..=distance {
            match angle {
                0 => y += 1,
                90 => x += 1,
                180 => y -= 1,
                270 => x -= 1,
                _ => panic!("unknown angle: {}", angle),
            }

            if !twice && visited.contains(&(x, y)) {
                println!("twice: {}", x.abs() + y.abs());
                twice = true;
            } else {
                visited.insert((x, y));
            }
        }
    }

    println!("Easter Bunny HQ: {}", x.abs() + y.abs());
}
