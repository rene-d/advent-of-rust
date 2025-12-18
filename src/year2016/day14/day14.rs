//! [Day 14: One-Time Pad](https://adventofcode.com/2016/day/14)

use rayon::prelude::*;

/// Solve the day 14 puzzle.
pub fn main() {
    let args = aoc::parse_args();
    args.run(solve);
}

/// # Panics
#[must_use]
pub fn solve(data: &str) -> (usize, usize) {
    (find_key(data, 0), find_key(data, 2016))
}

const HEX_DIGITS: [u8; 16] = [
    b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'a', b'b', b'c', b'd', b'e', b'f',
];

#[derive(Clone, Copy)]
struct HashData {
    triplet: Option<u8>, // 0-15 value
    quintuplets: u16,    // bitmask
}

fn compute_hash(salt: &[u8], index: usize, stretching: usize) -> HashData {
    // Construct the input string: salt + index
    // We avoid allocations where reasonable, though MD5 dominates cost.
    let mut engine = md5::Context::new();
    engine.consume(salt);
    engine.consume(index.to_string());
    let mut digest = engine.finalize();

    for _ in 0..stretching {
        let mut hex = [0u8; 32];
        for (i, b) in digest.0.iter().enumerate() {
            hex[i * 2] = HEX_DIGITS[usize::from(b >> 4)];
            hex[i * 2 + 1] = HEX_DIGITS[usize::from(b & 0xf)];
        }
        digest = md5::compute(hex);
    }

    // Convert final digest to hex for analysis
    let mut hex = [0u8; 32];
    for (i, b) in digest.0.iter().enumerate() {
        hex[i * 2] = HEX_DIGITS[usize::from(b >> 4)];
        hex[i * 2 + 1] = HEX_DIGITS[usize::from(b & 0xf)];
    }

    let mut triplet = None;
    let mut quintuplets = 0;

    // Scan for triplets and quintuplets
    // We need the *first* triplet for the index validity check.
    // We need *any* quintuplet for the verification check.

    // Optimization: Single pass?
    // We need to look at windows of 3 and 5.

    // Find first triplet
    for i in 0..32 - 2 {
        if hex[i] == hex[i + 1] && hex[i] == hex[i + 2] {
            if triplet.is_none() {
                let val = if hex[i] <= b'9' {
                    hex[i] - b'0'
                } else {
                    hex[i] - b'a' + 10
                };
                triplet = Some(val);
            }

            // Check for quintuplet starting at i
            if i < 32 - 4 && hex[i] == hex[i + 3] && hex[i] == hex[i + 4] {
                let val = if hex[i] <= b'9' {
                    hex[i] - b'0'
                } else {
                    hex[i] - b'a' + 10
                };
                quintuplets |= 1 << val;
            }
        }
    }

    HashData { triplet, quintuplets }
}

fn find_key(data: &str, stretching: usize) -> usize {
    let salt = data.trim().as_bytes();
    let batch_size = 5000;

    let mut hashes: Vec<HashData> = Vec::new();
    let mut search_idx = 0;
    let mut keys_found = 0;

    loop {
        // Ensure we have enough hashes to check the next 1000 items
        let needed_len = search_idx + 1000 + 1;
        if hashes.len() < needed_len {
            let start = hashes.len();
            let end = start + batch_size;

            // Generate next batch in parallel
            let new_hashes: Vec<HashData> = (start..end)
                .into_par_iter()
                .map(|i| compute_hash(salt, i, stretching))
                .collect();

            hashes.extend(new_hashes);
        }

        while search_idx + 1000 < hashes.len() {
            let h = hashes[search_idx];
            if let Some(t) = h.triplet {
                let mask = 1 << t;
                // Check if any of the next 1000 hashes has the corresponding quintuplet
                let mut found = false;
                for k in 1..=1000 {
                    if hashes[search_idx + k].quintuplets & mask != 0 {
                        found = true;
                        break;
                    }
                }

                if found {
                    keys_found += 1;
                    if keys_found == 64 {
                        return search_idx;
                    }
                }
            }
            search_idx += 1;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solve1() {
        assert_eq!(find_key("abc", 0), 22728);
    }

    #[test]
    #[cfg_attr(debug_assertions, ignore)] // Slow in debug
    fn test_solve2() {
        assert_eq!(find_key("abc", 2016), 22551);
    }
}
