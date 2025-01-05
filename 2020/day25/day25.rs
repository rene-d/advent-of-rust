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

#[derive(Debug)]
struct Puzzle {
    card_pub_key: u64,
    door_pub_key: u64,
}

impl Puzzle {
    /// Initialize from the puzzle input.
    fn new(data: &str) -> Self {
        let (a, b) = data.split_once('\n').unwrap();

        let card_pub_key = a.trim().parse().unwrap();
        let door_pub_key = b.trim().parse().unwrap();
        Self {
            card_pub_key,
            door_pub_key,
        }
    }

    /// Solve part one.
    fn part1(&self) -> u64 {
        let card_loop_count = discrete_log(7, self.card_pub_key, 20_201_227).unwrap();

        self.door_pub_key.mod_exp(card_loop_count, 20_201_227)
    }
}

fn main() {
    let args = aoc::parse_args();
    let puzzle = Puzzle::new(&args.input);
    println!("{}", puzzle.part1());
}

/// Test from puzzle input
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1() {
        let data = aoc::load_input_data("test.txt");
        let puzzle = Puzzle::new(&data);
        assert_eq!(puzzle.part1(), 14897079);
    }
}
