//! [Day 2: Password Philosophy](https://adventofcode.com/2020/day/2)

struct PolicyPassword {
    min: usize,
    max: usize,
    letter: char,
    password: String,
}

struct Puzzle {
    data: Vec<PolicyPassword>,
}

impl Puzzle {
    fn new(data: &str) -> Self {
        Self {
            data: data
                .lines()
                .map(|line| {
                    let row: Vec<_> = line.split(['-', ':', ' ']).collect();
                    assert_eq!(row[3], "");

                    PolicyPassword {
                        min: row[0].parse().unwrap(),
                        max: row[1].parse().unwrap(),
                        letter: row[2].chars().nth(0).unwrap(),
                        password: row[4].to_string(),
                    }
                })
                .collect(),
        }
    }

    /// Solve part one.
    fn part1(&self) -> usize {
        self.data
            .iter()
            .filter(|pp| {
                let n = pp.password.matches(pp.letter).count();
                (pp.min..=pp.max).contains(&n)
            })
            .count()
    }

    /// Solve part two.
    fn part2(&self) -> usize {
        self.data
            .iter()
            .filter(|pp| {
                // if (password[a - 1] == letter and password[b - 1] != letter) or (
                //  password[a - 1] != letter and password[b - 1] == letter

                let a = pp.password.chars().nth(pp.min - 1).unwrap_or('?');
                let b = pp.password.chars().nth(pp.max - 1).unwrap_or('?');

                (a == pp.letter && b != pp.letter) || (a != pp.letter && b == pp.letter)
            })
            .count()
    }
}

/// # Panics
/// over malformed input
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
        assert_eq!(puzzle.part2(), 1);
    }
}
