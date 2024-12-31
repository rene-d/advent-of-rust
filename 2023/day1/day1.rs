//! [Day 1: Trebuchet?!](https://adventofcode.com/2023/day/1)

struct Puzzle {
    data: String,
}

impl Puzzle {
    const fn new() -> Self {
        Self {
            data: String::new(),
        }
    }

    /// Get the puzzle input.
    fn configure(&mut self, data: &str) {
        self.data = data.to_string();
    }

    /// Solve part one.
    fn part1(&self) -> u32 {
        let mut sum = 0;
        for line in self.data.split_terminator('\n') {
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
        for line in self.data.split_terminator('\n') {
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
        let mut puzzle = Puzzle::new();
        puzzle.configure(&aoc::load_input_data("sample_1.txt"));
        assert_eq!(puzzle.part1(), 142);
    }

    #[test]
    fn test02() {
        let mut puzzle = Puzzle::new();
        puzzle.configure(&aoc::load_input_data("sample_2.txt"));
        assert_eq!(puzzle.part2(), 281);
    }
}
