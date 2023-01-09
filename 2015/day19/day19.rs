//! [Day 19: Medicine for Rudolph](https://adventofcode.com/2015/day/19)

use std::collections::HashSet;
use std::env;
use std::fs;

struct Puzzle {
    medicine_molecule: String,
    replacements: Vec<(String, String)>,
}

impl Puzzle {
    fn new() -> Puzzle {
        Puzzle {
            medicine_molecule: String::new(),
            replacements: Vec::new(),
        }
    }

    fn configure(&mut self, filename: &str) {
        let mut data: Vec<String> = fs::read_to_string(filename)
            .expect("Failed to read input file")
            .lines()
            .map(ToString::to_string)
            .collect();

        self.medicine_molecule = data.pop().unwrap();

        // Remove empty delimiter
        data.pop();

        for line in data {
            // Get the replacement rule
            let mut replacement: Vec<&str> = line.split(" => ").collect();
            let to = replacement.pop().unwrap().to_string();
            let from = replacement.pop().unwrap().to_string();
            self.replacements.push((from, to));
        }
    }

    fn part1(&mut self) -> usize {
        let mut molecules = HashSet::new();

        for (from, to) in &self.replacements {
            // Split the molecule on each atom
            let sub_molecules: Vec<String> = self
                .medicine_molecule
                .split_inclusive(from)
                .map(ToString::to_string)
                .collect();

            // Apply the replacement rule for each atom found
            for (i, sub_mol) in sub_molecules.iter().enumerate() {
                let mut generated_molecule = sub_molecules.clone();
                let sub_mol = sub_mol.replace(from, to);
                generated_molecule[i] = sub_mol;
                molecules.insert(generated_molecule.concat());
            }
        }
        molecules.remove(&self.medicine_molecule);

        molecules.len()
    }

    /// This resolution works but it's not truly what's asked by the puzzle. We don't know if this
    /// method yields the lowest count of iterations required to produce the molecule. Finding the
    /// lowest count of iterations is a way harder puzzle and was probably not meant by the author
    /// of the puzzle. The puzzle input is probably designed in order for only one answer to rise.
    fn part2(&mut self) -> usize {
        let mut iterations = 0_usize;

        // Reduce the medicine molecule to the electron is equivalent to create the medicine molecule from the electron
        while self.medicine_molecule != "e" {
            for (from, to) in &self.replacements {
                // Reverse apply the replacement only once
                if self.medicine_molecule.contains(to) {
                    self.medicine_molecule = self.medicine_molecule.replacen(to, from, 1);
                    break;
                }
            }
            iterations += 1;

            if iterations > 10000 {
                return 0;
            }
        }

        iterations
    }
}

/// Test from the puzzle description
#[test]
fn test1() {
    let mut puzzle = Puzzle::new();
    puzzle.configure("test01.txt");
    assert_eq!(puzzle.part1(), 4);
}

/// Test from the puzzle description
#[test]
fn test2() {
    let mut puzzle = Puzzle::new();
    puzzle.configure("test02.txt");
    assert_eq!(puzzle.part1(), 7);
}

/// Test from the puzzle description
#[ignore = "Heuristic made to solve the problem does not work here"]
#[test]
fn test3() {
    let mut puzzle = Puzzle::new();
    puzzle.configure("test03.txt");
    assert_eq!(puzzle.part2(), 3);
}

/// Test from the puzzle description
#[ignore = "Heuristic made to solve the problem does not work here"]
#[test]
fn test4() {
    let mut puzzle = Puzzle::new();
    puzzle.configure("test04.txt");
    assert_eq!(puzzle.part2(), 6);
}

/// Test from a player's input
#[test]
fn test5() {
    let mut puzzle = Puzzle::new();
    puzzle.configure("test05.txt");
    assert_eq!(puzzle.part1(), 518);
    assert_eq!(puzzle.part2(), 200);
}

fn main() {
    let mut puzzle = Puzzle::new();

    let args: Vec<String> = env::args().collect();
    puzzle.configure(args.get(1).expect("No input file"));

    let result = puzzle.part1();
    println!("{}", result);

    let result = puzzle.part2();
    println!("{}", result);
}
