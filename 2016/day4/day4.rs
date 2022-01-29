// Day 4: Security Through Obscurity
// https://adventofcode.com/2016/day/4

// cargo rustdoc --open -- --no-defaults --passes collapse-docs --passes unindent-comments --passes strip-priv-imports

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    /// Regex that mtches a line of input
    static ref RE: Regex = Regex::new(r"([\w-]+)\-(\d+)\[(\w+)\]").unwrap();
}

fn main() {
    let data = std::fs::read_to_string("input.txt").unwrap();

    println!("{}", part1(&data));
    println!("{}", part2(&data));
}

/// ``part1`` returns the sum of sector id of valid rooms.
fn part1(data: &str) -> u32 {
    let mut sum = 0;

    for line in data.split('\n') {
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
    for line in data.split('\n') {
        let (name, checksum, sector_id) = extract(line);

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
            let c = ((c as u32 - 'a' as u32 + sector_id) % 26 + 'a' as u32) as u8;
            decrypted_name.push(c as char);
        }
    }

    decrypted_name
}

/// ``extract`` extracts the name, checksum and sector id from a line of data.
fn extract(line: &str) -> (String, String, u32) {
    let caps = RE.captures(line).unwrap();

    let name = caps.get(1).unwrap().as_str().to_string();
    let sector_id = caps.get(2).unwrap().as_str().parse::<u32>().unwrap();
    let checksum = caps.get(3).unwrap().as_str().to_string();

    (name, checksum, sector_id)
}

/// ``is_real_room`` checks if the room is real by comparing the checksum.
fn is_real_room(name: &str, checksum: &str) -> bool {
    let mut counts = [0; 26];
    for c in name.chars() {
        if ('a'..='z').contains(&c) {
            counts[(c as u8 - b'a') as usize] += 1;
        }
    }

    let mut current = 0;
    let max = counts.iter().max().unwrap();
    let mut current_max = *max;

    for _ in 0..5 {
        let mut next_max = 0;
        let mut max_found = false;

        for (c, count) in counts.iter().enumerate() {
            if *count == current_max {
                let letter = (b'a' + c as u8) as char;
                if checksum.chars().nth(current).unwrap() == letter {
                    current += 1;
                    if current == 5 {
                        return true;
                    }
                    max_found = true;
                }
            } else if *count < current_max && *count > next_max {
                next_max = *count;
            }
        }

        if !max_found {
            break;
        }

        current_max = next_max;
    }

    false
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
    assert_eq!(is_real_room("aaaaa-bbb-z-y-x", "abxyz"), true);
    assert_eq!(is_real_room("a-b-c-d-e-f-g-h", "abcde"), true);
    assert_eq!(is_real_room("not-a-real-room", "oarel"), true);
    assert_eq!(is_real_room("totally-real-room", "decoy"), false);
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
