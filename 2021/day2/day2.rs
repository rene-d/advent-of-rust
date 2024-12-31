//! [Day 2: Dive!](https://adventofcode.com/2021/day/2)

fn main() {
    let args = aoc::parse_args();
    part1(&args.input);
    part2(&args.input);
}

fn part1(data: &str) {
    let mut pos_h = 0;
    let mut pos_v = 0;

    for line in data.lines() {
        if let Some((direction, step_str)) = line.rsplit_once(' ') {
            let step = step_str.parse::<i32>().unwrap();

            if direction == "forward" {
                pos_h += step;
            } else if direction == "down" {
                pos_v += step;
            } else if direction == "up" {
                pos_v -= step;
            }
        }
    }

    println!("{}", pos_h * pos_v);
}

fn part2(data: &str) {
    let mut pos_h = 0;
    let mut pos_v = 0;
    let mut aim = 0;

    for line in data.lines() {
        if let Some((direction, step_str)) = line.rsplit_once(' ') {
            let step = step_str.parse::<i32>().unwrap();

            if direction == "down" {
                aim += step;
            } else if direction == "up" {
                aim -= step;
            } else if direction == "forward" {
                pos_h += step;
                pos_v += aim * step;
            }
        }
    }

    println!("{}", pos_h * pos_v);
}
