// Day 6: Lanternfish
// https://adventofcode.com/2021/day/6

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

    let mut timers = [0u64; 9];
    for timer in data[0].split(",").map(|s| s.parse::<u64>().unwrap()) {
        timers[timer as usize] += 1;
    }

    lanterfishes(&timers, 80);
    lanterfishes(&timers, 256);
}

fn lanterfishes(initial: &[u64; 9], days: u64) {
    let mut timers = *initial;

    for _day in 0..days {
        let mut new = [0u64; 9];
        for (i, timer) in timers.iter().enumerate() {
            if i == 0 {
                new[6] += *timer;
                new[8] += *timer;
            } else {
                new[(i - 1) as usize] += *timer;
            }
        }
        timers = new;
    }

    println!("{}", timers.iter().sum::<u64>());
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
        for line in lines {
            if let Ok(bits) = line {
                data.push(bits);
            }
        }
    }
    data
}
