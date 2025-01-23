//! [Day 18: Operation Order](https://adventofcode.com/2020/day/18)

#[derive(PartialEq, Eq, Debug)]
enum Token {
    Num(u64),
    Mul,
    Add,
    Open,
    Close,
}
impl Token {
    const fn calc(&self, a: u64, b: u64) -> u64 {
        match self {
            Self::Add => a + b,
            Self::Mul => a * b,
            _ => panic!(),
        }
    }
}

struct Expr(Vec<Token>);

impl Expr {
    fn parse(line: &str) -> Self {
        let mut tokens = Vec::new();
        for c in line.chars() {
            let x = match c {
                '(' => Token::Open,
                ')' => Token::Close,
                '+' => Token::Add,
                '*' => Token::Mul,
                '0'..='9' => Token::Num(u64::from(c.to_digit(10).unwrap())),
                ' ' => continue,
                _ => panic!(),
            };
            tokens.push(x);
        }
        Self(tokens)
    }

    fn eval(&self, i: usize) -> (u64, usize) {
        let (mut left, mut next_idx) = self.get_num(i);

        loop {
            match self.0.get(next_idx) {
                Some(Token::Close) | None => break,
                Some(op @ (Token::Add | Token::Mul)) => {
                    let right;
                    (right, next_idx) = self.get_num(next_idx + 1);
                    left = op.calc(left, right);
                }
                op => panic!("unexpected token {op:?}"),
            }
        }

        (left, next_idx)
    }

    fn get_num(&self, idx: usize) -> (u64, usize) {
        match self.0.get(idx) {
            Some(Token::Open) => {
                let (value, end_idx) = self.eval(idx + 1);
                (value, end_idx + 1)
            }
            Some(Token::Num(num)) => (*num, idx + 1),
            op => panic!("unexpected token {op:?}"),
        }
    }

    fn eval_mul(&self, i: usize) -> (u64, usize) {
        let (mut left, mut next_idx) = self.eval_add(i);

        while next_idx < self.0.len() {
            match self.0.get(next_idx) {
                Some(Token::Close) => break,
                Some(Token::Mul) => {
                    let right;
                    (right, next_idx) = self.eval_add(next_idx + 1);
                    left *= right;
                }
                op => panic!("unexpected token {op:?}"),
            };
        }

        (left, next_idx)
    }

    fn eval_add(&self, idx: usize) -> (u64, usize) {
        let (mut left, mut next_idx) = self.get_num_mul(idx);

        while next_idx < self.0.len() {
            match self.0.get(next_idx) {
                Some(Token::Close | Token::Mul) => break,
                Some(Token::Add) => {
                    let right;
                    (right, next_idx) = self.get_num_mul(next_idx + 1);
                    left += right;
                }
                op => panic!("unexpected token {op:?}"),
            };
        }

        (left, next_idx)
    }

    fn get_num_mul(&self, idx: usize) -> (u64, usize) {
        match self.0.get(idx) {
            Some(Token::Open) => {
                let (value, end_idx) = self.eval_mul(idx + 1);
                (value, end_idx + 1)
            }
            Some(Token::Num(num)) => (*num, idx + 1),
            op => panic!("unexpected token {op:?}"),
        }
    }
}

/// # Panics
/// over malformed input
#[must_use]
pub fn solve(data: &str) -> (u64, u64) {
    let mut part1 = 0;
    let mut part2 = 0;

    for line in data.lines() {
        let expr = Expr::parse(line);

        part1 += expr.eval(0).0;
        part2 += expr.eval_mul(0).0;
    }

    (part1, part2)
}

pub fn main() {
    let args = aoc::parse_args();
    args.run(solve);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn expr() {
        let a = Expr::parse("1 + 2 * 3 + 4 * 5 + 6");
        assert_eq!(a.eval(0).0, 71);
        assert_eq!(a.eval_mul(0).0, 231);

        let a = Expr::parse("1 + (2 * 3) + (4 * (5 + 6))");
        assert_eq!(a.eval_mul(0).0, 51);

        let a = Expr::parse("2 * 3 + (4 * 5)");
        assert_eq!(a.eval(0).0, 26);
        assert_eq!(a.eval_mul(0).0, 46);

        let a = Expr::parse("5 + (8 * 3 + 9 + 3 * 4 * 3)");
        assert_eq!(a.eval(0).0, 437);
        assert_eq!(a.eval_mul(0).0, 1445);

        let a = Expr::parse("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))");
        assert_eq!(a.eval(0).0, 12240);
        assert_eq!(a.eval_mul(0).0, 669060);

        let a = Expr::parse("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2");
        assert_eq!(a.eval(0).0, 13632);
        assert_eq!(a.eval_mul(0).0, 23340);
    }
}
