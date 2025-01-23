//! [Day 25: Combo Breaker](https://adventofcode.com/2020/day/25)

use aoc::math::{IntegerMathOps, SignedMathOps, UnsignedMathOps};
use rustc_hash::FxHashMap;

fn mod_inv(a: u64, n: u64) -> u64 {
    // clippy doesn't mess with the sign...
    let a = i64::try_from(a).unwrap();
    let n = i64::try_from(n).unwrap();
    u64::try_from(a.mod_inv(n).unwrap()).unwrap()
}

/// Compute discrete log with Baby-step giant-step algorithm.
///
/// Find `x` such that `a^x = b (mod n)`.
///
/// <https://en.wikipedia.org/wiki/Baby-step_giant-step>
fn discrete_log(a: u64, mut b: u64, order_n: u64) -> Option<u64> {
    let m = order_n.sqrt() + 1;

    let mut a_j = FxHashMap::default();

    // Baby-Step
    let mut a_m = 1;
    for j in 0..m {
        a_j.insert(a_m, j);
        a_m = (a_m * a) % order_n;
    }

    let inv_a_m = mod_inv(a_m, order_n);

    // Giant-Step
    for i in 0..m {
        if let Some(j) = a_j.get(&b) {
            return Some(i * m + j);
        }
        b = (b * inv_a_m) % order_n;
    }

    None
}

/// # Panics
/// over malformed input
#[must_use]
pub fn solve(data: &str) -> (u64, aoc::Christmas) {
    let (a, b) = data.split_once('\n').unwrap();

    let card_pub_key: u64 = a.trim().parse().unwrap();
    let door_pub_key: u64 = b.trim().parse().unwrap();

    let card_loop_count = discrete_log(7, card_pub_key, 20_201_227).unwrap();

    (
        door_pub_key.mod_exp(card_loop_count, 20_201_227),
        aoc::CHRISTMAS,
    )
}

pub fn main() {
    let args = aoc::parse_args();
    args.run(solve);
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = include_str!("test.txt");

    #[test]
    fn part1() {
        assert_eq!(solve(TEST_INPUT).0, 14897079);
    }
}
