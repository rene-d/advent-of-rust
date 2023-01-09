//! [Day 2: Dive!](https://adventofcode.com/2021/day/2)

use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    part1();
    part2();
}

fn part1() {
    let mut pos_h = 0;
    let mut pos_v = 0;

    if let Ok(lines) = read_lines() {
        for line in lines.flatten() {
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
    }

    println!("{}", pos_h * pos_v);
}

fn part2() {
    let mut pos_h = 0;
    let mut pos_v = 0;
    let mut aim = 0;

    if let Ok(lines) = read_lines() {
        for line in lines.flatten() {
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
    }

    println!("{}", pos_h * pos_v);
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines() -> io::Result<io::Lines<io::BufReader<File>>> {
    let filename = if let Some(x) = std::env::args().collect::<Vec<String>>().get(1) {
        x.clone()
    } else {
        "input.txt".to_string()
    };

    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
