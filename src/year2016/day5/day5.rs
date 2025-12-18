//! [Day 5: How About a Nice Game of Chess?](https://adventofcode.com/2016/day/5)

use rayon::prelude::*;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

const HEX_DIGITS: &[char] = &[
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f',
];

const CHUNK_SIZE: u32 = 10_000;

pub fn main() {
    let args = aoc::parse_args();
    args.run(solve);
}

/// Compute MD5 hash for a given `door_id` and index
#[inline]
fn compute_hash(door_id: &[u8], index: u32) -> [u8; 16] {
    let mut hash = [0u8; 32];
    let index_pos = door_id.len();

    // Copy door_id
    hash[..index_pos].copy_from_slice(door_id);

    // Manually append index digits to avoid format! overhead
    let mut hash_size = index_pos;
    let mut index_temp = index;

    // Count digits
    loop {
        hash_size += 1;
        index_temp /= 10;
        if index_temp == 0 {
            break;
        }
    }

    // Write digits from right to left
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

    // Compute MD5
    *md5::compute(&hash[..hash_size])
}

/// Process a chunk of indices and return found characters
fn process_chunk(
    door_id: &[u8],
    start_index: u32,
    end_index: u32,
    part1_complete: &AtomicBool,
    part2_complete: &AtomicBool,
) -> Vec<(u32, u8, u8, u8)> {
    let mut results = Vec::new();

    for index in start_index..end_index {
        // Early termination if both parts are complete
        if part1_complete.load(Ordering::Relaxed) && part2_complete.load(Ordering::Relaxed) {
            break;
        }

        let digest = compute_hash(door_id, index);

        // Check if first 5 hex digits are zero
        if digest[0] == 0 && digest[1] == 0 && (digest[2] & 0xF0) == 0 {
            let sixth = digest[2] & 0x0F;
            let seventh = digest[3] >> 4;
            results.push((index, sixth, seventh, digest[3] & 0x0F));
        }
    }

    results
}

#[must_use]
pub fn solve(data: &str) -> (String, String) {
    let door_id = data.trim_ascii();
    let door_id_bytes = door_id.as_bytes();

    let mut password_part1 = ['_'; 8];
    let mut password_part2 = ['_'; 8];

    let mut found_part1 = 0;
    let mut found_part2 = 0;

    let part1_complete = Arc::new(AtomicBool::new(false));
    let part2_complete = Arc::new(AtomicBool::new(false));

    let mut chunk_start = 0u32;

    while found_part1 < 8 || found_part2 < 8 {
        let chunk_end = chunk_start + CHUNK_SIZE;

        // Process chunks in parallel
        let results: Vec<_> = (chunk_start..chunk_end)
            .into_par_iter()
            .step_by(1000)
            .flat_map(|start| {
                let end = (start + 1000).min(chunk_end);
                process_chunk(door_id_bytes, start, end, &part1_complete, &part2_complete)
            })
            .collect();

        // Sort results by index to maintain order
        let mut sorted_results = results;
        sorted_results.sort_by_key(|r| r.0);

        // Process results
        for (_index, sixth, seventh, _eighth) in sorted_results {
            // Part 1
            if found_part1 < 8 {
                password_part1[found_part1] = HEX_DIGITS[sixth as usize];
                found_part1 += 1;
                if found_part1 == 8 {
                    part1_complete.store(true, Ordering::Relaxed);
                }
            }

            // Part 2
            if found_part2 < 8 {
                let position = sixth;
                if position < 8 && password_part2[position as usize] == '_' {
                    password_part2[position as usize] = HEX_DIGITS[seventh as usize];
                    found_part2 += 1;
                    if found_part2 == 8 {
                        part2_complete.store(true, Ordering::Relaxed);
                    }
                }
            }

            // Early exit if both complete
            if found_part1 >= 8 && found_part2 >= 8 {
                break;
            }
        }

        chunk_start = chunk_end;
    }

    (
        password_part1.iter().collect::<String>(),
        password_part2.iter().collect::<String>(),
    )
}

#[cfg(not(debug_assertions))]
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solve() {
        let r = solve("abc");

        assert_eq!(r.0, "18f47a30");
        assert_eq!(r.1, "05ace8e3");
    }
}
