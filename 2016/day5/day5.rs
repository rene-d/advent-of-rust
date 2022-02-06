//! [Day 5: How About a Nice Game of Chess?](https://adventofcode.com/2016/day/5)

use num_traits::cast::FromPrimitive;
use std::time::Instant;

use indicatif::{ProgressBar, ProgressStyle};

const TICK_CHARS: &str = r"⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏ "; // r"⠁⠂⠄⡀⢀⠠⠐⠈ "

/// ``main`` reads the puzzle input then solves part 1 and part 2
fn main() {
    let door_id = std::fs::read_to_string("input.txt").unwrap();
    let door_id = door_id.trim();

    let now = Instant::now();

    println!("{}", part1(door_id));
    println!("{}", part2(door_id));

    let micros = f64::from_u128(now.elapsed().as_micros()).unwrap();
    println!("elapsed: {} s", micros / 1_000_000.);
}

/// ``part1`` solves part 1 of the puzzle
fn part1(door_id: &str) -> String {
    let mut password = ['_'; 8];
    let mut found = 0;

    let pb = ProgressBar::new_spinner();
    pb.enable_steady_tick(200);
    pb.set_style(
        ProgressStyle::default_spinner()
            .tick_chars(TICK_CHARS)
            .template("  {prefix:.bold.dim} {spinner} {wide_msg}"),
    );
    pb.set_prefix("cracking password");

    let mut index = 0;
    loop {
        let hash = format!("{}{}", door_id, index);
        let digest = md5::compute(hash);
        let hex = format!("{:x}", digest);

        if hex.starts_with("00000") {
            password[found] = hex.chars().nth(6).unwrap();
            found += 1;
            if found == 8 {
                break;
            }
        }
        index += 1;

        if index % 1000 == 0 {
            pb.set_message(password.iter().collect::<String>());
            pb.tick();
        }
    }

    pb.finish_and_clear();

    password.iter().collect::<String>()
}

/// ``part2`` solves part 2 of the puzzle
fn part2(door_id: &str) -> String {
    let mut password = ['_'; 8];
    let mut remaining = 8;

    let pb = ProgressBar::new_spinner();
    pb.enable_steady_tick(200);
    pb.set_style(
        ProgressStyle::default_spinner()
            .tick_chars(TICK_CHARS)
            .template("  {prefix:.bold.dim} {spinner} {wide_msg}"),
    );
    pb.set_prefix("cracking password");

    let mut index = 0;
    loop {
        let hash = format!("{}{}", door_id, index);
        let digest = md5::compute(hash);
        let hex = format!("{:x}", digest);

        if hex.starts_with("00000") {
            if let Some(position) = hex.chars().nth(5).unwrap().to_digit(10) {
                if position < 8 && password[position as usize] == '_' {
                    password[position as usize] = hex.chars().nth(6).unwrap();
                    remaining -= 1;
                    if remaining == 0 {
                        break;
                    }
                }
            }
        }
        index += 1;

        if index % 1000 == 0 {
            pb.set_message(password.iter().collect::<String>());
            pb.tick();
        }
    }

    pb.finish_and_clear();

    password.iter().collect()
}

#[cfg(test)]
#[test]
fn test_part1() {
    // assert_eq!(part1("abc"), "18f47a30");
}

#[test]
fn test_part2() {
    // assert_eq!(part1("abc"), "05ace8e3");
}
