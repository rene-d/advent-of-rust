//! [Day 13: Claw Contraption](https://adventofcode.com/2024/day/13)

struct ClawMachine {
    a_x: i64,
    a_y: i64,
    b_x: i64,
    b_y: i64,
    p_x: i64,
    p_y: i64,
}

impl ClawMachine {
    fn parse(s: &str) -> Self {
        let mut values = Vec::new();
        let mut chars = s.chars().peekable();

        // find positive integers into chars
        while let Some(ch) = chars.next() {
            if let Some(mut num) = ch.to_digit(10).map(i64::from) {
                while let Some(next_ch) = chars.peek().copied() {
                    if let Some(digit) = next_ch.to_digit(10) {
                        num = num * 10 + i64::from(digit);
                        chars.next();
                    } else {
                        break;
                    }
                }
                values.push(num);
            }
        }

        Self {
            a_x: values[0],
            a_y: values[1],
            b_x: values[2],
            b_y: values[3],
            p_x: values[4],
            p_y: values[5],
        }
    }

    const fn price(&self, position_offset: i64) -> i64 {
        let p_x = self.p_x + position_offset;
        let p_y = self.p_y + position_offset;

        // Cramer's rule:
        // a * a_x + b * b_x = p_x
        // a * a_y + b * b_y = p_y
        //
        // | a_x b_x | | a | = | p_x |
        // | a_y b_y | | b | = | p_y |
        //
        // det = a_x * b_y - a_y * b_x
        // a = (p_x * b_y - p_y * b_x) / det
        // b = (a_x * p_y - a_y * p_x) / det

        let det = self.a_x * self.b_y - self.a_y * self.b_x;

        // If determinant is 0, lines are parallel.
        // Logic assumes a unique solution is required, or at least one exists.
        // Given problem constraints, usually unique structure is implied unless parallel.
        if det == 0 {
            return 0;
        }

        let num_a = p_x * self.b_y - p_y * self.b_x;
        let num_b = self.a_x * p_y - self.a_y * p_x;

        if num_a % det != 0 || num_b % det != 0 {
            return 0;
        }

        let a = num_a / det;
        let b = num_b / det;

        if a < 0 || b < 0 {
            return 0;
        }

        a * 3 + b
    }
}

struct Puzzle {
    machines: Vec<ClawMachine>,
}

impl Puzzle {
    fn new(data: &str) -> Self {
        Self {
            machines: data.split("\n\n").map(ClawMachine::parse).collect(),
        }
    }

    /// Solve part one.
    fn part1(&self) -> i64 {
        self.machines.iter().map(|machine| machine.price(0)).sum()
    }

    /// Solve part two.
    fn part2(&self) -> i64 {
        self.machines
            .iter()
            .map(|machine| machine.price(10_000_000_000_000))
            .sum()
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
    fn test01() {
        let puzzle = Puzzle::new(TEST_INPUT);
        assert_eq!(puzzle.part1(), 480);
    }

    #[test]
    fn test02() {
        let puzzle = Puzzle::new(TEST_INPUT);
        assert_eq!(puzzle.part2(), 875318608908);
    }
}
