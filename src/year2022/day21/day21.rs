//! [Day 21: Monkey Math](https://adventofcode.com/2022/day/21)


use num::rational::Rational64;
use rustc_hash::FxHashMap;
use std::ops;

enum Job {
    Number(i64),
    Operation((String, char, String)),
}

/// Store an affine equation ax+b
#[derive(Debug)]
struct Affine {
    a: Rational64,
    b: Rational64,
}

struct Puzzle {
    monkeys: FxHashMap<String, Job>,
}

impl Puzzle {
    fn new(data: &str) -> Self {
        let mut monkeys = FxHashMap::default();

        for line in data.lines() {
            if line.is_empty() {
                continue;
            }
            let mut it = line.split(':');
            let monkey = it.next().unwrap();
            let job = it.next().unwrap().trim();

            // println!("job {}={}", monkey, job);

            if let Ok(n) = job.parse::<i64>() {
                monkeys.insert(monkey.to_string(), Job::Number(n));
            } else {
                let mut args = job.split(' ');
                let l = args.next().unwrap().to_string();
                let o = args.next().unwrap().chars().next().unwrap();
                let r = args.next().unwrap().to_string();
                monkeys.insert(monkey.to_string(), Job::Operation((l, o, r)));
            }
        }

        Self { monkeys }
    }

    fn eval(&self, var: &str) -> i64 {
        self.monkeys.get(var).map_or_else(
            || {
                panic!("???");
            },
            |m| match m {
                Job::Number(n) => *n,
                Job::Operation((l, o, r)) => {
                    let l = self.eval(l);
                    let r = self.eval(r);

                    match o {
                        '+' => l + r,
                        '-' => l - r,
                        '*' => l * r,
                        '/' => l / r,
                        _ => panic!("unknown operation"),
                    }
                }
            },
        )
    }

    fn eval_sym(&self, var: &str) -> Affine {
        if var == "humn" {
            Affine::from_x()
        } else if let Some(m) = self.monkeys.get(var) {
            match m {
                Job::Number(n) => Affine::from(*n),
                Job::Operation((l, o, r)) => {
                    let l = self.eval_sym(l);
                    let r = self.eval_sym(r);

                    return match o {
                        '+' => l + r,
                        '-' => l - r,
                        '*' => l * r,
                        '/' => l / r,
                        _ => panic!("unknown operation"),
                    };
                }
            }
        } else {
            panic!("???");
        }
    }

    // Solves part one
    fn part1(&self) -> i64 {
        self.eval("root")
    }

    // Solve part two
    fn part2(&self) -> i64 {
        let eq = match self.monkeys.get("root") {
            Some(Job::Operation((l, _, r))) => self.eval_sym(l) - self.eval_sym(r),
            _ => panic!("root problem"),
        };

        let x = eq.x();

        // we expect an integer solution
        assert_eq!(x.denom(), &1);

        *x.numer()
    }
}

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
    fn test01() {
        let puzzle = Puzzle::new(TEST_INPUT);
        assert_eq!(puzzle.part1(), 152);
        assert_eq!(puzzle.part2(), 301);
    }
}

impl Affine {
    fn from(n: i64) -> Self {
        Self {
            a: Rational64::from_integer(0),
            b: Rational64::from_integer(n),
        }
    }
    fn from_x() -> Self {
        Self {
            a: Rational64::from_integer(1),
            b: Rational64::from_integer(0),
        }
    }
    fn x(&self) -> Rational64 {
        -self.b / self.a
    }
}

impl ops::Add<Self> for Affine {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self {
            a: self.a + rhs.a,
            b: self.b + rhs.b,
        }
    }
}

impl ops::Sub<Self> for Affine {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self {
            a: self.a - rhs.a,
            b: self.b - rhs.b,
        }
    }
}

impl ops::Mul<Self> for Affine {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        if self.a.numer() == &0 {
            Self {
                a: self.b * rhs.a,
                b: self.b * rhs.b,
            }
        } else if rhs.a.numer() == &0 {
            Self {
                a: self.a * rhs.b,
                b: self.b * rhs.b,
            }
        } else {
            // cannot (ax+b)*(cx+d)
            panic!("mul");
        }
    }
}

impl ops::Div<Self> for Affine {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        // cannot A/(ax+b)
        assert_eq!(rhs.a.numer(), &0);

        Self {
            a: self.a / rhs.b,
            b: self.b / rhs.b,
        }
    }
}

impl std::fmt::Display for Affine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}x + {}", self.a, self.b)
    }
}
