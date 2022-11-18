//! [Day 6: Lanternfish](https://adventofcode.com/2021/day/6)

use std::fs;
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

    let data = fs::read_to_string(args.path).unwrap();

    let mut timers = [0u64; 9];
    for timer in data.trim().split(',').map(|s| s.parse::<usize>().unwrap()) {
        timers[timer] += 1;
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
