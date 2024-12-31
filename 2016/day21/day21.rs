//! [Day 21: Scrambled Letters and Hash](https://adventofcode.com/2016/day/21)

use regex::Regex;

enum Operation {
    SwapPosition(usize, usize),
    SwapLetter(char, char),
    ReversePositions(usize, usize),
    RotateLeft(usize),
    RotateRight(usize),
    MovePosition(usize, usize),
    RotateBased(char),
}

impl Operation {
    #[cfg(test)]
    fn perform_str(&self, password: &str) -> String {
        let mut p: Vec<_> = password.chars().collect();
        self.perform(&mut p);
        String::from_iter(p.iter())
    }

    #[cfg(test)]
    fn reverse_str(&self, password: &str) -> String {
        let mut p: Vec<_> = password.chars().collect();
        self.reverse(&mut p);
        String::from_iter(p.iter())
    }

    fn perform(&self, password: &mut [char]) {
        let pos = |ch: &char| password.iter().position(|c| c == ch).unwrap();

        match self {
            &Self::SwapPosition(a, b) => {
                password.swap(a, b);
            }
            Self::SwapLetter(a, b) => {
                let a = pos(a);
                let b = pos(b);
                password.swap(a, b);
            }
            &Self::ReversePositions(a, b) => {
                for i in 0..=((b - a) / 2) {
                    password.swap(a + i, b - i);
                }
            }
            &Self::RotateLeft(p) => {
                password.rotate_left(p);
            }

            &Self::RotateRight(p) => {
                password.rotate_right(p);
            }

            &Self::MovePosition(a, b) => {
                let c = password[a];
                if a < b {
                    for i in a..b {
                        password[i] = password[i + 1];
                    }
                } else {
                    for i in (b..a).rev() {
                        password[i + 1] = password[i];
                    }
                }
                password[b] = c;
            }
            Self::RotateBased(a) => {
                let a = pos(a);
                password.rotate_right(a);
                password.rotate_right(1);
                if a >= 4 {
                    password.rotate_right(1);
                }
            }
        }
    }

    fn reverse(&self, password: &mut [char]) {
        match self {
            Self::SwapPosition(_, _) | Self::SwapLetter(_, _) | Self::ReversePositions(_, _) => {
                self.perform(password);
            }
            Self::RotateLeft(p) => Self::RotateRight(*p).perform(password),
            Self::RotateRight(p) => Self::RotateLeft(*p).perform(password),
            Self::MovePosition(a, b) => Self::MovePosition(*b, *a).perform(password),
            Self::RotateBased(_a) => {
                // dummy reverse
                let n = password.len();
                for i in 0..n {
                    let mut p: Vec<char> = password.to_vec();
                    p.rotate_right(i);
                    self.perform(&mut p);
                    if p == password {
                        password.rotate_right(i);
                        break;
                    }
                }
            }
        }
    }
}

struct Puzzle {
    ops: Vec<Operation>,
}

impl Puzzle {
    const fn new() -> Self {
        Self { ops: vec![] }
    }

    /// Get the puzzle input.
    fn configure(&mut self, data: &str) {
        let re1 = Regex::new(r"rotate based on position of letter (\w)").unwrap();
        let re2 = Regex::new(r"move position (\d+) to position (\d+)").unwrap();
        let re3 = Regex::new(r"reverse positions (\d+) through (\d+)").unwrap();
        let re4 = Regex::new(r"rotate left (\d+) steps?").unwrap();
        let re5 = Regex::new(r"rotate right (\d+) steps?").unwrap();
        let re6 = Regex::new(r"swap letter (\w) with letter (\w)").unwrap();
        let re7 = Regex::new(r"swap position (\d+) with position (\d+)").unwrap();

        for line in data.lines() {
            if let Some(m) = re1.captures(line) {
                let c = m[1].chars().next().unwrap();
                self.ops.push(Operation::RotateBased(c));
            } else if let Some(m) = re2.captures(line) {
                let a = m[1].parse().unwrap();
                let b = m[2].parse().unwrap();
                self.ops.push(Operation::MovePosition(a, b));
            } else if let Some(m) = re3.captures(line) {
                let a = m[1].parse().unwrap();
                let b = m[2].parse().unwrap();
                self.ops.push(Operation::ReversePositions(a, b));
            } else if let Some(m) = re4.captures(line) {
                let a = m[1].parse().unwrap();
                self.ops.push(Operation::RotateLeft(a));
            } else if let Some(m) = re5.captures(line) {
                let a = m[1].parse().unwrap();
                self.ops.push(Operation::RotateRight(a));
            } else if let Some(m) = re6.captures(line) {
                let a = m[1].chars().next().unwrap();
                let b = m[2].chars().next().unwrap();
                self.ops.push(Operation::SwapLetter(a, b));
            } else if let Some(m) = re7.captures(line) {
                let a = m[1].parse().unwrap();
                let b = m[2].parse().unwrap();
                self.ops.push(Operation::SwapPosition(a, b));
            } else {
                panic!("error: {line}");
            }
        }
    }

    /// Solve part one.
    fn part1(&self) -> String {
        let mut password: Vec<_> = "abcdefgh".chars().collect();

        for op in &self.ops {
            op.perform(&mut password);
        }

        String::from_iter(&password)
    }

    /// Solve part two.
    fn part2(&self) -> String {
        let mut password: Vec<_> = "fbgdceah".chars().collect();

        for op in self.ops.iter().rev() {
            op.reverse(&mut password);
        }

        String::from_iter(&password)
    }
}

fn main() {
    let args = aoc::parse_args();
    let mut puzzle = Puzzle::new();
    puzzle.configure(&args.input);
    println!("{}", puzzle.part1());
    println!("{}", puzzle.part2());
}

/// Test from puzzle input
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test01() {
        let mut password = "abcde".to_string();

        password = Operation::SwapPosition(0, 4).perform_str(&password);
        assert_eq!(password, "ebcda");

        password = Operation::SwapLetter('d', 'b').perform_str(&password);
        assert_eq!(password, "edcba");

        password = Operation::ReversePositions(0, 4).perform_str(&password);
        assert_eq!(password, "abcde");

        password = Operation::RotateLeft(1).perform_str(&password);
        assert_eq!(password, "bcdea");

        password = Operation::MovePosition(1, 4).perform_str(&password);
        assert_eq!(password, "bdeac");

        password = Operation::MovePosition(3, 0).perform_str(&password);
        assert_eq!(password, "abdec");

        password = Operation::RotateBased('b').perform_str(&password);
        assert_eq!(password, "ecabd");

        password = Operation::RotateBased('d').perform_str(&password);
        assert_eq!(password, "decab");
    }

    #[test]
    fn test02() {
        let p = Operation::RotateBased('b').perform_str("abdec");
        assert_eq!(p, "ecabd");

        let p = Operation::RotateBased('b').reverse_str(&p);
        assert_eq!(p, "abdec");

        let p = Operation::RotateBased('d').reverse_str("facdgehb");
        assert_eq!(p, "cdgehbfa");
    }
}
