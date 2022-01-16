// Day 4: Giant Squid
// https://adventofcode.com/2021/day/4

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

    let drawn = data[0]
        .split(',')
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    // load the grids
    let mut grids = vec![];
    let mut i = 2;
    while i < data.len() {
        let mut grid = [[0i32; 5]; 5];

        for y in 0..5 {
            let line = data[i + y]
                .split_whitespace()
                .filter_map(|s| s.parse::<i32>().ok())
                .collect::<Vec<i32>>();

            grid[y][..5].clone_from_slice(&line[..5]);
        }

        grids.push(grid);

        i += 6;
    }

    let mut first_win = false;
    let mut last_draw = 0;

    for draw in drawn {
        for grid in grids.iter_mut() {
            if grid[0][0] == -2 {
                // grid invalidated
                continue;
            }

            for line in grid.iter_mut() {
                for val in line {
                    if *val == draw {
                        *val = -1; // clear the case
                    }
                }
            }

            if win(grid) {
                last_draw = draw * sum(grid);
                if !first_win {
                    first_win = true;
                    println!("{}", last_draw);
                }
                grid[0][0] = -2; // invalidate the grid
            }
        }
    }

    println!("{}", last_draw);
}

/// sum computes the sum of non-cleared cases
fn sum(grid: &[[i32; 5]; 5]) -> i32 {
    let mut s = 0;
    for line in grid {
        for val in line {
            if *val != -1 {
                s += *val;
            }
        }
    }
    s
}

/// has_win returns true if the grid has an cleared row or column
fn win(grid: &[[i32; 5]; 5]) -> bool {
    for i in 0..5 {
        if grid[i][0] == -1
            && grid[i][1] == -1
            && grid[i][2] == -1
            && grid[i][3] == -1
            && grid[i][4] == -1
        {
            return true;
        }

        if grid[0][i] == -1
            && grid[1][i] == -1
            && grid[2][i] == -1
            && grid[3][i] == -1
            && grid[4][i] == -1
        {
            return true;
        }
    }

    false
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
