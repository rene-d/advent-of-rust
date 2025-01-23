//! [Day 9: Encoding Error](https://adventofcode.com/2020/day/9)

struct Puzzle {
    numbers: Vec<u64>,
    window: usize,
}

impl Puzzle {
    fn new(data: &str) -> Self {
        Self {
            numbers: data
                .lines()
                .map_while(|line| line.parse::<u64>().ok())
                .collect(),
            window: 25,
        }
    }

    /// Solve part one.
    fn part1(&self) -> u64 {
        'outer: for i in self.window..self.numbers.len() {
            let invalid = self.numbers[i];

            for a in &self.numbers[(i - self.window)..i] {
                for b in &self.numbers[(i - self.window)..i] {
                    if a + b == invalid {
                        continue 'outer;
                    }
                }
            }

            return invalid;
        }
        0
    }

    /// Solve part two.
    fn part2(&self) -> u64 {
        let invalid = self.part1();

        for i in 0..self.numbers.len() {
            let mut acc = 0;

            for j in i..self.numbers.len() {
                acc += self.numbers[j];
                if acc == invalid {
                    return self.numbers[i..=j].iter().min().unwrap()
                        + self.numbers[i..=j].iter().max().unwrap();
                }
                if acc > invalid {
                    break;
                }
            }
        }

        0
    }
}

/// # Panics
/// over malformed input
#[must_use]
pub fn solve(data: &str) -> (u64, u64) {
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
    fn test_part1() {
        let mut puzzle = Puzzle::new(TEST_INPUT);
        puzzle.window = 5;
        assert_eq!(puzzle.part1(), 127);
    }

    #[test]
    fn test_part2() {
        let mut puzzle = Puzzle::new(TEST_INPUT);
        puzzle.window = 5;
        assert_eq!(puzzle.part2(), 62);
    }
}
