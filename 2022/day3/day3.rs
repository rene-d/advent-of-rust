//! [Day 3: Rucksack Reorganization](https://adventofcode.com/2022/day/3)

struct Puzzle {
    rucksacks: Vec<String>,
}

impl Puzzle {
    const fn new() -> Self {
        Self {
            rucksacks: Vec::new(),
        }
    }

    fn char_to_priority(c: char) -> u32 {
        match c {
            'A'..='Z' => 27 + u32::from(c) - u32::from('A'),
            _ => 1 + u32::from(c) - u32::from('a'),
        }
    }

    fn configure(&mut self, path: &str) {
        let data = std::fs::read_to_string(path).unwrap();
        let lines = data.trim().lines().collect::<Vec<_>>();
        self.rucksacks = lines.iter().map(std::string::ToString::to_string).collect();
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

/// Solve the puzzle with the user input
fn main() {
    let args = aoc::parse_args();
    let mut puzzle = Puzzle::new();
    puzzle.configure(args.path.as_str());
    println!("{}", puzzle.part1());
    println!("{}", puzzle.part2());
}

/// Test from puzzle input
#[test]
fn test_puzzle() {
    let mut puzzle = Puzzle::new();
    puzzle.configure("test.txt");
    assert_eq!(puzzle.part1(), 157);
    assert_eq!(puzzle.part2(), 70);
}
