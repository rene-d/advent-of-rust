// Day 13: Transparent Origami
// https://adventofcode.com/2021/day/13

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

    let n = 2000;
    let mut grid = vec![vec![0i8; n]; n];
    let mut part1 = false;

    for line in &data {
        if line.is_empty() {
            break;
        }
        let (x, y) = line.split_once(",").unwrap();
        let xx = x.parse::<usize>().unwrap();
        let yy = y.parse::<usize>().unwrap();
        grid[yy][xx] = 1;
    }

    for line in &data {
        if !line.starts_with("fold") {
            continue;
        }

        if line.starts_with("fold along x=") {
            let (_, s) = line.split_once("=").unwrap();
            let fold = s.parse::<usize>().unwrap();
            for line in &mut grid {
                for x in 0..fold {
                    if line[fold + 1 + x] == 1 {
                        line[fold - 1 - x] = 1;
                    }
                }
            }
            for line in &mut grid {
                line.resize(fold, 0);
            }
        } else if line.starts_with("fold along y=") {
            let (_, s) = line.split_once("=").unwrap();
            let fold = s.parse::<usize>().unwrap();
            for y in 0..fold {
                for x in 0..grid[0].len() {
                    if grid[fold + 1 + y][x] == 1 {
                        grid[fold - 1 - y][x] = 1;
                    }
                }
            }
            grid.resize(fold, vec![0; grid[0].len()]);
        }

        if !part1 {
            part1 = true;
            let mut sum = 0;
            for row in &grid {
                for cell in row {
                    sum += *cell as u32;
                }
            }
            println!("part1: {}", sum);
        }
    }

    println!("part2:");
    for row in &grid {
        for cell in row {
            if *cell == 0 {
                print!(" ");
            } else {
                print!("#");
            }
        }
        println!();
    }
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
