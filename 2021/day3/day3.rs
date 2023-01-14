//! [Day 3: Binary Diagnostic](https://adventofcode.com/2021/day/3)

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

    println!("{}", part1(&data));
    println!("{}", part2(&data));
}

/// step 2
fn part2(data: &[String]) -> isize {
    let nb_bits = data[0].len();
    // println!("nb_bits: {}", nb_bits);

    // oxygen generator rating
    let mut dioxygen_rate = 0;
    let mut dioxygen_start = String::new();

    for bit in 0..nb_bits {
        let mut one = 0;
        let mut nb = 0;

        for value in data.iter() {
            if value.starts_with(&dioxygen_start) {
                let c = value.chars().nth(bit).unwrap();
                if c == '1' {
                    one += 1;
                    dioxygen_rate = isize::from_str_radix(value, 2).unwrap();
                }
                nb += 1;
            }
        }

        if one >= nb - one {
            dioxygen_start.push('1');
        } else {
            dioxygen_start.push('0');
        }
    }

    // CO2 scrubber rating
    let mut co2_rate = 0;
    let mut co2_start = String::new();

    for bit in 0..nb_bits {
        let mut one = 0;
        let mut nb = 0;

        for value in data.iter() {
            if value.starts_with(&co2_start) {
                let c = value.chars().nth(bit).unwrap();
                if c == '1' {
                    one += 1;
                } else {
                    co2_rate = isize::from_str_radix(value, 2).unwrap();
                }
                nb += 1;
            }
        }

        if one >= nb - one {
            co2_start.push('0');
        } else {
            co2_start.push('1');
        }
    }
    dioxygen_rate * co2_rate
}

/// step 1: compute `gamma_rate` * `espilon_rate`
fn part1(data: &[String]) -> i32 {
    let mut gamma_rate = 0;
    let mut freq_list: [i32; 12] = [0; 12];
    let mut nb = 0;

    let width = data.first().unwrap().len();
    let mask = (1 << width) - 1;

    for bits in data {
        for (i, bit) in bits.chars().enumerate() {
            assert!(i < width);
            if bit == '1' {
                freq_list[i] += 1_i32;
            }
        }
        nb += 1;
    }

    for freq in freq_list.iter().take(width) {
        gamma_rate *= 2;
        if *freq >= nb / 2 {
            gamma_rate += 1;
        }
    }

    let espilon_rate = mask - gamma_rate;

    gamma_rate * espilon_rate
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
        for line in lines.flatten() {
            data.push(line);
        }
    }
    data
}

#[test]
fn test_part1() {
    let data = load_data("test.txt".into());

    assert_eq!(part1(&data), 198);
}

#[test]
fn test_part2() {
    let data = load_data("test.txt".into());

    assert_eq!(part2(&data), 230);
}
