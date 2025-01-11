//! [Day 18: Snailfish](https://adventofcode.com/2021/day/18)

use itertools::Itertools;
use std::fmt;
use std::ops::{Add, AddAssign};

const DEPTH: usize = 5;
const MAX_SIZE: usize = 1 << DEPTH;

#[derive(Clone, PartialEq, Debug)]
struct Snailfish {
    v: Vec<Option<u32>>,
}

impl Snailfish {
    fn new() -> Self {
        Self {
            v: vec![None; MAX_SIZE],
        }
    }

    fn from_str(s: &str) -> Self {
        let mut fish = Self::new();
        let mut depth = 0;
        let mut i = 0;

        for c in s.chars() {
            match c {
                '[' => {
                    depth += 1;
                    assert!(depth <= DEPTH, "Depth exceeds maximum allowed");
                }
                ']' => depth -= 1,
                c if c.is_ascii_digit() => {
                    fish.v[i] = c.to_digit(10);
                    i += 1 << (DEPTH - depth);
                }
                _ => {}
            }
        }
        assert_eq!(depth, 0, "Mismatched brackets");
        fish
    }

    fn raw_add(&mut self, rhs: &Self) {
        if self.v.iter().all_equal_value() == Ok(&None) {
            self.v.clone_from(&rhs.v);
        } else {
            for i in 0..MAX_SIZE / 2 {
                self.v[i] = self.v[i * 2];
            }
            for i in 0..MAX_SIZE / 2 {
                self.v[i + 16] = rhs.v[i * 2];
            }
        }
    }

    fn reduce(&mut self) {
        while self.explode() || self.split() {}
    }

    fn explode(&mut self) -> bool {
        for i in (0..MAX_SIZE).step_by(2) {
            if let (Some(left), Some(right)) = (self.v[i], self.v[i + 1]) {
                // Find first number to the left
                let mut k = i;
                while k > 0 && self.v[k - 1].is_none() {
                    k -= 1;
                }
                if k > 0 {
                    self.v[k - 1] = Some(self.v[k - 1].unwrap() + left);
                }

                // Find first number to the right
                k = i + 1;
                while k < MAX_SIZE - 1 && self.v[k + 1].is_none() {
                    k += 1;
                }
                if k < MAX_SIZE - 1 {
                    self.v[k + 1] = Some(self.v[k + 1].unwrap() + right);
                }

                // Replace pair with 0
                self.v[i] = Some(0);
                self.v[i + 1] = None;
                return true;
            }
        }
        false
    }

    fn split(&mut self) -> bool {
        for i in (0..MAX_SIZE).step_by(2) {
            if let Some(num) = self.v[i] {
                if num >= 10 {
                    self.v[i] = Some(num / 2);

                    let mut k = 1;
                    let mut ii = i;
                    while ii % 2 == 0 && k < DEPTH {
                        k *= 2;
                        ii /= 2;
                    }
                    while k > 0 {
                        let j = i + k;
                        if j < MAX_SIZE && self.v[j].is_none() {
                            self.v[j] = Some((num + 1) / 2);
                            return true;
                        }
                        k /= 2;
                    }
                    panic!("Unable to find slot for split");
                }
            }
        }
        false
    }

    fn magnitude(&self) -> u32 {
        fn mag(v: &[Option<u32>]) -> u32 {
            if v.len() == 1 || v[v.len() / 2].is_none() {
                v[0].unwrap()
            } else {
                let (left, right) = v.split_at(v.len() / 2);
                3 * mag(left) + 2 * mag(right)
            }
        }
        mag(&self.v)
    }
}

impl Add for Snailfish {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        let mut result = self;
        result.raw_add(&rhs);
        result.reduce();
        result
    }
}

impl AddAssign for Snailfish {
    fn add_assign(&mut self, other: Self) {
        self.raw_add(&other);
        self.reduce();
    }
}

impl fmt::Display for Snailfish {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fn format(v: &[Option<u32>]) -> String {
            if v.len() == 1 || v[v.len() / 2].is_none() {
                v[0].unwrap().to_string()
            } else {
                let (left, right) = v.split_at(v.len() / 2);
                format!("[{},{}]", format(left), format(right))
            }
        }
        write!(f, "{}", format(&self.v))
    }
}

fn solve(data: &str) -> (Snailfish, u32, u32) {
    let numbers: Vec<Snailfish> = data.lines().map(Snailfish::from_str).collect();

    let sum = numbers.iter().cloned().reduce(|a, b| a + b).unwrap();

    let part1 = sum.magnitude();

    let part2 = numbers
        .iter()
        .permutations(2)
        .map(|pair| (pair[0].clone() + pair[1].clone()).magnitude())
        .max()
        .unwrap();

    (sum, part1, part2)
}

