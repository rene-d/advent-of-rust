//! [Day 5: Alchemical Reduction](https://adventofcode.com/2018/day/5)

// Return true if same letter and different cases
const fn react(a: u8, b: u8) -> bool {
    a ^ b == 32 // <=> a.to_ascii_lowercase() == b.to_ascii_lowercase() && a.is_lowercase() ^ b.is_lowercase()
}

fn react_polymer(polymer: &[u8]) -> usize {
    let mut reacted = Vec::with_capacity(polymer.len());

    let mut last = 0;

    for &unit in polymer {
        if react(last, unit) {
            last = reacted.pop().unwrap_or(0);
        } else {
            if last != 0 {
                reacted.push(last);
            }
            last = unit;
        }
    }

    if last != 0 {
        reacted.push(last);
    }

    reacted.len()
}

struct Puzzle {
    polymer: Vec<u8>,
}

impl Puzzle {
    const fn new() -> Self {
        Self { polymer: vec![] }
    }

    /// Get the puzzle input.
    fn configure(&mut self, data: &str) {
        self.polymer = data.trim().bytes().collect();
    }

    /// Solve part one.
    fn part1(&self) -> usize {
        react_polymer(&self.polymer)
    }

    /// Solve part two.
    fn part2(&self) -> usize {
        (b'a'..=b'z')
            .map(|unit| {
                let mut polymer = self.polymer.clone();
                polymer.retain(|c| (c | 32) != unit); // c|32 <=> c.to_ascii_lowercase()

                react_polymer(&polymer)
            })
            .min()
            .unwrap()
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
        assert_eq!(puzzle.part1(), 10);
    }

    #[test]
    fn test02() {
        let mut puzzle = Puzzle::new();
        puzzle.configure(&aoc::load_input_data("test.txt"));
        assert_eq!(puzzle.part2(), 4);
    }
}
