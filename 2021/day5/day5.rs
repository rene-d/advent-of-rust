// Day 5: Hydrothermal Venture
// https://adventofcode.com/2021/day/5

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

    println!("reading data from: {}", args.path.display());

    let data = load_data(args.path);

    let re = Regex::new(r"^(\d+),(\d+) -> (\d+),(\d+)$").unwrap();

    // --- Part One ---
    let mut grid = [[0i16; 1000]; 1000];

    for line in &data {
        let drawn = re.captures(line).unwrap();

        let mut x1 = drawn[1].parse::<i32>().unwrap();
        let mut y1 = drawn[2].parse::<i32>().unwrap();
        let mut x2 = drawn[3].parse::<i32>().unwrap();
        let mut y2 = drawn[4].parse::<i32>().unwrap();

        if x1 == x2 {
            if y1 > y2 {
                std::mem::swap(&mut y1, &mut y2);
            }
            for y in y1..y2 + 1 {
                grid[x1 as usize][y as usize] += 1;
            }
        } else if y1 == y2 {
            if x1 > x2 {
                std::mem::swap(&mut x1, &mut x2);
            }
            for x in x1..x2 + 1 {
                grid[x as usize][y1 as usize] += 1;
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
    println!("{:?}", sum);

    // --- Part Two ---
    for line in &data {
        let drawn = re.captures(line).unwrap();

        let mut x1 = drawn[1].parse::<i32>().unwrap();
        let mut y1 = drawn[2].parse::<i32>().unwrap();
        let mut x2 = drawn[3].parse::<i32>().unwrap();
        let mut y2 = drawn[4].parse::<i32>().unwrap();

        if x1 != x2 && y1 != y2 {
            if x1 > x2 {
                std::mem::swap(&mut x1, &mut x2);
                std::mem::swap(&mut y1, &mut y2);
            }
            if y1 < y2 {
                for x in x1..x2 + 1 {
                    grid[x as usize][(y1 + (x - x1)) as usize] += 1;
                }
            } else {
                for x in x1..x2 + 1 {
                    grid[x as usize][(y1 - (x - x1)) as usize] += 1;
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
    println!("{:?}", sum2);
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

fn load_data(path: std::path::PathBuf) -> Vec<String> {
    let mut data = vec![];
    if let Ok(lines) = read_lines(path) {
        for line in lines.flatten() {
            data.push(line);
        }
    }
    data
}
