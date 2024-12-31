//! [Day 5: Alchemical Reduction](https://adventofcode.com/2018/day/5)

// Return true if same letter and different cases
fn react(a: char, b: char) -> bool {
    a.to_ascii_lowercase() == b.to_ascii_lowercase() && a.is_lowercase() ^ b.is_lowercase()
}

fn react_polymer(polymer: &[char]) -> usize {
    let mut polymer = polymer.to_owned();
    let mut i = 0;

    while i < polymer.len() - 1 {
        if react(polymer[i], polymer[i + 1]) {
            polymer.remove(i);
            polymer.remove(i);

            i = i.saturating_sub(1);
        } else {
            i += 1;
        }
    }

    polymer.len()
}

struct Puzzle {
    polymer: Vec<char>,
}

impl Puzzle {
    const fn new() -> Self {
        Self { polymer: vec![] }
    }

    /// Get the puzzle input.
    fn configure(&mut self, data: &str) {
        self.polymer = data.trim().chars().collect();
    }

    /// Solve part one.
    fn part1(&self) -> usize {
        react_polymer(&self.polymer)
    }

    /// Solve part two.
    fn part2(&self) -> usize {
        ('a'..='z')
            .map(|unit| {
                let mut polymer = self.polymer.clone();
                polymer.retain(|c| c.to_ascii_lowercase() != unit);

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
