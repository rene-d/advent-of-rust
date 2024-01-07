//! [Day 22: Slam Shuffle](https://adventofcode.com/2019/day/22)

#![allow(clippy::unreadable_literal)]

use clap::Parser;

#[derive(Parser)]
struct Args {
    /// Puzzle input
    #[arg(default_value = "input.txt")]
    path: String,
}

// Calculates (n^x) % p
fn modular_exponent(mut n: i128, mut x: i128, p: i128) -> i128 {
    let mut ans = 1;
    if x <= 0 {
        return 1;
    }
    loop {
        if x == 1 {
            return (ans * n) % p;
        }
        if x & 1 == 0 {
            n = (n * n) % p;
            x >>= 1;
        } else {
            ans = (ans * n) % p;
            x -= 1;
        }
    }
}

// Check GCD
fn gcd(mut a: i128, mut b: i128) -> i128 {
    if a == b {
        return a;
    }
    if b > a {
        std::mem::swap(&mut a, &mut b);
    }
    while b > 0 {
        let temp = a;
        a = b;
        b = temp % b;
    }
    a
}

// Magic starts here
fn modular_inverse(n: i128, p: i128) -> i128 {
    // Returns 0 if no Modular Multiplicative Inverse exist
    if p <= 1 || gcd(n, p) > 1 {
        return 0;
    }

    // Return Modular Multiplicative Inverse, that is (n^(p-2)) mod p
    // From Fermat's little theorem
    modular_exponent(n, p - 2, p)
}

struct Congruence {
    a: i128,
    c: i128,
    m: i128,
}

impl Congruence {
    fn compose(&self, lhs: &Self) -> Self {
        assert_eq!(self.m, lhs.m);
        Congruence {
            a: (self.a * lhs.a) % self.m,
            c: (self.c * lhs.a + lhs.c) % self.m,
            m: self.m,
        }
    }

    fn value(&self, index: i128) -> i128 {
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
    fn new() -> Puzzle {
        Puzzle { shuffles: vec![] }
    }

    /// Get the puzzle input.
    fn configure(&mut self, path: &str) {
        let data = std::fs::read_to_string(path).unwrap();

        for line in data.lines() {
            self.shuffles.push(if line == "deal into new stack" {
                Shuffle::DealIntoNewStack
            } else if let Some(cut) = line.strip_prefix("cut ") {
                Shuffle::Cut(cut.parse().unwrap())
            } else if let Some(inc) = line.strip_prefix("deal with increment ") {
                Shuffle::DealWithIncrement(inc.parse().unwrap())
            } else {
                panic!("bad line: {line}");
            });
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
        let m = 119315717514047;
        self.shuffles
            .iter()
            .fold(Congruence { a: 1, c: 0, m }, |acc, t| acc.compose(&t.op(m)))
            .inv()
            .pow(101741582076661)
            .value(2020)
    }
}

fn main() {
    let args = Args::parse();
    let mut puzzle = Puzzle::new();
    puzzle.configure(args.path.as_str());
    println!("{}", puzzle.part1());
    println!("{}", puzzle.part2());
}

/// Test from puzzle input
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
