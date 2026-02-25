//! [Day 22: Monkey Market](https://adventofcode.com/2024/day/22)

use rayon::prelude::*;

const fn next_secret(mut secret: i64) -> i64 {
    /*
    let secret = (secret ^ (secret * 64)) % 16_777_216;
    let secret = (secret ^ (secret / 32)) % 16_777_216;
    (secret ^ (secret * 2048)) % 16_777_216
    */
    secret ^= secret << 6;
    secret &= 0xFF_FFFF;
    secret ^= secret >> 5;
    secret ^= secret << 11;
    secret &= 0xFF_FFFF;
    secret
}

struct Puzzle {
    initial_secrets: Vec<i64>,
}

impl Puzzle {
    fn new(data: &str) -> Self {
        Self {
            initial_secrets: data.lines().map_while(|s| s.parse::<i64>().ok()).collect(),
        }
    }

    /// Solve part one.
    fn part1(&self) -> i64 {
        self.initial_secrets
            .par_iter()
            .map(|&initial_secret| (0..2000).fold(initial_secret, |secret, _| next_secret(secret)))
            .sum()
    }

    /// Solve part two.
    fn part2(&self) -> i64 {
        const RANGE: usize = 19;
        const SIZE: usize = RANGE * RANGE * RANGE * RANGE;

        let total_bananas: Vec<i32> = self
            .initial_secrets
            .par_iter()
            .fold(
                || vec![0; SIZE],
                |mut acc, &initial_secret| {
                    let mut seen = vec![false; SIZE];
                    let mut secret = initial_secret;
                    let mut idx = 0;

                    for i in 0..2000 {
                        let next = next_secret(secret);
                        let p_prev = (secret % 10) as i8;
                        let p_curr = (next % 10) as i8;
                        let diff = p_prev.abs_diff(p_curr + 9) as usize;

                        idx = (idx % 6859) * 19 + diff;

                        if i >= 3 && !seen[idx] {
                            acc[idx] += i32::from(p_curr);
                            seen[idx] = true;
                        }
                        secret = next;
                    }
                    acc
                },
            )
            .reduce(
                || vec![0; SIZE],
                |mut a, b| {
                    for (x, y) in a.iter_mut().zip(b.iter()) {
                        *x += *y;
                    }
                    a
                },
            );

        i64::from(*total_bananas.iter().max().unwrap_or(&0))
    }
}

/// # Panics
#[must_use]
pub fn solve(data: &str) -> (i64, i64) {
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

    const TEST_INPUT: &str = include_str!("test.txt");

    #[test]
    fn test_part1() {
        let puzzle = Puzzle::new(TEST_INPUT);
        assert_eq!(puzzle.part1(), 37327623);
    }

    #[test]
    fn test_part2() {
        let puzzle = Puzzle::new(TEST_INPUT);
        assert_eq!(puzzle.part2(), 23 + 1);
    }
}
