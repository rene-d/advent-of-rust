//! [Day 8: Matchsticks](https://adventofcode.com/2015/day/8)

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

    let mut total_raw = 0;
    let mut total_decoded = 0;
    let mut total_encoded = 0;

    for line in &data {
        assert_eq!(line.chars().next().unwrap(), '"');
        assert_eq!(line.chars().last().unwrap(), '"');

        let mut len_decoded = line.len() - 2;
        let mut len_encoded = line.len() + 4;

        let mut escape_char = false;
        for c in line.chars().skip(1).take(line.len() - 2) {
            if escape_char {
                escape_char = false;
                if c == 'x' {
                    len_decoded -= 2; // remove the two digits
                }
            } else if c == '\\' {
                escape_char = true;
                len_decoded -= 1; // remove the escape char from the length
            }

            if c == '\\' {
                len_encoded += 1; // escape the backslash
            }
            if c == '"' {
                len_encoded += 1; // escape the double quote
            }
        }

        total_decoded += len_decoded;
        total_encoded += len_encoded;
        total_raw += line.len();
    }

    println!("{}", total_raw - total_decoded);
    println!("{}", total_encoded - total_raw);
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
