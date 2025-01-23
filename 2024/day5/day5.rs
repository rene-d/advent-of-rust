//! [Day 5: Print Queue](https://adventofcode.com/2024/day/5)

use rustc_hash::{FxHashMap, FxHashSet};

struct Puzzle {
    ordering_rules: FxHashMap<i32, FxHashSet<i32>>,
    page_updates: Vec<Vec<i32>>,
}

impl Puzzle {
    fn new(data: &str) -> Self {
        let mut ordering_rules: FxHashMap<i32, FxHashSet<i32>> = FxHashMap::default();
        let mut page_updates = Vec::new();

        let data = data.split_once("\n\n").unwrap();

        //
        for line in data.0.lines() {
            let (p1, p2) = line.split_once('|').unwrap();

            let p1 = p1.parse().unwrap();
            let p2 = p2.parse().unwrap();

            ordering_rules.entry(p1).or_default().insert(p2);
        }

        //
        for line in data.1.lines() {
            let v: Vec<_> = line.split(',').map(|x| x.parse::<i32>().unwrap()).collect();
            page_updates.push(v);
        }

        Self {
            ordering_rules,
            page_updates,
        }
    }

    /// Solve part one.
    fn part1(&self) -> i32 {
        let mut result = 0;

        for pu in &self.page_updates {
            let mut correctly_ordered = true;

            for (i, page) in pu.iter().enumerate() {
                if let Some(or) = self.ordering_rules.get(page) {
                    let hs: FxHashSet<i32> = pu.iter().copied().take(i).collect();

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
                let hs: FxHashSet<i32> = pu.iter().copied().take(i).collect();

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

/// # Panics
/// over malformed input
#[must_use]
pub fn solve(data: &str) -> (i32, i32) {
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
    fn test01() {
        let puzzle = Puzzle::new(TEST_INPUT);
        assert_eq!(puzzle.part1(), 143);
    }

    #[test]
    fn test02() {
        let puzzle = Puzzle::new(TEST_INPUT);
        assert_eq!(puzzle.part2(), 123);
    }
}
