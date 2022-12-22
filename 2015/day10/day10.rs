//! [Day 10: Elves Look, Elves Say](https://adventofcode.com/2015/day/10)

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

    solve(&data[0], 40);
    solve(&data[0], 50);
}

fn solve(start_sequence: &str, turns: u32) {
    let mut look = start_sequence.chars().collect::<Vec<char>>();

    for _ in 0..turns {
        let mut say: Vec<char> = Vec::new();

        let mut count = 0;
        let mut previous = '\0';
        for current in look {
            if previous != '\0' && previous != current {
                say.extend(count.to_string().chars());
                say.push(previous);
                count = 0;
            }
            count += 1;
            previous = current;
        }

        say.extend(count.to_string().chars());
        say.push(previous);

        look = say.clone();
    }
    println!("{}", look.len());
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
