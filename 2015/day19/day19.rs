//! [Day 19: Medicine for Rudolph](https://adventofcode.com/2015/day/19)

use clap::Parser;
use std::collections::HashSet;

#[derive(Parser)]
struct Args {
    /// Puzzle input
    #[arg(default_value = "input.txt")]
    path: String,
}

struct Puzzle {
    replacements: Vec<(String, String)>,
    medicine_molecule: String,
}

impl Puzzle {
    fn new() -> Puzzle {
        Puzzle {
            replacements: vec![],
            medicine_molecule: String::new(),
        }
    }

    /// Get the puzzle input.
    fn configure(&mut self, path: &str) {
        let data = std::fs::read_to_string(path).unwrap();

        for line in data.lines() {
            if let Some((a, b)) = line.split_once(" => ") {
                self.replacements.push((a.to_string(), b.to_owned()));
            } else if !line.is_empty() {
                self.medicine_molecule = line.to_owned();
            }
        }
    }

    fn replacements<'a>(
        molecule: &'a str,
        from: &'a str,
        to: &'a str,
    ) -> impl Iterator<Item = String> + 'a {
        molecule.match_indices(from).map(move |(i, _)| {
            let mut s = String::new();

            s.push_str(&molecule[..i]);
            s.push_str(to);
            s.push_str(&molecule[(i + from.len())..]);

            s
        })
    }

    /// Solve part one.
    fn part1(&self) -> usize {
        let mut molecules = HashSet::new();

        for (from, to) in &self.replacements {
            for s in Self::replacements(&self.medicine_molecule, from, to) {
                molecules.insert(s);
            }
        }

        molecules.len()
    }

    /// Solve part two.
    fn part2(&self) -> usize {
        let mut molecule = self.medicine_molecule.clone();
        for steps in 1.. {
            let next = self
                .replacements
                .iter()
                .find_map(|(from, to)| Self::replacements(&molecule, to, from).next());
            if next.is_none() {
                eprintln!("not found... steps so far: {steps}");
                break;
            }
            let next = next.unwrap();
            if next == "e" {
                return steps;
            }
            molecule = next;
        }

        // formulae
        // https://github.com/petertseng/adventofcode-rb-2015/blob/e968bc59e527e47ca9a28b313f58cc04b6f074cb/19_molecule_replacement.rb#L54
        // I don't know if there's an algorithm to solve this problem 😕

        let molecule = &self.medicine_molecule;
        let elements = molecule.chars().filter(|&c| c.is_ascii_uppercase()).count();
        let rn = molecule.matches("Rn").count();
        let y = molecule.matches('Y').count();

        elements - 2 * (rn + y) - 1
    }
}

fn main() {
    let args = Args::parse();
    let mut puzzle = Puzzle::new();
    puzzle.configure(args.path.as_str());
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
        puzzle.configure("sample_1.txt");

        puzzle.medicine_molecule = "HOH".to_string();
        assert_eq!(puzzle.part1(), 4);

        puzzle.medicine_molecule = "HOHOHO".to_string();
        assert_eq!(puzzle.part1(), 7);
    }

    #[test]
    fn test02() {
        let mut puzzle = Puzzle::new();
        puzzle.configure("sample_2.txt");

        puzzle.medicine_molecule = "HOH".to_string();
        assert_eq!(puzzle.part2(), 3);

        puzzle.medicine_molecule = "HOHOHO".to_string();
        assert_eq!(puzzle.part2(), 6);
    }
}
