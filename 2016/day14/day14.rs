//! [Day 14: One-Time Pad](https://adventofcode.com/2016/day/14)

use std::collections::HashMap;
// use std::time::Instant;

/// Solve the day 14 puzzle.
fn main() {
    let args = aoc::parse_args();

    let data = std::fs::read_to_string(args.path).unwrap();
    let data = data.trim();

    // let now = Instant::now();

    println!("{}", solve(data, 0));
    println!("{}", solve(data, 2016));

    // let micros = now.elapsed().as_micros();
    // println!("elapsed: {}.{:06} s", micros / 1_000_000, micros % 1_000_000);
}

/// Triplet for a given index.
#[derive(Debug, Clone, Copy)]
struct TripletHash {
    index: usize,        // index that produces hash with triplet
    triplet: u8,         // the first triplet of the hash
    quintuplet: [u8; 6], // six quintuplets max in 32 digits
}

impl TripletHash {
    /// Find the next index that produces a hash that contains a triplet
    /// and search for eventual quintuplets. As quintuplets are also triplets,
    /// we cannot miss them.
    fn next(index: usize, salt: &str, key_stretching: usize) -> Self {
        let mut index = index;

        loop {
            let hash = format!("{salt}{index}");
            let mut digest = md5::compute(hash);

            // apply key stretching
            for _ in 0..key_stretching {
                let hex = format!("{digest:x}");
                digest = md5::compute(hex);
            }

            // get the 32 hexadecimal digits
            let mut digits = [0_u8; 32];
            for (i, b) in digest.iter().enumerate() {
                digits[i * 2] = b >> 4;
                digits[i * 2 + 1] = b & 0xf;
            }

            let mut triplet = u8::MAX;
            let mut quintuplet = [u8::MAX; 6];
            let mut q_count = 0;
            let mut i = 0;

            // look only for the first triplet
            while i < 32 - 2 {
                if digits[i] == digits[i + 1] && digits[i] == digits[i + 2] {
                    triplet = digits[i];
                    break;
                }
                i += 1;
            }
            if triplet == u8::MAX {
                // no triplet found (and no quintuplet!)
                // we can safely ignore this index
                index += 1;
                continue;
            }

            // now search for quintuplets
            // we can start at the first triplet at position i
            while i < 32 - 4 {
                i += if digits[i] == digits[i + 1]
                    && digits[i] == digits[i + 2]
                    && digits[i] == digits[i + 3]
                    && digits[i] == digits[i + 4]
                {
                    quintuplet[q_count] = digits[i];
                    q_count += 1;
                    5 // increment by 5 the search position
                } else {
                    1 // skip one digit
                }
            }

            return Self {
                index,
                triplet,
                quintuplet,
            };
        }
    }
}

/// Find the 64th key with the given salt and key stretching.
fn solve(salt: &str, key_stretching: usize) -> usize {
    let mut memoize = HashMap::new();

    let mut hasher = |index| {
        *memoize
            .entry(index)
            .or_insert_with(|| TripletHash::next(index, salt, key_stretching))
    };

    let mut found = 0;
    let mut index = 0;

    'a: loop {
        let h = hasher(index);

        index = h.index;

        let mut q_index = index;

        loop {
            let q = hasher(q_index + 1);

            q_index = q.index;

            if q_index - index > 1000 {
                // no matching quintuplet found
                break;
            }

            if q.quintuplet.iter().any(|&x| x == h.triplet) {
                found += 1;
                if found == 64 {
                    break 'a;
                }
                break;
            }
        }

        index += 1;
    }

    index
}

#[test]
fn test_solve1() {
    assert_eq!(solve("abc", 0), 22728);
}

#[test]
fn test_solve2() {
    // assert_eq!(solve("abc", 2016), 22551);
}
