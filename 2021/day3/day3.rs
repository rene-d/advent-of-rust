// --- Day 3: Binary Diagnostic ---
// https://adventofcode.com/2021/day/3

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

    // println!("{:?}", data);
    step1(&data);
    step2(&data);
}

/// step 2
fn step2(data: &Vec<String>) {
    let nb_bits = data[0].len();
    // println!("nb_bits: {}", nb_bits);

    // oxygen generator rating
    let mut o2_rate = 0;
    let mut o2_start = "".to_owned();

    for bit in 0..nb_bits {
        let mut one = 0;
        let mut nb = 0;

        for value in data.iter() {
            if value.starts_with(&o2_start) {
                let c = value.chars().nth(bit).unwrap();
                if c == '1' {
                    one += 1;
                    o2_rate = isize::from_str_radix(&value, 2).unwrap();
                }
                nb += 1;
            }
        }

        if one >= nb - one {
            o2_start.push_str("1");
        } else {
            o2_start.push_str("0");
        }
    }

    // CO2 scrubber rating
    let mut co2_rate = 0;
    let mut co2_start = "".to_owned();

    for bit in 0..nb_bits {
        let mut one = 0;
        let mut nb = 0;

        for value in data.iter() {
            if value.starts_with(&co2_start) {
                let c = value.chars().nth(bit).unwrap();
                if c == '1' {
                    one += 1;
                    co2_rate = isize::from_str_radix(&value, 2).unwrap();
                }
                nb += 1;
            }
        }

        if one >= nb - one {
            co2_start.push_str("0");
        } else {
            co2_start.push_str("1");
        }
    }
    println!("{}", o2_rate * co2_rate);
}

/// step 1: compute gamma_rate * espilon_rate
fn step1(data: &Vec<String>) {
    let mut gamma_rate = 0;
    let mut freq: [i32; 12] = [0; 12];
    let mut nb = 0;

    for bits in data {
        for (i, bit) in bits.chars().enumerate() {
            if bit == '1' {
                freq[i] += 1i32;
            }
        }
        nb += 1;
    }

    for i in 0..12 {
        gamma_rate = gamma_rate * 2;
        if freq[i] >= nb / 2 {
            gamma_rate += 1;
        }
    }

    let espilon_rate = 0b111111111111 - gamma_rate;

    println!("{}", gamma_rate * espilon_rate);
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
