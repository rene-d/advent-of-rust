//! [Day 11: Dumbo Octopus](https://adventofcode.com/2021/day/11)

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

    // read the grid
    let n = data.len();
    let mut grid = vec![vec![0u8; n]; n];
    for (y, line) in data.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            grid[y][x] = c.to_string().parse().unwrap();
        }
    }

    let mut flashes = 0;

    for turn in 1..1000 {
        // First, the energy level of each octopus increases by 1.
        for line in &mut grid {
            for val in line {
                *val += 1;
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
                    let nx = isize::try_from(x).unwrap() + dx;
                    let ny = isize::try_from(y).unwrap() + dy;
                    if 0 <= nx
                        && nx < n.try_into().unwrap()
                        && 0 <= ny
                        && ny < n.try_into().unwrap()
                    {
                        let nx = usize::try_from(nx).unwrap();
                        let ny = usize::try_from(ny).unwrap();
                        let v = grid[ny][nx];
                        if v != 0 && v <= 9 {
                            grid[ny][nx] = v + 1;
                        }
                    }
                }
            }
        }

        if all_flashing(&grid) {
            println!("{}", turn);
            break;
        }

        if turn == 100 {
            println!("{}", flashes);
        }
    }
}

fn all_flashing(grid: &[Vec<u8>]) -> bool {
    for line in grid {
        for val in line {
            if *val != 0 {
                return false;
            }
        }
    }
    true
}

fn find_flash(grid: &[Vec<u8>]) -> (usize, usize) {
    for (y, line) in grid.iter().enumerate() {
        for (x, val) in line.iter().enumerate() {
            if *val == 10 {
                return (y, x);
            }
        }
    }
    (grid.len(), grid.len())
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
