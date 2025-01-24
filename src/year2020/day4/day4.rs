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
                value.strip_suffix("cm").is_some_and(|centimeters| {
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

struct Puzzle<'a> {
    data: &'a str,
}

impl<'a> Puzzle<'a> {
    const fn new(data: &'a str) -> Self {
        Self { data }
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

/// # Panics
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
    const SAMPLE_3: &str = include_str!("sample_3.txt");
    const SAMPLE_4: &str = include_str!("sample_4.txt");

    #[test]
    fn test_part1() {
        let puzzle = Puzzle::new(SAMPLE_1);
        assert_eq!(puzzle.part1(), 2);
    }

    #[test]
    fn test_part2_invalid() {
        let puzzle = Puzzle::new(SAMPLE_3);
        assert_eq!(puzzle.part2(), 0);
    }

    #[test]
    fn test_part2_valid() {
        let puzzle = Puzzle::new(SAMPLE_4);
        assert_eq!(puzzle.part2(), 4);
    }
}
