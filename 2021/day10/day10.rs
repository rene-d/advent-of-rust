//! [Day 10: Syntax Scoring](https://adventofcode.com/2021/day/10)

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

/// main function
fn main() {
    let filename = if let Some(x) = std::env::args().collect::<Vec<String>>().get(1) {
        x.clone()
    } else {
        "input.txt".to_string()
    };

    let data = load_data(filename);

    let mut part1 = 0;
    let mut part2 = vec![];

    for line in data {
        let (corrupted, completed) = check(&line);

        part1 += corrupted;
        if completed != 0 {
            part2.push(completed);
        }
    }

    // part2.sort_by(|a, b| a.cmp(b));
    part2.sort_unstable();

    println!("{}", part1);
    println!("{:?}", part2[part2.len() / 2]);
}

fn check(line: &str) -> (u64, u64) {
    let mut stack = vec![];

    for c in line.chars() {
        match c {
            '(' => stack.push(')'),
            '[' => stack.push(']'),
            '{' => stack.push('}'),
            '<' => stack.push('>'),
            _ => {
                let d = stack.pop().unwrap();
                if c != d {
                    match c {
                        ')' => return (3, 0),
                        ']' => return (57, 0),
                        '}' => return (1197, 0),
                        '>' => return (25137, 0),
                        _ => return (0, 0),
                    }
                }
            }
        }
    }

    let mut score = 0u64;
    while !stack.is_empty() {
        let d = stack.pop().unwrap();
        match d {
            ')' => score = score * 5 + 1,
            ']' => score = score * 5 + 2,
            '}' => score = score * 5 + 3,
            '>' => score = score * 5 + 4,
            _ => score *= 5,
        }
    }

    (0, score)
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
fn load_data<P>(path: P) -> Vec<String>
where
    P: AsRef<Path>,
{
    let mut data = vec![];
    if let Ok(lines) = read_lines(path) {
        for line in lines.flatten() {
            data.push(line);
        }
    }
    data
}
