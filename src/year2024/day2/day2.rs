//! [Day 2: Red-Nosed Reports](https://adventofcode.com/2024/day/2)

struct Puzzle {
    reports: Vec<Vec<i32>>,
}

impl Puzzle {
    fn new(data: &str) -> Self {
        let mut reports = Vec::new();

        for line in data.lines() {
            reports.push(
                line.split_whitespace()
                    .filter_map(|s: &str| s.parse().ok())
                    .collect(),
            );
        }

        Self { reports }
    }

    fn is_safe(v: &[i32]) -> bool {
        let safe_rule = |a: i32, b: i32| -> bool { (1 <= a - b) && (a - b <= 3) };

        let increasing = v.windows(2).all(|pair| safe_rule(pair[0], pair[1]));
        let decreasing = v.windows(2).all(|pair| safe_rule(pair[1], pair[0]));

        increasing || decreasing
    }

    fn is_safe_except_one(v: &[i32]) -> bool {
        for i in 0..v.len() {
            let mut w = v.to_vec();
            w.remove(i);
            if Self::is_safe(&w) {
                return true;
            }
        }
        false
    }

    /// Solve part one.
    fn part1(&self) -> usize {
        self.reports
            .iter()
            .filter(|v: &&Vec<i32>| Self::is_safe(v))
            .count()
    }

    /// Solve part two.
    fn part2(&self) -> usize {
        self.reports
            .iter()
            .filter(|v: &&Vec<i32>| Self::is_safe(v) || Self::is_safe_except_one(v))
            .count()
    }
}

/// # Panics
#[must_use]
pub fn solve(data: &str) -> (usize, usize) {
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
        assert_eq!(puzzle.part1(), 2);
    }

    #[test]
    fn test02() {
        let puzzle = Puzzle::new(TEST_INPUT);
        assert_eq!(puzzle.part2(), 4);
    }
}
