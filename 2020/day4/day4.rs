//! [Day 4: Passport Processing](https://adventofcode.com/2020/day/4)

use rustc_hash::FxHashSet;

fn validate_field(field: &str, value: &str) -> bool {
    match (field, value.len()) {
        ("byr", 4) => {
            let byr: u32 = value.parse().unwrap_or(0);
            (1920..=2002).contains(&byr)
        }
        ("iyr", 4) => {
            let iyr: u32 = value.parse().unwrap_or(0);
            (2010..=2020).contains(&iyr)
        }
        ("eyr", 4) => {
            let eyr: u32 = value.parse().unwrap_or(0);
            (2020..=2030).contains(&eyr)
        }
        ("hgt", _) => value.strip_suffix("in").map_or_else(
            || {
                value.strip_suffix("cm").map_or(false, |centimeters| {
                    let height: u8 = centimeters.parse().unwrap_or(0);
                    (150..=193).contains(&height)
                })
            },
            |inches| {
                let height: u8 = inches.parse().unwrap_or(0);
                (59..=76).contains(&height)
            },
        ),
        ("hcl", 7) => {
            value.starts_with('#')
                && value
                    .chars()
                    .skip(1)
                    .all(|c| "abcdef0123456789".contains(c))
        }
        ("ecl", 3) => ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&value),
        ("pid", 9) => value.chars().all(|c| c.is_ascii_digit()),
        _ => false,
    }
}

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
        let mandatory_fields: FxHashSet<_> = ["eyr", "iyr", "byr", "ecl", "pid", "hcl", "hgt"]
            .iter()
            .copied()
            .collect();

        self.data
            .split("\n\n")
            .map(|record| {
                let mut fields = FxHashSet::default();
                for item in record.split_ascii_whitespace() {
                    let (field, _) = item.split_once(':').unwrap();
                    fields.insert(field);
                }
                u32::from(fields.is_superset(&mandatory_fields))
            })
            .sum()
    }

    /// Solve part two.
    fn part2(&self) -> u32 {
        let mandatory_fields: FxHashSet<_> = ["eyr", "iyr", "byr", "ecl", "pid", "hcl", "hgt"]
            .iter()
            .copied()
            .collect();

        self.data
            .split("\n\n")
            .map(|record| {
                let mut fields = FxHashSet::default();
                for item in record.split_ascii_whitespace() {
                    let (field, value) = item.split_once(':').unwrap();

                    if validate_field(field, value) {
                        fields.insert(field);
                    }
                }
                u32::from(fields.is_superset(&mandatory_fields))
            })
            .sum()
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
    fn test_part1() {
        let mut puzzle = Puzzle::new();
        puzzle.configure(&aoc::load_input_data("sample_1.txt"));
        assert_eq!(puzzle.part1(), 2);
    }

    #[test]
    fn test_part2_invalid() {
        let mut puzzle = Puzzle::new();
        puzzle.configure(&aoc::load_input_data("sample_3.txt"));
        assert_eq!(puzzle.part2(), 0);
    }

    #[test]
    fn test_part2_valid() {
        let mut puzzle = Puzzle::new();
        puzzle.configure(&aoc::load_input_data("sample_4.txt"));
        assert_eq!(puzzle.part2(), 4);
    }
}
