// Day 7: The Treachery of Whales
// https://adventofcode.com/2021/day/7

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

    let positions = data[0]
        .split(",")
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let mut min_sum1 = std::i32::MAX;
    let mut min_sum2 = std::i32::MAX;

    let mm = positions.iter().max().unwrap();
    for pos in 0..*mm {
        let mut sum1 = 0;
        let mut sum2 = 0;
        for q in &positions {
            let n = (q - pos).abs();

            sum1 += n;
            sum2 += n * (n + 1) / 2;
        }
        if sum1 < min_sum1 {
            min_sum1 = sum1;
        }

        if sum2 < min_sum2 {
            min_sum2 = sum2;
        }
    }

    println!("{}", min_sum1);
    println!("{}", min_sum2);
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
