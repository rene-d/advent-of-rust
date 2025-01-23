//! [Day 19: Medicine for Rudolph](https://adventofcode.com/2015/day/19)

use rustc_hash::FxHashSet;

struct Puzzle {
    replacements: Vec<(String, String)>,
    medicine_molecule: String,
}

impl Puzzle {
    fn new(data: &str) -> Self {
        let mut replacements = vec![];
        let mut medicine_molecule = String::new();

        for line in data.lines() {
            if let Some((a, b)) = line.split_once(" => ") {
                replacements.push((a.to_string(), b.to_owned()));
            } else if !line.is_empty() {
                line.clone_into(&mut medicine_molecule);
            }
        }

        Self {
            replacements,
            medicine_molecule,
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
        let mut molecules = FxHashSet::default();

        for (from, to) in &self.replacements {
            for s in Self::replacements(&self.medicine_molecule, from, to) {
                molecules.insert(s);
            }
        }

        molecules.len()
    }

    /// Solve part two.
    fn part2(&self) -> usize {
        // formulae
        // https://github.com/petertseng/adventofcode-rb-2015/blob/e968bc59e527e47ca9a28b313f58cc04b6f074cb/19_molecule_replacement.rb#L54
        // I don't know if there's an algorithm to solve this problem 😕

        let molecule = &self.medicine_molecule;
        let max_e = &self
            .replacements
            .iter()
            .filter_map(|r| (r.0 == "e").then_some(&r.1))
            .map(|r| r.chars().filter(|&c| c.is_ascii_uppercase()).count())
            .max();
        let elements = molecule.chars().filter(|&c| c.is_ascii_uppercase()).count();
        let rn = molecule.matches("Rn").count();
        let y = molecule.matches('Y').count();
        let ar = molecule.matches("Ar").count();
        assert_eq!(rn, ar);

        let formulae = elements - (max_e.unwrap_or(0) - 1) - rn - ar - y * 2;

        let mut molecule = self.medicine_molecule.clone();
        for steps in 1.. {
            let next = self
                .replacements
                .iter()
                .find_map(|(from, to)| Self::replacements(&molecule, to, from).next());
            if next.is_none() {
                eprintln!("not found... steps so far: {steps}");
                return formulae;
            }
            let next = next.unwrap();
            if next == "e" {
                return steps;
            }
            molecule = next;
        }

        unreachable!();
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

    const SAMPLE_1: &str = include_str!("sample_1.txt");
    const SAMPLE_2: &str = include_str!("sample_2.txt");

    #[test]
    fn test01() {
        let mut puzzle = Puzzle::new(SAMPLE_1);

        puzzle.medicine_molecule = "HOH".to_string();
        assert_eq!(puzzle.part1(), 4);

        puzzle.medicine_molecule = "HOHOHO".to_string();
        assert_eq!(puzzle.part1(), 7);
    }

    #[test]
    fn test02() {
        let mut puzzle = Puzzle::new(SAMPLE_2);

        puzzle.medicine_molecule = "HOH".to_string();
        assert_eq!(puzzle.part2(), 3);

        puzzle.medicine_molecule = "HOHOHO".to_string();
        assert_eq!(puzzle.part2(), 6);
    }
}
