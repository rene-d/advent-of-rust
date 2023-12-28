//! [Day 9: Smoke Basin](https://adventofcode.com/2021/day/9)

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

    // parse the data
    let sy = data.len();
    let sx = data[0].len();
    let mut grid = vec![vec![0; sx]; sy];

    for (y, line) in data.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            grid[y][x] = c.to_string().parse().unwrap();
        }
    }

    let mut basins = vec![];

    let mut risk = 0;
    for y in 0..sy {
        for x in 0..sx {
            let v = grid[y][x];
            if y > 0 && v >= grid[y - 1][x] {
                continue;
            }
            if x > 0 && v >= grid[y][x - 1] {
                continue;
            }
            if y < sy - 1 && v >= grid[y + 1][x] {
                continue;
            }
            if x < sx - 1 && v >= grid[y][x + 1] {
                continue;
            }
            risk += v + 1;

            basins.push(basin(&mut grid, y, x));
        }
    }

    // part 1
    println!("{risk}");

    // part 2
    if basins.len() >= 3 {
        basins.sort_by(|a, b| b.cmp(a));
        println!("{:?}", basins[0] * basins[1] * basins[2]);
    }
}

fn basin(grid: &mut Vec<Vec<i32>>, y: usize, x: usize) -> i32 {
    let mut stack = vec![(1, y, x)];
    let mut n = 0;
    while let Some((size, y, x)) = stack.pop() {
        if grid[y][x] == 9 {
            continue;
        }
        n += 1;
        grid[y][x] = 9;
        if y > 0 && grid[y - 1][x] != 0 {
            stack.push((size + 1, y - 1, x));
        }
        if x > 0 && grid[y][x - 1] != 0 {
            stack.push((size + 1, y, x - 1));
        }
        if y < grid.len() - 1 && grid[y + 1][x] != 0 {
            stack.push((size + 1, y + 1, x));
        }
        if x < grid[0].len() - 1 && grid[y][x + 1] != 0 {
            stack.push((size + 1, y, x + 1));
        }
    }
    n
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
