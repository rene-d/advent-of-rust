// Day 8: Seven Segment Search
// https://adventofcode.com/2021/day/8

use std::collections::HashMap;
use std::collections::HashSet;
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

    part1(&data);
    part2(&data);
}

fn contains(a: &str, b: &str) -> bool {
    for c in b.chars() {
        if !a.contains(c) {
            return false;
        }
    }
    true
}

fn part2(data: &[String]) {
    // Nota: the Python version is much more readable...
    let mut total = 0;
    for line in data {
        let (notes1, code1) = line.split_once('|').unwrap();

        let notes = notes1.split_whitespace().collect::<Vec<&str>>();
        let code = code1.split_whitespace().collect::<Vec<&str>>();

        let mut d: HashMap<usize, HashSet<String>> = HashMap::new();

        for c in notes {
            let mut s: Vec<char> = c.chars().collect();
            s.sort_unstable();
            let s_sorted = String::from_iter(s);
            d.entry(c.len())
                .or_insert_with(HashSet::new)
                .insert(s_sorted);
        }

        let mut map: HashMap<String, u8> = HashMap::new();

        let mut zero: &str = "";
        let one = d.get(&2).unwrap().iter().next().unwrap(); // the only digit with the 2 segments
        let mut two: &str = "";
        let mut three: &str = "";
        let four = d.get(&4).unwrap().iter().next().unwrap(); // the only digit with the 4 segments
        let mut five: &str = "";
        let mut six: &str = "";
        let seven = d.get(&3).unwrap().iter().next().unwrap(); // the only digit with the 3 segments
        let eight = d.get(&7).unwrap().iter().next().unwrap(); // the only digit with 7 segments
        let mut nine: &str = "";

        for x in d.get(&6).unwrap().iter() {
            // could be 0 6 9
            if contains(x, one) {
                // si on n'a pas les deux segments du 1
                if contains(x, four) {
                    // si 4 est inclus, c'est forcément un 9
                    nine = x;
                } else {
                    // sinon, c'est forcément un 0
                    zero = x;
                }
            } else {
                // sinon, c'est forcément un 6
                six = x;
            }
        }

        for x in d.get(&5).unwrap().iter() {
            // 2 3 5
            if contains(x, seven) {
                // si 7 est inclus, c est forcément un 3
                three = x;
            } else if contains(nine, x) {
                // si c'est inclus dans 9, c'est forcément un 5
                five = x;
            } else {
                // sinon, c'est forcément un 2
                two = x;
            }
        }

        map.insert(zero.to_string(), 0);
        map.insert(one.to_string(), 1);
        map.insert(two.to_string(), 2);
        map.insert(three.to_string(), 3);
        map.insert(four.to_string(), 4);
        map.insert(five.to_string(), 5);
        map.insert(six.to_string(), 6);
        map.insert(seven.to_string(), 7);
        map.insert(eight.to_string(), 8);
        map.insert(nine.to_string(), 9);

        let mut r = 0;
        for d in code {
            let mut s: Vec<char> = d.chars().collect();
            s.sort_unstable();
            let s_sorted = String::from_iter(s);

            let v = u32::from(*map.entry(s_sorted).or_insert(0));
            r = r * 10 + v;
        }
        total += r;
    }
    println!("{:?}", total);
}

fn part1(data: &[String]) {
    let mut digit_one = 0;
    let mut digit_four = 0;
    let mut digit_seven = 0;
    let mut digit_eight = 0;

    for line in data {
        let x = line.split_once('|').unwrap().1.trim();
        for c in x.split_whitespace() {
            match c.len() {
                2 => digit_one += 1,
                3 => digit_seven += 1,
                4 => digit_four += 1,
                7 => digit_eight += 1,
                _ => (),
            }
        }
    }

    println!("{}", digit_one + digit_four + digit_seven + digit_eight);
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

#[cfg(test)]
#[test]
fn test_contains() {
    assert_eq!(contains("abcde", "ab"), true);
    assert_eq!(contains("abcde", "ba"), true);
    assert_eq!(contains("abcde", "abcde"), true);
    assert_eq!(contains("abcde", "az"), false);
    assert_eq!(contains("abcde", "ef"), false);
}
