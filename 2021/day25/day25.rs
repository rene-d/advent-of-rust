//! [Day 25: Sea Cucumber](https://adventofcode.com/2021/day/25)

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

    let nx = data[0].len();
    let ny = data.len();
    let mut grid = vec![vec!['.'; nx]; ny];

    // load the grid
    for (y, line) in data.iter().enumerate() {
        for (x, col) in line.chars().enumerate() {
            grid[y][x] = col;
        }
    }

    // move the sea cucumbers
    let mut step = 1;
    while do_move(&mut grid) {
        step += 1;
    }
    println!("{}", step);
}

fn do_move(grid: &mut [Vec<char>]) -> bool {
    let mut moved = false;

    let nx = grid[0].len();
    let ny = grid.len();

    // don't move blocked sea cucumbers
    for y in 0..ny {
        for x in 0..nx {
            if grid[y][x] == '>' && grid[y][x] == grid[y][(x + 1) % nx] {
                grid[y][x] = 'H';
            }
            if grid[y][x] == 'v' && grid[y][x] == grid[(y + 1) % ny][x] {
                grid[y][x] = 'V';
            }
        }
    }

    // During a single step, the east-facing herd moves first,
    for line in grid.iter_mut() {
        for x in 0..nx {
            if line[x] == '>' && line[(x + 1) % nx] == '.' {
                line[(x + 1) % nx] = 'H';
                line[x] = '.';
                moved = true;
            }
        }
    }

    // then the south-facing herd moves.
    for y in 0..ny {
        for x in 0..nx {
            let c = grid[y][x];
            if c == 'v' && grid[(y + 1) % ny][x] == '.' {
                grid[(y + 1) % ny][x] = 'V';
                grid[y][x] = '.';
                moved = true;
            }
        }
    }

    // restore blocked and moving sea cucumbers
    for line in grid {
        for val in line {
            match val {
                'H' => *val = '>',
                'V' => *val = 'v',
                _ => (),
            }
        }
    }

    // indicate if any sea cucumbers moved
    moved
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
