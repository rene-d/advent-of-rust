// Day 2: Dive!
// https://adventofcode.com/2021/day/2

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    part1();
    part2();
}

fn part1() {
    let mut hpos = 0;
    let mut vpos = 0;

    if let Ok(lines) = read_lines("input.txt") {
        for line in lines.flatten() {
            if let Some((direction, _step)) = line.rsplit_once(' ') {
                let step = _step.parse::<i32>().unwrap();

                if direction == "forward" {
                    hpos += step;
                } else if direction == "down" {
                    vpos += step;
                } else if direction == "up" {
                    vpos -= step;
                }
            }
        }
    }

    println!("{}", hpos * vpos);
}

fn part2() {
    let mut hpos = 0;
    let mut vpos = 0;
    let mut aim = 0;

    if let Ok(lines) = read_lines("input.txt") {
        for line in lines.flatten() {
            if let Some((direction, _step)) = line.rsplit_once(' ') {
                let step = _step.parse::<i32>().unwrap();

                if direction == "down" {
                    aim += step;
                } else if direction == "up" {
                    aim -= step;
                } else if direction == "forward" {
                    hpos += step;
                    vpos += aim * step;
                }
            }
        }
    }

    println!("{}", hpos * vpos);
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
