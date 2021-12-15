// Day 5: Hydrothermal Venture
// https://adventofcode.com/2021/day/5

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use structopt::StructOpt;
use regex::Regex;


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

        let drawn = re.captures(&line).unwrap();

        let mut x1 = drawn[1].parse::<i32>().unwrap();
        let mut y1 = drawn[2].parse::<i32>().unwrap();
        let mut x2 = drawn[3].parse::<i32>().unwrap();
        let mut y2 = drawn[4].parse::<i32>().unwrap();

        if x1 == x2 {
            if y1 > y2 {
                let tmp = y1;
                y1 = y2;
                y2 = tmp;
            }
            for y in y1..y2 + 1 {
                grid[x1 as usize][y as usize] += 1;
            }
        } else if y1 == y2 {
            if x1 > x2 {
                let tmp = x1;
                x1 = x2;
                x2 = tmp;
            }
            for x in x1..x2 + 1 {
                grid[x as usize][y1 as usize] += 1;
            }
        }
    }

    let mut sum = 0;
    for x in 0..1000 {
        for y in 0..1000 {
            if grid[x][y] > 1 {
                sum += 1;
            }
        }
    }
    println!("{:?}", sum);

    // --- Part Two ---
    for line in &data {

        let drawn = re.captures(&line).unwrap();

        let mut x1 = drawn[1].parse::<i32>().unwrap();
        let mut y1 = drawn[2].parse::<i32>().unwrap();
        let mut x2 = drawn[3].parse::<i32>().unwrap();
        let mut y2 = drawn[4].parse::<i32>().unwrap();

        if x1 != x2 && y1 != y2 {
            if x1 > x2 {
                let tmpx = x1;
                x1 = x2;
                x2 = tmpx;

                let tmpy = y1;
                y1 = y2;
                y2 = tmpy;
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
    for x in 0..1000 {
        for y in 0..1000 {
            if grid[x][y] > 1 {
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

        for line in lines {
            if let Ok(bits) = line {
                data.push(bits);
            }
        }
    }
    return data;
}
