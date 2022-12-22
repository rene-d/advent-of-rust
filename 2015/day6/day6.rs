//! [Day 6: Probably a Fire Hazard](https://adventofcode.com/2015/day/6)

use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use structopt::StructOpt;

/// parse command line arguments
#[derive(StructOpt)]
struct Cli {
    #[structopt(default_value = "input.txt", parse(from_os_str))]
    path: std::path::PathBuf,
}

/// main function
fn main() {
    let args = Cli::from_args();
    // println!("reading data from: {}", args.path.display());
    let data = load_data(args.path);

    part1(&data);
    part2(&data);
}

fn part2(data: &[String]) {
    let mut grid = vec![[0_i8; 1000]; 1000];

    let re = Regex::new(r"^(turn on|turn off|toggle) (\d+),(\d+) through (\d+),(\d+)$").unwrap();

    for line in data {
        let drawn = re.captures(line).unwrap();

        let op = drawn[1].to_string();
        let x1 = drawn[2].parse::<usize>().unwrap();
        let y1 = drawn[3].parse::<usize>().unwrap();
        let x2 = drawn[4].parse::<usize>().unwrap();
        let y2 = drawn[5].parse::<usize>().unwrap();

        if op == "turn on" {
            for line in grid.iter_mut().take(x2 + 1).skip(x1) {
                for val in line.iter_mut().take(y2 + 1).skip(y1) {
                    *val += 1;
                }
            }
        } else if op == "turn off" {
            for line in grid.iter_mut().take(x2 + 1).skip(x1) {
                for val in line.iter_mut().take(y2 + 1).skip(y1) {
                    if *val > 0 {
                        *val -= 1;
                    }
                }
            }
        } else if op == "toggle" {
            for line in grid.iter_mut().take(x2 + 1).skip(x1) {
                for val in line.iter_mut().take(y2 + 1).skip(y1) {
                    *val += 2;
                }
            }
        }
    }

    let mut count: u32 = 0;
    for row in &grid {
        for cell in row.iter() {
            count += u32::try_from(*cell).unwrap();
        }
    }
    println!("{}", count);
}

fn part1(data: &[String]) {
    let mut grid = vec![[0_i8; 1000]; 1000];

    let re = Regex::new(r"^(turn on|turn off|toggle) (\d+),(\d+) through (\d+),(\d+)$").unwrap();

    for line in data {
        let drawn = re.captures(line).unwrap();

        let op = drawn[1].to_string();
        let x1 = drawn[2].parse::<usize>().unwrap();
        let y1 = drawn[3].parse::<usize>().unwrap();
        let x2 = drawn[4].parse::<usize>().unwrap();
        let y2 = drawn[5].parse::<usize>().unwrap();

        if op == "turn on" {
            for line in grid.iter_mut().take(x2 + 1).skip(x1) {
                for val in line.iter_mut().take(y2 + 1).skip(y1) {
                    *val = 1;
                }
            }
        } else if op == "turn off" {
            for line in grid.iter_mut().take(x2 + 1).skip(x1) {
                for val in line.iter_mut().take(y2 + 1).skip(y1) {
                    *val = 0;
                }
            }
        } else if op == "toggle" {
            for line in grid.iter_mut().take(x2 + 1).skip(x1) {
                for val in line.iter_mut().take(y2 + 1).skip(y1) {
                    *val = 1 - *val;
                }
            }
        }
    }

    let mut count: u32 = 0;
    for row in &grid {
        for cell in row.iter() {
            count += u32::try_from(*cell).unwrap();
        }
    }
    println!("{}", count);
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

/// load data from file
fn load_data(path: std::path::PathBuf) -> Vec<String> {
    let mut data = vec![];
    if let Ok(lines) = read_lines(path) {
        for line in lines.flatten() {
            data.push(line);
        }
    }
    data
}
