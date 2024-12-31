//! [Day 5: Print Queue](https://adventofcode.com/2024/day/5)

use std::collections::{HashMap, HashSet};

struct Puzzle {
    ordering_rules: HashMap<i32, HashSet<i32>>,
    page_updates: Vec<Vec<i32>>,
}

impl Puzzle {
    fn new() -> Self {
        Self {
            ordering_rules: HashMap::new(),
            page_updates: Vec::new(),
        }
    }

    /// Get the puzzle input.
    fn configure(&mut self, data: &str) {
        let data = data.split_once("\n\n").unwrap();

        //
        for line in data.0.lines() {
            let (p1, p2) = line.split_once('|').unwrap();

            let p1 = p1.parse().unwrap();
            let p2 = p2.parse().unwrap();

            self.ordering_rules.entry(p1).or_default().insert(p2);
        }

        //
        for line in data.1.lines() {
            let v: Vec<_> = line.split(',').map(|x| x.parse::<i32>().unwrap()).collect();
            self.page_updates.push(v);
        }
    }

    /// Solve part one.
    fn part1(&self) -> i32 {
        let mut result = 0;

        for pu in &self.page_updates {
            let mut correctly_ordered = true;

            for (i, page) in pu.iter().enumerate() {
                if let Some(or) = self.ordering_rules.get(page) {
                    let hs: HashSet<i32> = pu.iter().copied().take(i).collect();

                    if hs.intersection(or).count() != 0 {
                        correctly_ordered = false;
                        break;
                    }
                }
            }

            if correctly_ordered {
                result += pu[pu.len() / 2];
            }
        }

        result
    }

    /// Solve part two.
    fn part2(&self) -> i32 {
        self.page_updates
            .iter()
            .map(|x| self.bubble_sort_updates(x))
            .sum()
    }

    /// Sort the page updates.
    /// If a fix has been made, return the middle page number.
    /// Return 0 otherwise.
    fn bubble_sort_updates(&self, updates: &[i32]) -> i32 {
        let mut pu = updates.to_vec();
        let mut reorder = false;
        let mut i = 0;

        while i < pu.len() {
            let page = pu[i];

            if let Some(or) = self.ordering_rules.get(&page) {
                let hs: HashSet<i32> = pu.iter().copied().take(i).collect();

                if hs.intersection(or).count() != 0 {
                    pu.swap(i - 1, i);

                    i = 0;
                    reorder = true;
                    continue;
                }
            }

            i += 1;
        }

        if reorder {
            pu[pu.len() / 2]
        } else {
            0
        }
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
        let data = aoc::load_input_data("test.txt");
        puzzle.configure(&data);
        assert_eq!(puzzle.part1(), 143);
    }

    #[test]
    fn test02() {
        let mut puzzle = Puzzle::new();
        let data = aoc::load_input_data("test.txt");
        puzzle.configure(&data);
        assert_eq!(puzzle.part2(), 123);
    }
}
