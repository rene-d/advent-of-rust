//! [Day 3: Rucksack Reorganization](https://adventofcode.com/2022/day/3)

struct Puzzle {
    rucksacks: Vec<String>,
}

impl Puzzle {
    fn new(data: &str) -> Self {
        Self {
            rucksacks: data.lines().map(std::string::ToString::to_string).collect(),
        }
    }

    fn char_to_priority(c: char) -> u32 {
        match c {
            'A'..='Z' => 27 + u32::from(c) - u32::from('A'),
            _ => 1 + u32::from(c) - u32::from('a'),
        }
    }

    fn part1(&self) -> u32 {
        let mut result = 0;
        for rucksack in &self.rucksacks {
            let (first_compartment, second_compartment) = rucksack.split_at(rucksack.len() / 2);
            for c in first_compartment.chars() {
                if second_compartment.contains(c) {
                    result += Self::char_to_priority(c);
                    break;
                }
            }
        }
        result
    }

    fn part2(&self) -> u32 {
        let mut result = 0;

        // Iterate over rucksacks by triples
        for slice in self.rucksacks.chunks(3) {
            if slice.len() == 2 {
                continue;
            }
            let first = &slice[0];
            let second = &slice[1];
            let third = &slice[2];
            for c in first.chars() {
                // Look for the character in the two others
                if second.contains(c) && third.contains(c) {
                    result += Self::char_to_priority(c);
                    break;
                }
            }
        }
        result
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

    const TEST_INPUT: &str = include_str!("test.txt");

    #[test]
    fn test_puzzle() {
        let puzzle = Puzzle::new(TEST_INPUT);
        assert_eq!(puzzle.part1(), 157);
        assert_eq!(puzzle.part2(), 70);
    }
}
