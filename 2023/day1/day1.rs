//! [Day 1: Trebuchet?!](https://adventofcode.com/2023/day/1)

struct Puzzle<'a> {
    data: &'a str,
}

impl<'a> Puzzle<'a> {
    const fn new(data: &'a str) -> Self {
        Self { data }
    }

    /// Solve part one.
    fn part1(&self) -> u32 {
        let mut sum = 0;
        for line in self.data.lines() {
            let digits = line
                .chars()
                .filter_map(|d| d.to_digit(10))
                .collect::<Vec<_>>();

            sum += digits.first().unwrap() * 10 + digits.last().unwrap();
        }
        sum
    }

    /// Return the value of the digit at the beginning of s or None
    fn valid_digit(s: &str) -> Option<u32> {
        let d = s.chars().nth(0).unwrap();
        let d = d.to_digit(10);
        if d.is_some() {
            d
        } else if s.starts_with("one") {
            Some(1)
        } else if s.starts_with("two") {
            Some(2)
        } else if s.starts_with("three") {
            Some(3)
        } else if s.starts_with("four") {
            Some(4)
        } else if s.starts_with("five") {
            Some(5)
        } else if s.starts_with("six") {
            Some(6)
        } else if s.starts_with("seven") {
            Some(7)
        } else if s.starts_with("eight") {
            Some(8)
        } else if s.starts_with("nine") {
            Some(9)
        } else {
            None
        }
    }

    /// Solve part two.
    fn part2(&self) -> u32 {
        let mut sum = 0;
        for line in self.data.lines() {
            for i in 0..line.len() {
                if let Some(first) = Self::valid_digit(&line[i..]) {
                    sum += first * 10;
                    break;
                }
            }

            for i in (0..line.len()).rev() {
                if let Some(last) = Self::valid_digit(&line[i..]) {
                    sum += last;
                    break;
                }
            }
        }
        sum
    }
}

/// # Panics
/// over malformed input
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

    const SAMPLE_1: &str = include_str!("sample_1.txt");
    const SAMPLE_2: &str = include_str!("sample_2.txt");

    #[test]
    fn test01() {
        let puzzle = Puzzle::new(SAMPLE_1);
        assert_eq!(puzzle.part1(), 142);
    }

    #[test]
    fn test02() {
        let puzzle = Puzzle::new(SAMPLE_2);
        assert_eq!(puzzle.part2(), 281);
    }
}