fn main() {
    let args = aoc::parse_args();
    let (_, part1, part2) = solve(&args.input);
    println!("{part1}");
    println!("{part2}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_str() {
        let data = include_str!("sample_1.txt");

        for line in data.lines() {
            let a = Snailfish::from_str(line);
            assert_eq!(a.to_string().as_str(), line);
        }
    }

    #[test]
    fn test_explode() {
        let cases = vec![
            ("[[[[[9,8],1],2],3],4]", "[[[[0,9],2],3],4]"),
            ("[7,[6,[5,[4,[3,2]]]]]", "[7,[6,[5,[7,0]]]]"),
            ("[[6,[5,[4,[3,2]]]],1]", "[[6,[5,[7,0]]],3]"),
            (
                "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]",
                "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]",
            ),
            (
                "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]",
                "[[3,[2,[8,0]]],[9,[5,[7,0]]]]",
            ),
        ];

        for (before, after) in cases {
            let mut fish = Snailfish::from_str(before);
            fish.explode();
            assert_eq!(fish, Snailfish::from_str(after));
        }
    }

    #[test]
    fn test_split() {
        let mut a = Snailfish::from_str("[[[[4,3],4],4],[7,[[8,4],9]]]");
        let b = Snailfish::from_str("[1,1]");

        a.raw_add(&b);
        assert_eq!(a.to_string(), "[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]");

        a.explode();
        assert_eq!(a.to_string(), "[[[[0,7],4],[7,[[8,4],9]]],[1,1]]");
        a.explode();
        assert_eq!(a.to_string(), "[[[[0,7],4],[15,[0,13]]],[1,1]]");
        a.split();
        assert_eq!(a.to_string(), "[[[[0,7],4],[[7,8],[0,13]]],[1,1]]");
        a.split();
        assert_eq!(a.to_string(), "[[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]");
        a.explode();
        assert_eq!(a.to_string(), "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]");
    }

    #[test]
    fn test_sum() {
        let mut a = Snailfish::new();

        a += Snailfish::from_str("[1,1]");
        a += Snailfish::from_str("[2,2]");
        a += Snailfish::from_str("[3,3]");
        a += Snailfish::from_str("[4,4]");
        assert_eq!(a, Snailfish::from_str("[[[[1,1],[2,2]],[3,3]],[4,4]]"));

        a += Snailfish::from_str("[5,5]");
        assert_eq!(a, Snailfish::from_str("[[[[3,0],[5,3]],[4,4]],[5,5]]"));

        a += Snailfish::from_str("[6,6]");
        assert_eq!(a, Snailfish::from_str("[[[[5,0],[7,4]],[5,5]],[6,6]]"));
    }

    #[test]
    fn test_additions() {
        for addition in include_str!("sample_7.txt").split("\n\n") {
            let lines: Vec<_> = addition.lines().collect();

            let a = Snailfish::from_str(lines[0].trim_ascii());
            let b = Snailfish::from_str(lines[1].strip_prefix('+').unwrap().trim_ascii());
            let c = Snailfish::from_str(lines[2].strip_prefix('=').unwrap().trim_ascii());

            assert_eq!(a + b, c);
        }
    }

    #[test]
    fn test_magnitude() {
        assert_eq!(Snailfish::from_str("[9,1]").magnitude(), 29);
        assert_eq!(Snailfish::from_str("[[9,1],[1,9]]").magnitude(), 129);
        assert_eq!(Snailfish::from_str("[[1,2],[[3,4],5]]").magnitude(), 143);

        assert_eq!(
            Snailfish::from_str("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]").magnitude(),
            1384
        );
        assert_eq!(
            Snailfish::from_str("[[[[1,1],[2,2]],[3,3]],[4,4]]").magnitude(),
            445
        );
        assert_eq!(
            Snailfish::from_str("[[[[3,0],[5,3]],[4,4]],[5,5]]").magnitude(),
            791
        );
        assert_eq!(
            Snailfish::from_str("[[[[5,0],[7,4]],[5,5]],[6,6]]").magnitude(),
            1137
        );
        assert_eq!(
            Snailfish::from_str("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]")
                .magnitude(),
            3488
        );
    }

    #[test]
    fn test_solve() {
        let data = include_str!("sample_8.txt");

        let (sum, mag, mag_max) = solve(data);

        assert_eq!(
            sum,
            Snailfish::from_str("[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]")
        );
        assert_eq!(mag, 4140);
        assert_eq!(mag_max, 3993);

        let a = Snailfish::from_str("[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]");
        let b = Snailfish::from_str("[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]");
        let c = a + b;
        assert_eq!(
            c,
            Snailfish::from_str("[[[[7,8],[6,6]],[[6,0],[7,7]]],[[[7,8],[8,8]],[[7,9],[0,6]]]]")
        );
        assert_eq!(c.magnitude(), 3993);
    }
}
