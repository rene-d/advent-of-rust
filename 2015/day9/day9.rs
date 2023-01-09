//! [Day 9: All in a Single Night](https://adventofcode.com/2015/day/9)

use permutator::HeapPermutationIterator;
use regex::Regex;
use std::collections::{HashMap, HashSet};
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

    let mut places: HashSet<String> = HashSet::new();
    let mut distances: HashMap<(String, String), u32> = HashMap::new();

    let re = Regex::new(r"^(.+) to (.+) = (\d+)$").unwrap();

    for line in &data {
        if let Some(op) = re.captures(line) {
            places.insert(op[1].to_string());
            places.insert(op[2].to_string());

            distances.insert(
                (op[1].to_string(), op[2].to_string()),
                op[3].parse().unwrap(),
            );
            distances.insert(
                (op[2].to_string(), op[1].to_string()),
                op[3].parse().unwrap(),
            );
        }
    }

    let perm_places = &mut places.iter().collect::<Vec<&String>>();
    let permutator = HeapPermutationIterator::new(perm_places);

    let mut min_distance = std::u32::MAX;
    let mut max_distance = std::u32::MIN;

    for permutated in permutator {
        let mut distance = 0;
        for i in 0..permutated.len() - 1 {
            let from = permutated[i];
            let to = permutated[i + 1];

            distance += distances.get(&(from.to_string(), to.to_string())).unwrap();
        }

        // println!("{:?}  ->  {}", permutated, distance);

        if distance < min_distance {
            min_distance = distance;
        }
        if distance > max_distance {
            max_distance = distance;
        }
    }

    println!("{}", min_distance);
    println!("{}", max_distance);
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
