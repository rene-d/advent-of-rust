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
    const fn new() -> Self {
        Self { data: vec![] }
    }

    /// Get the puzzle input.
    fn configure(&mut self, data: &str) {
        for line in data.lines() {
            let row: Vec<_> = line.split(['-', ':', ' ']).collect();
            assert_eq!(row[3], "");

            self.data.push(PolicyPassword {
                min: row[0].parse().unwrap(),
                max: row[1].parse().unwrap(),
                letter: row[2].chars().nth(0).unwrap(),
                password: row[4].to_string(),
            });
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
        puzzle.configure(&aoc::load_input_data("test.txt"));
        assert_eq!(puzzle.part1(), 2);
    }

    #[test]
    fn test02() {
        let mut puzzle = Puzzle::new();
        puzzle.configure(&aoc::load_input_data("test.txt"));
        assert_eq!(puzzle.part2(), 1);
    }
}
