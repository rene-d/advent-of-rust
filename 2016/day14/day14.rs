//! [Day 14: One-Time Pad](https://adventofcode.com/2016/day/14)

use rustc_hash::FxHashMap;

/// Solve the day 14 puzzle.
fn main() {
    let args = aoc::parse_args();

    let data = args.input.trim_ascii();

    println!("{}", solve(data.as_bytes(), 0));
    println!("{}", solve(data.as_bytes(), 2016));
}

const HEX_DIGITS: [u8; 16] = [
    b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'a', b'b', b'c', b'd', b'e', b'f',
];

/// Triplet for a given index.
#[derive(Debug, Clone, Copy)]
struct TripletHash {
    index: u32,          // index that produces hash with triplet
    triplet: u8,         // the first triplet of the hash
    quintuplet: [u8; 6], // six quintuplets max in 32 digits
}

impl TripletHash {
    /// Find the next index that produces a hash that contains a triplet
    /// and search for eventual quintuplets. As quintuplets are also triplets,
    /// we cannot miss them.
    fn next(index: u32, salt: &[u8], key_stretching: u32) -> Self {
        let mut index = index;

        let salt_len = salt.len();
        let mut hash = [0u8; 32];

        hash[..salt_len].copy_from_slice(salt);

        loop {
            // number of digits of index
            let mut hash_len = salt_len;
            let mut tmp_index = index;
            loop {
                hash_len += 1;
                tmp_index /= 10;
                if tmp_index == 0 {
                    break;
                }
            }
            // write digits of index in hash
            let mut tmp_index = index;
            let mut i = hash_len;
            loop {
                i -= 1;
                hash[i] = (tmp_index % 10) as u8 + b'0';
                tmp_index /= 10;
                if tmp_index == 0 {
                    break;
                }
            }

            let mut digest = md5::compute(&hash[..hash_len]);

            // apply key stretching
            for _ in 0..key_stretching {
                let mut hex = [0u8; 32];
                for (i, b) in digest.0.iter().enumerate() {
                    hex[i * 2] = HEX_DIGITS[usize::from(b >> 4)];
                    hex[i * 2 + 1] = HEX_DIGITS[usize::from(b & 0xf)];
                }

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
fn solve(salt: &[u8], key_stretching: u32) -> u32 {
    let mut memoize = FxHashMap::default();

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
    assert_eq!(solve("abc".as_bytes(), 0), 22728);
}

#[test]
fn test_solve2() {
    // assert_eq!(solve("abc", 2016), 22551);
}
