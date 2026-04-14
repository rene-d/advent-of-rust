//! [Day 15: Dueling Generators](https://adventofcode.com/2017/day/15)

use rayon::iter::{ParallelBridge, ParallelIterator};

const CHUNK: usize = 1_000_000;
const MODULUS: u64 = 0x7fff_ffff;
const FACTOR_A: u64 = 16807;
const FACTOR_B: u64 = 48271;

struct Puzzle {
    a: u64,
    b: u64,
}

/// Fast modulo reduction for the Mersenne prime `2^31 − 1`.
///
/// For all products of two values below `MODULUS`, `n < (MODULUS-1)^2 < 2^62`,
/// so `(n >> 31) + (n & MODULUS) < 2 * MODULUS` and one conditional subtraction suffices.
#[inline]
const fn mmod(n: u64) -> u64 {
    let t = (n >> 31) + (n & MODULUS);
    if t >= MODULUS { t - MODULUS } else { t }
}

const fn mod_pow(mut base: u64, mut exp: u64) -> u64 {
    let mut res = 1_u64;
    base = mmod(base);
    while exp > 0 {
        if exp & 1 == 1 {
            res = mmod(res * base);
        }
        exp >>= 1;
        base = mmod(base * base);
    }
    res
}

impl Puzzle {
    fn new(data: &str) -> Self {
        let mut a = 0;
        let mut b = 0;

        for line in data.lines() {
            if let Some(gen_a) = line.strip_prefix("Generator A starts with ") {
                a = gen_a.parse().unwrap();
            } else if let Some(gen_b) = line.strip_prefix("Generator B starts with ") {
                b = gen_b.parse().unwrap();
            }
        }

        Self { a, b }
    }

    /// Solve part one.
    fn part1(&self) -> u32 {
        (0_u64..40_000_000_u64)
            .step_by(CHUNK)
            .par_bridge()
            .map(|start| {
                let mut a = mmod(self.a * mod_pow(FACTOR_A, start));
                let mut b = mmod(self.b * mod_pow(FACTOR_B, start));
                let mut n = 0_u32;

                for _ in 0..CHUNK {
                    a = mmod(a * FACTOR_A);
                    b = mmod(b * FACTOR_B);
                    n += u32::from(a & 0xffff == b & 0xffff);
                }

                n
            })
            .sum()
    }

    /// Solve part two.
    fn part2(&self) -> u32 {
        const N: usize = 5_000_000;
        let (start_a, start_b) = (self.a, self.b);

        // Generate both filtered sequences in parallel: A (multiples of 4) and B (multiples of 8).
        let (a_vals, b_vals) = rayon::join(
            move || {
                let mut vals = Vec::with_capacity(N);
                let mut a = start_a;
                while vals.len() < N {
                    a = mmod(a * FACTOR_A);
                    if a.trailing_zeros() >= 2 {
                        vals.push(a & 0xffff);
                    }
                }
                vals
            },
            move || {
                let mut vals = Vec::with_capacity(N);
                let mut b = start_b;
                while vals.len() < N {
                    b = mmod(b * FACTOR_B);
                    if b.trailing_zeros() >= 3 {
                        vals.push(b & 0xffff);
                    }
                }
                vals
            },
        );

        a_vals
            .iter()
            .zip(b_vals.iter())
            .map(|(&a, &b)| u32::from(a == b))
            .sum()
    }
}

/// # Panics
#[must_use]
pub fn solve(data: &str) -> (u32, u32) {
    let puzzle = Puzzle::new(data);
    (puzzle.part1(), puzzle.part2())
}

pub fn main() {
    let args = aoc::parse_args();
    args.run(solve);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test01() {
        let mut puzzle = Puzzle::new("");
        puzzle.a = 65;
        puzzle.b = 8921;
        assert_eq!(puzzle.part1(), 588);
    }

    #[test]
    fn test02() {
        let mut puzzle = Puzzle::new("");
        puzzle.a = 65;
        puzzle.b = 8921;
        assert_eq!(puzzle.part2(), 309);
    }
}
