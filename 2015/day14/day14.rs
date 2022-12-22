//! [Day 14: Reindeer Olympics](https://adventofcode.com/2015/day/14)

use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use structopt::StructOpt;

#[derive(Debug)]
struct Reinder {
    // name: String,
    speed: u32,
    duration: u32,
    rest: u32,
}

/// parse command line arguments
#[derive(StructOpt)]
struct Cli {
    #[structopt(default_value = "input.txt", parse(from_os_str))]
    path: std::path::PathBuf,

    #[structopt(default_value = "2503")]
    duration: u32,
}

/// main function
fn main() {
    let args = Cli::from_args();
    // println!("reading data from: {}", args.path.display());
    let data = load_data(args.path);

    let mut reinders = Vec::new();

    let re = Regex::new(
        r"(\w+) can fly (\d+) km/s for (\d+) seconds, but then must rest for (\d+) seconds.",
    )
    .unwrap();
    for line in &data {
        re.captures(line).map(|cap| {
            let reinder = Reinder {
                // name: cap[1].to_string(),
                speed: cap[2].parse().unwrap(),
                duration: cap[3].parse().unwrap(),
                rest: cap[4].parse().unwrap(),
            };
            // println!("{:?}", reinder);
            reinders.push(reinder);
            0
        });
    }

    // part 1
    let max_distance = reinders
        .iter()
        .map(|reinder: &Reinder| -> u32 {
            let mut seconds = args.duration;
            let mut distance = 0;
            while seconds >= reinder.duration + reinder.rest {
                seconds -= reinder.duration + reinder.rest;
                distance += reinder.speed * reinder.duration;
            }
            distance += reinder.speed * std::cmp::min(seconds, reinder.duration);
            // println!("{:10} -> {}", reinder.name, distance);
            distance
        })
        .max()
        .unwrap();
    println!("{}", max_distance);

    // part 2
    let mut scores: Vec<u32> = vec![0; reinders.len()];
    let mut distances: Vec<u32> = vec![0; reinders.len()];

    for elapsed in 1..args.duration {
        for i in 0..reinders.len() {
            let reinder = &reinders[i];

            let mut seconds = elapsed;
            let mut distance = 0;
            while seconds >= reinder.duration + reinder.rest {
                seconds -= reinder.duration + reinder.rest;
                distance += reinder.speed * reinder.duration;
            }
            distance += reinder.speed * std::cmp::min(seconds, reinder.duration);
            distances[i] = distance;

            // println!("{:4}: {:10} -> {:4} {:4}", elapsed, reinder.name, distance, scores[i]);
        }

        let distance_max = distances.iter().max().unwrap();
        for i in 0..reinders.len() {
            if distances[i] == *distance_max {
                scores[i] += 1;
            }
        }
    }
    println!("{:?}", scores.iter().max().unwrap());
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
