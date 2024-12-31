//! [Day 5: How About a Nice Game of Chess?](https://adventofcode.com/2016/day/5)

use indicatif::{ProgressBar, ProgressStyle};
use std::time::{Duration, Instant};

const HEX_DIGITS: &[char] = &[
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f',
];

const TICK_CHARS: &str = "\u{280b}\u{2819}\u{2839}\u{2838}\u{283c}\u{2834}\u{2826}\u{2827}\u{2807}\u{280f} ";

/// ``main`` reads the puzzle input then solves part 1 and part 2
fn main() {
    let args = aoc::parse_args();

    let door_id = args.input.trim();

    let mut password_part1 = ['_'; 8];
    let mut password_part2 = ['_'; 8];

    let pb = ProgressBar::new_spinner();
    if args.verbose {
        pb.enable_steady_tick(Duration::from_millis(200));
        pb.set_style(
            ProgressStyle::default_spinner()
                .tick_chars(TICK_CHARS)
                .template("  {prefix:.bold.dim} {spinner} {wide_msg}")
                .unwrap(),
        );
        pb.set_prefix("cracking password");
    }

    let now = Instant::now();

    // prepare a byte buffer to store the `door_id` and the `index``
    let mut hash = [0u8; 32];
    let index_pos = door_id.len();
    for (i, c) in door_id.chars().enumerate() {
        hash[i] = c as u8;
    }

    // let's crack the passwords
    let mut found = 0;
    let mut remaining: i32 = 8;
    let mut index: u32 = 0;

    while found != 8 || remaining != 0 {
        //

        // manually do « hash = format!("{door_id}{index}"); »
        // since format! is very slow

        // the length of the hash: "{door_id}{index}"
        let mut hash_size = index_pos;

        // first, loop to count the digits of `index`
        let mut index_temp = index;
        loop {
            hash_size += 1;
            index_temp /= 10;
            if index_temp == 0 {
                break;
            }
        }

        // second, loop to write the digits of `index`, from right to left
        index_temp = index;
        let mut index_digit_pos = hash_size;
        loop {
            index_digit_pos -= 1;
            hash[index_digit_pos] = b'0' + (index_temp % 10) as u8;
            index_temp /= 10;
            if index_temp == 0 {
                break;
            }
        }

        // compute hash
        let digest = md5::compute(&hash[..hash_size]);

        // test the 5 first hex digits of the digest
        if digest[0] == 0 && digest[1] == 0 && (digest[2] & 0xF0) == 0 {
            // part 1
            if found != 8 {
                password_part1[found] = HEX_DIGITS[(digest[2] & 0x0F) as usize];
                found += 1;
            }

            // part 2
            if remaining != 0 {
                // 6th digit of the digest
                let position = digest[2] & 0x0F;

                if position < 8 && password_part2[position as usize] == '_' {
                    password_part2[position as usize] = HEX_DIGITS[(digest[3] >> 4) as usize];
                    remaining -= 1;
                }
            }
        }
        index += 1;

        if args.verbose && index % 10000 == 0 {
            pb.set_message(format!(
                "{} {}",
                password_part1.iter().collect::<String>(),
                password_part2.iter().collect::<String>()
            ));
        }
    }

    if args.verbose {
        pb.finish_and_clear();
    }

    println!("{}", password_part1.iter().collect::<String>());
    println!("{}", password_part2.iter().collect::<String>());

    if args.verbose {
        let micros = now.elapsed().as_micros();
        eprintln!("elapsed: {}.{:03} ms", micros / 1000, micros % 1000);
    }
}

#[cfg(test)]
#[test]
fn test_part1() {
    // assert_eq!(part1("abc"), "18f47a30");
}

#[test]
fn test_part2() {
    // assert_eq!(part2("abc"), "05ace8e3");
}
