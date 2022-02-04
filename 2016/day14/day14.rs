/*!
[Day 14: One-Time Pad](https://adventofcode.com/2016/day/14)

In order to communicate securely with Santa while you're on this mission,
you've been using a [one-time pad](https://en.wikipedia.org/wiki/One-time_pad)
that you [generate](https://en.wikipedia.org/wiki/Security_through_obscurity)
using a pre-agreed algorithm. Unfortunately, you've run out of keys in your
one-time pad, and so you need to generate some more.

To generate keys, you first get a stream of random data by taking the
[MD5](https://en.wikipedia.org/wiki/MD5) of a pre-arranged
[salt](https://en.wikipedia.org/wiki/Salt_(cryptography)) (your puzzle
input) and an increasing integer index (starting with 0, and represented
in decimal); the resulting MD5 hash should be represented as a string of
**lowercase** hexadecimal digits.

However, not all of these MD5 hashes are **keys**, and you need `64` new keys
for your one-time pad. A hash is a key **only if**:

- It contains **three** of the same character in a row, like `777`.
  Only consider the first such triplet in a hash.
- One of the next `1000` hashes in the stream contains that same
  character **five** times in a row, like `77777`.

Considering future hashes for five-of-a-kind sequences does not cause
those hashes to be skipped; instead, regardless of whether the current
hash is a key, always resume testing for keys starting with the very next
hash.

Given the actual salt in your puzzle input, **what index** produces your
`64`th one-time pad key?

--- Part Two ---

Of course, in order to make this process [even more secure](https://en.wikipedia.org/wiki/MD5#Security),
you've also implemented [key stretching](https://en.wikipedia.org/wiki/Key_stretching).

Key stretching forces attackers to spend more time generating hashes.
Unfortunately, it forces everyone else to spend more time, too.

To implement key stretching, whenever you generate a hash, before you use
it, you first find the MD5 hash of that hash, then the MD5 hash of **that**
hash, and so on, a total of **`2016` additional hashings**. Always use
lowercase hexadecimal representations of hashes.

Given the actual salt in your puzzle input and using `2016` extra MD5 calls
of key stretching, **what index** now produces your 64th one-time pad key?
*/

use std::collections::HashMap;
use std::time::Instant;

/// Solve the day 14 puzzle.
fn main() {
    let data = std::fs::read_to_string("input.txt").unwrap();
    let data = data.trim();

    let now = Instant::now();

    println!("{}", solve(data, 0));
    println!("{}", solve(data, 2016));

    let micros = now.elapsed().as_micros();
    println!("elapsed: {}.{:06} s", micros / 1_000_000, micros % 1_000_000);
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
    /// and search for eventual quintuplets.
    fn next(index: usize, salt: &str, key_stretching: usize) -> TripletHash {
        let mut index = index;

        loop {
            let hash = format!("{}{}", salt, index);
            let mut digest = md5::compute(hash);

            for _ in 0..key_stretching {
                let hex = format!("{:x}", digest);
                digest = md5::compute(hex);
            }

            let mut digits = [0u8; 32];
            for (i, b) in digest.iter().enumerate() {
                digits[i * 2] = (b >> 4) as u8;
                digits[i * 2 + 1] = b & 0xf;
            }

            let mut triplet = u8::MAX;
            let mut quintuplet = [u8::MAX; 6];
            let mut q_count = 0;
            let mut i = 0;

            while i < 32 - 2 {
                if digits[i] == digits[i + 1] && digits[i] == digits[i + 2] {
                    triplet = digits[i];
                    break;
                }
                i += 1;
            }
            if triplet == u8::MAX {
                // no triplet found
                index += 1;
                continue;
            }

            // search for quintuplets (a quintuplet is a also a triplet)
            while i < 32 - 4 {
                i += if digits[i] == digits[i + 1]
                    && digits[i] == digits[i + 2]
                    && digits[i] == digits[i + 3]
                    && digits[i] == digits[i + 4]
                {
                    quintuplet[q_count] = digits[i];
                    q_count += 1;
                    5
                } else {
                    1
                }
            }

            return TripletHash {
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
fn test_solve() {
    assert_eq!(solve("abc", 0), 22728);
}
