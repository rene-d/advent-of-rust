//! Implements the 'knot hash' as described in [2017 day 10](https://adventofcode.com/2017/day/10)
//! Also used in [2017 day 14](https://adventofcode.com/2017/day/14

use crate::hex::HexDisplayExt;

pub fn knot_tie(values: &[usize], sparse: &mut [u8], skip: &mut usize, pos: &mut usize) {
    let n = sparse.len();

    for length in values {
        for k in 0..(length / 2) {
            sparse.swap((*pos + k) % n, (*pos + length - 1 - k) % n);
        }

        *pos = (*pos + length + *skip) % n;
        *skip += 1;
    }
}

/// Compute a [knot hash](https://adventofcode.com/2017/day/10).
pub fn knot_hash_raw(text: &str) -> [u8; 16] {
    let mut lengths: Vec<_> = text.chars().map(u32::from).map(|u| u as usize).collect();

    lengths.extend_from_slice(&[17, 31, 73, 47, 23]);

    let mut skip = 0;
    let mut pos = 0;
    let mut sparse: Vec<u8> = (0..=255).collect();

    for _ in 0..64 {
        knot_tie(&lengths, &mut sparse, &mut skip, &mut pos);
    }

    let dense: Vec<_> = sparse
        .chunks(16)
        .map(|chunk| chunk.iter().copied().reduce(|x, y| (x ^ y)).unwrap())
        .collect();

    dense.try_into().unwrap()
}

/// Compute a [knot hash](https://adventofcode.com/2017/day/10).
/// and return its hexadecimal representation.
pub fn knot_hash(text: &str) -> String {
    knot_hash_raw(text).hex_display().to_string()
}

/// Test from puzzle input
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tie() {
        let mut skip = 0;
        let mut pos = 0;
        let mut sparse: Vec<u8> = (0..=4).collect();

        knot_tie(&[3, 4, 1, 5], &mut sparse, &mut skip, &mut pos);

        assert_eq!(sparse, &[3, 4, 2, 1, 0]);
    }

    #[test]
    fn hash1() {
        assert_eq!(knot_hash(""), "a2582a3a0e66e6e86e3812dcb672a272");
    }

    #[test]
    fn hash2() {
        assert_eq!(knot_hash("AoC 2017"), "33efeb34ea91902bb2f59c9920caa6cd");
    }

    #[test]
    fn hash3() {
        assert_eq!(knot_hash("1,2,3"), "3efbe78a8d82f29979031a4aa0b16a9d");
    }

    #[test]
    fn hash4() {
        assert_eq!(knot_hash("1,2,4"), "63960835bcdc130f0b66d7ff4f6a5a8e");
    }
}
