// Day 11: Dumbo Octopus
// https://adventofcode.com/2021/day/11

#![allow(clippy::needless_range_loop)]
#![allow(clippy::collapsible_if)]
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

    // read the grid
    let n = data.len();
    let mut grid = vec![vec![0i8; n]; n];
    for (y, line) in data.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            grid[y][x] = c.to_digit(10).unwrap() as i8;
        }
    }

    let mut flashes = 0;

    for turn in 1..1000 {
        // First, the energy level of each octopus increases by 1.
        for y in 0..n {
            for x in 0..n {
                grid[y][x] += 1;
            }
        }

        loop {
            let (y, x) = find_flash(&grid);
            if y == n {
                break;
            }

            flashes += 1;

            //any octopus that flashed during this step has its energy level set to 0
            grid[y][x] = 0;

            // increases the energy level of all adjacent octopuses by 1
            for dy in -1..=1 {
                for dx in -1..=1 {
                    if dy == 0 && dx == 0 {
                        continue;
                    }
                    let nx = x as isize + dx;
                    let ny = y as isize + dy;
                    if 0 <= nx && nx < n as isize {
                        if 0 <= ny && ny < n as isize {
                            let v = grid[ny as usize][nx as usize];
                            if v != 0 && v <= 9 {
                                grid[ny as usize][nx as usize] = v + 1;
                            }
                        }
                    }
                }
            }
        }

        // for y in 0..n {
        //     for x in 0..n {
        //         print!("{}",grid[y][x]);
        //     }
        //     println!();
        // }

        if all_flashing(&grid) {
            println!("part2: {}", turn);
            break;
        }

        if turn == 100 {
            println!("part1: {}", flashes);
        }
    }
}

fn all_flashing(grid: &[Vec<i8>]) -> bool {
    let n = grid.len();
    for y in 0..n {
        for x in 0..n {
            if grid[y][x] != 0 {
                return false;
            }
        }
    }
    true
}

fn find_flash(grid: &[Vec<i8>]) -> (usize, usize) {
    let n = grid.len();
    for y in 0..n {
        for x in 0..n {
            if grid[y][x] == 10 {
                return (y, x);
            }
        }
    }
    (n, n)
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
