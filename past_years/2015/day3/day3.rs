// Day 3: Perfectly Spherical Houses in a Vacuum
// https://adventofcode.com/2015/day/3

#![allow(unused_imports)]
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

    part1(&data[0]);
    part2(&data[0]);
}

fn part2(line: &str) {
    let mut visited = std::collections::HashSet::new();

    let mut position_santa = (0, 0);
    let mut position_robot = (0, 0);

    for (i, dir) in line.chars().enumerate() {
        if i % 2 == 0 {
            match dir {
                '<' => position_santa.0 -= 1,
                '>' => position_santa.0 += 1,
                '^' => position_santa.1 -= 1,
                'v' => position_santa.1 += 1,
                _ => panic!("invalid direction: {}", dir),
            }

            visited.insert(position_santa);
        } else {
            match dir {
                '<' => position_robot.0 -= 1,
                '>' => position_robot.0 += 1,
                '^' => position_robot.1 -= 1,
                'v' => position_robot.1 += 1,
                _ => panic!("invalid direction: {}", dir),
            }

            visited.insert(position_robot);
        }
    }

    println!("{}", visited.len());
}

fn part1(line: &str) {
    let mut visited = std::collections::HashSet::new();

    let mut position = (0, 0);

    for dir in line.chars() {
        match dir {
            '<' => position.0 -= 1,
            '>' => position.0 += 1,
            '^' => position.1 -= 1,
            'v' => position.1 += 1,
            _ => panic!("invalid direction: {}", dir),
        }

        visited.insert(position);
    }

    println!("{}", 1 + visited.len());
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
