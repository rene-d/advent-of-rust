//! [Day 22: Slam Shuffle](https://adventofcode.com/2019/day/22)

use aoc::math::IntegerMathOps;
use aoc::math::SignedMathOps;

// Calculate `(n^x) % p`.
fn modular_exponent(n: i128, x: i128, p: i128) -> i128 {
    n.mod_exp(x, p)
}

/// Return Modular Multiplicative Inverse `x | x * n = 1 (mod p)` or 0.
fn modular_inverse(n: i128, p: i128) -> i128 {
    n.mod_inv(p).unwrap_or(0)
}

struct Congruence {
    a: i128,
    c: i128,
    m: i128,
}

impl Congruence {
    fn compose(&self, lhs: &Self) -> Self {
        assert_eq!(self.m, lhs.m);
        Self {
            a: (self.a * lhs.a) % self.m,
            c: (self.c * lhs.a + lhs.c) % self.m,
            m: self.m,
        }
    }

    const fn value(&self, index: i128) -> i128 {
        (self.a * index + self.c) % self.m
    }

    fn inv(&self) -> Self {
        let m = self.m;
        let a = modular_inverse(self.a, m);
        let c = m - (a * self.c) % m;
        Self { a, c, m }
    }

    fn pow(&self, e: i128) -> Self {
        let m = self.m;
        let a = modular_exponent(self.a, e, m);
        let c = (((a - 1) * modular_inverse(self.a - 1, m) % m) * self.c) % m;
        Self { a, c, m }
    }
}

enum Shuffle {
    DealIntoNewStack,
    Cut(i32),
    DealWithIncrement(i32),
}

impl Shuffle {
    fn op(&self, m: i128) -> Congruence {
        match self {
            Self::DealIntoNewStack => {
                // equivalent to (m-1)*x+(m-1) mod m
                Congruence {
                    a: m - 1,
                    c: m - 1,
                    m,
                }
            }

            Self::Cut(n) => {
                // equivalent to x+(m-n) mod m
                Congruence {
                    a: 1,
                    c: m - i128::from(*n),
                    m,
                }
            }

            Self::DealWithIncrement(i) => {
                // equivalent to i*x mod m
                Congruence {
                    a: i128::from(*i) % m,
                    c: 0,
                    m,
                }
            }
        }
    }
}

struct Puzzle {
    shuffles: Vec<Shuffle>,
}

impl Puzzle {
    fn new(data: &str) -> Self {
        Self {
            shuffles: data
                .lines()
                .map(|line| {
                    if line == "deal into new stack" {
                        Shuffle::DealIntoNewStack
                    } else if let Some(cut) = line.strip_prefix("cut ") {
                        Shuffle::Cut(cut.parse().unwrap())
                    } else if let Some(inc) = line.strip_prefix("deal with increment ") {
                        Shuffle::DealWithIncrement(inc.parse().unwrap())
                    } else {
                        panic!("bad line: {line}");
                    }
                })
                .collect(),
        }
    }

    /// Solve part one.
    fn part1(&self) -> i128 {
        // let mut deck: Vec<u32> = (0..10007).collect();
        // let n = deck.len();
        //
        // for technique in &self.shuffles {
        //     match technique {
        //         Shuffle::DealIntoNewStack => {
        //             deck.reverse();
        //         }
        //         Shuffle::Cut(cut) => {
        //             if cut.is_positive() {
        //                 deck.rotate_left((*cut as usize) % n);
        //             } else if cut.is_negative() {
        //                 deck.rotate_right((-*cut as usize) % n);
        //             } else {
        //                 panic!("cut 0 : wtf???");
        //             }
        //         }
        //         Shuffle::DealWithIncrement(inc) => {
        //             let orig = deck.clone();
        //             let n = n as i32;
        //             let mut pos = 0;
        //             for card in orig {
        //                 deck[pos as usize] = card;
        //                 pos = (pos + inc).rem_euclid(n);
        //             }
        //         }
        //     }
        // }
        //
        // deck.iter()
        //     .enumerate()
        //     .find(|(_, c)| c == &&2019)
        //     .unwrap()
        //     .0

        let m = 10007;
        self.shuffles
            .iter()
            .fold(Congruence { a: 1, c: 0, m }, |acc, x| acc.compose(&x.op(m)))
            .value(2019)
    }

    /// Solve part two.
    fn part2(&self) -> i128 {
        let m = 119_315_717_514_047;
        self.shuffles
            .iter()
            .fold(Congruence { a: 1, c: 0, m }, |acc, t| acc.compose(&t.op(m)))
            .inv()
            .pow(101_741_582_076_661)
            .value(2020)
    }
}

/// # Panics
#[must_use]
pub fn solve(data: &str) -> (i128, i128) {
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
        assert_eq!(modular_inverse(213, 1000000007), 32863850);
    }

    #[test]
    fn test02() {
        assert_eq!(modular_exponent(29, 830, 20253), 14587);
    }
}
