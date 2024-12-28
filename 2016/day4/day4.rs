//! [Day 4: Security Through Obscurity](https://adventofcode.com/2016/day/4)

use lazy_static::lazy_static;
use regex::Regex;
use std::{collections::HashMap, convert::TryFrom};

lazy_static! {
    /// Regex that matches a line of input
    static ref RE_INPUT: Regex = Regex::new(r"([\w-]+)\-(\d+)\[(\w+)\]").unwrap();
}

/// ``main`` reads the puzzle input then solves part 1 and part 2
fn main() {
    let args = aoc::parse_args();
    let data = std::fs::read_to_string(args.path).unwrap();

    println!("{}", part1(&data));
    println!("{}", part2(&data));
}

/// ``part1`` returns the sum of sector id of valid rooms.
fn part1(data: &str) -> u32 {
    let mut sum = 0;

    for line in data.lines() {
        let (name, checksum, sector_id) = extract(line);

        if is_real_room(&name, &checksum) {
            sum += sector_id;
        }
    }

    sum
}

/// ``part2`` returns the sector id of the room with name ``northpole object``.
/// if no such room is found, returns ``0``.
fn part2(data: &str) -> u32 {
    for line in data.lines() {
        let (name, _, sector_id) = extract(line);

        //  Nota: the full decrypted name of the room is "northpole object storage"
        // and there is only one room with that name.
        if decrypt(&name, sector_id).contains("northpole object") {
            return sector_id;
        }
    }
    0
}

/// ``decrypt`` decrypts the name of a room given the sector id with the Caesar cipher.
fn decrypt(name: &str, sector_id: u32) -> String {
    let mut decrypted_name = String::new();

    for c in name.chars() {
        if c == '-' {
            decrypted_name.push(' ');
        } else {
            let c = (c as u32 - 'a' as u32 + sector_id) % 26 + 'a' as u32;
            decrypted_name.push(char::try_from(c).unwrap());
        }
    }

    decrypted_name
}

/// ``extract`` extracts the name, checksum and sector id from a line of data.
fn extract(line: &str) -> (String, String, u32) {
    let caps = RE_INPUT.captures(line).unwrap();

    let name = caps.get(1).unwrap().as_str().to_string();
    let sector_id = caps.get(2).unwrap().as_str().parse::<u32>().unwrap();
    let checksum = caps.get(3).unwrap().as_str().to_string();

    (name, checksum, sector_id)
}

/// ``is_real_room`` checks if the room is real by comparing the checksum.
fn is_real_room(name: &str, checksum: &str) -> bool {
    let mut freqs = HashMap::<char, i32>::new();

    for c in name.chars() {
        if c.is_ascii_lowercase() {
            *freqs.entry(c).or_default() += 1;
        }
    }

    let mut freqs = freqs.iter().map(|(c, n)| (-n, c)).collect::<Vec<_>>();
    freqs.sort_unstable();

    let check = freqs.iter().take(5).map(|(_, &c)| c).collect::<String>();

    return check == checksum;
}

#[cfg(test)]
#[test]
fn test_extract() {
    let (name, checksum, sector_id) = extract("aaaaa-bbb-z-y-x-123[abxyz]");
    assert_eq!(name, "aaaaa-bbb-z-y-x");
    assert_eq!(checksum, "abxyz");
    assert_eq!(sector_id, 123);

    let (name, checksum, sector_id) = extract("a-b-c-d-e-f-g-h-987[abcde]");
    assert_eq!(name, "a-b-c-d-e-f-g-h");
    assert_eq!(checksum, "abcde");
    assert_eq!(sector_id, 987);

    let (name, checksum, sector_id) = extract("not-a-real-room-404[oarel]");
    assert_eq!(name, "not-a-real-room");
    assert_eq!(checksum, "oarel");
    assert_eq!(sector_id, 404);

    let (name, checksum, sector_id) = extract("totally-real-room-200[decoy]");
    assert_eq!(name, "totally-real-room");
    assert_eq!(checksum, "decoy");
    assert_eq!(sector_id, 200);
}

#[test]
fn test_is_real_room() {
    assert!(is_real_room("aaaaa-bbb-z-y-x", "abxyz"));
    assert!(is_real_room("a-b-c-d-e-f-g-h", "abcde"));
    assert!(is_real_room("not-a-real-room", "oarel"));
    assert!(!is_real_room("totally-real-room", "decoy"));
}

#[test]
fn test_part1() {
    let data = "aaaaa-bbb-z-y-x-123[abxyz]
a-b-c-d-e-f-g-h-987[abcde]
not-a-real-room-404[oarel]
totally-real-room-200[decoy]";

    assert_eq!(part1(data), 1514);
}

#[test]
fn test_decrypt() {
    assert_eq!(decrypt("qzmt-zixmtkozy-ivhz", 343), "very encrypted name");
}
