//! [Day 5: Hydrothermal Venture](https://adventofcode.com/2021/day/5)

use clap::Parser;
use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

/// parse command line arguments
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(help = "puzzle input", default_value = "input.txt")]
    path: String,
}

/// main function
fn main() {
    let args = Args::parse();

    // println!("reading data from: {}", args.path);

    let data = load_data(args.path);

    let re = Regex::new(r"^(\d+),(\d+) -> (\d+),(\d+)$").unwrap();

    // --- Part One ---
    let mut grid = vec![[0_i16; 1000]; 1000];

    for line in &data {
        let drawn = re.captures(line).unwrap();

        let mut x1 = drawn[1].parse::<usize>().unwrap();
        let mut y1 = drawn[2].parse::<usize>().unwrap();
        let mut x2 = drawn[3].parse::<usize>().unwrap();
        let mut y2 = drawn[4].parse::<usize>().unwrap();

        if x1 == x2 {
            if y1 > y2 {
                std::mem::swap(&mut y1, &mut y2);
            }
            for y in y1..=y2 {
                grid[x1][y] += 1;
            }
        } else if y1 == y2 {
            if x1 > x2 {
                std::mem::swap(&mut x1, &mut x2);
            }
            for row in grid.iter_mut().take(x2 + 1).skip(x1) {
                row[y1] += 1;
            }
        }
    }

    let mut sum = 0;
    for line in &grid {
        for val in line {
            if *val > 1 {
                sum += 1;
            }
        }
    }
    println!("{sum:?}");

    // --- Part Two ---
    for line in &data {
        let drawn = re.captures(line).unwrap();

        let mut x1 = drawn[1].parse::<usize>().unwrap();
        let mut y1 = drawn[2].parse::<usize>().unwrap();
        let mut x2 = drawn[3].parse::<usize>().unwrap();
        let mut y2 = drawn[4].parse::<usize>().unwrap();

        if x1 != x2 && y1 != y2 {
            if x1 > x2 {
                std::mem::swap(&mut x1, &mut x2);
                std::mem::swap(&mut y1, &mut y2);
            }
            if y1 < y2 {
                for (x, row) in grid.iter_mut().enumerate().take(x2 + 1).skip(x1) {
                    row[y1 + (x - x1)] += 1;
                }
            } else {
                for (x, row) in grid.iter_mut().enumerate().take(x2 + 1).skip(x1) {
                    row[y1 - (x - x1)] += 1;
                }
            }
        }
    }

    let mut sum2 = 0;
    for line in &grid {
        for val in line {
            if *val > 1 {
                sum2 += 1;
            }
        }
    }
    println!("{sum2:?}");
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

fn load_data(path: String) -> Vec<String> {
    let mut data = vec![];
    if let Ok(lines) = read_lines(path) {
        for line in lines.flatten() {
            data.push(line);
        }
    }
    data
}
