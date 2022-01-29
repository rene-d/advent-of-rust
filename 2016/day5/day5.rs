/*!
[Day 5: How About a Nice Game of Chess?](https://adventofcode.com/2016/day/5)

You are faced with a security door designed by Easter Bunny engineers
that seem to have acquired most of their security knowledge by watching
[hacking movies](https://en.wikipedia.org/wiki/WarGames).

The **eight-character password** for the door is generated one character at a
time by finding the [MD5](https://en.wikipedia.org/wiki/MD5) hash of some Door ID
(your puzzle input) and an increasing integer index (starting with `0`).

A hash indicates the *next character* in the password if its
[hexadecimal](https://en.wikipedia.org/wiki/Hexadecimal) representation starts
with **five zeroes**. If it does, the sixth character in the hash is the next
character of the password.

For example, if the Door ID is `abc`:

- The first index which produces a hash that starts with five zeroes is `3231929`,
  which we find by hashing `abc3231929`; the sixth character of the hash, and thus
  the first character of the password, is `1`.
- `5017308` produces the next interesting hash, which starts with `000008f82...`,
  so the second character of the password is `8`.
- The third time a hash starts with five zeroes is for `abc5278568`, discovering
  the character `f`.

In this example, after continuing this search a total of eight times, the password
is `18f47a30`.

Given the actual Door ID, **what is the password**?

--- Part Two ---

As the door slides open, you are presented with a second door that uses a
slightly more inspired security mechanism. Clearly unimpressed by the
last version (in what movie is the password decrypted in **order**?!), the
Easter Bunny engineers have worked out a
[better solution](https://www.youtube.com/watch?v=NHWjlCaIrQo&t=25).

Instead of simply filling in the password from left to right, the hash
now also indicates the **position** within the password to fill. You still
look for hashes that begin with five zeroes; however, now, the **sixth**
character represents the position (0-7), and the **seventh** character is
the character to put in that position.

A hash result of `000001f` means that `f` is the **second** character in
the password. Use only the **first result** for each position, and ignore
invalid positions.

For example, if the Door ID is `abc`:

- The first interesting hash is from `abc3231929`, which produces
  `0000015...`; so, `5` goes in position `1`: `_5______`.
- In the previous method, `5017308` produced an interesting hash; however,
  it is ignored, because it specifies an invalid position (`8`).
- The second interesting hash is at index `5357525`, which produces
  `000004e...`; so, `e` goes in position `4`: `_5__e___`.

  You almost choke on your popcorn as the final character falls into place,
  producing the password `05ace8e3`.

Given the actual Door ID and this new method, **what is the password**?
Be extra proud of your solution if it uses a cinematic "decrypting" animation.
*/

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
