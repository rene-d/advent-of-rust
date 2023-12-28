//! [Day 1: Not Quite Lisp](https://adventofcode.com/2015/day/1)

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

    for line in data {
        let mut floor = 0;
        let mut position = 0;
        let mut enter = 0;
        for c in line.chars() {
            position += 1;
            match c {
                '(' => floor += 1,
                ')' => floor -= 1,
                _ => panic!("invalid character"),
            }
            if floor == -1 && enter == 0 {
                enter = position;
            }
        }
        println!("{floor}");
        println!("{enter}");
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
