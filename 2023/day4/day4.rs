//! [Day 4: Scratchcards](https://adventofcode.com/2023/day/4)

use rustc_hash::FxHashSet;

struct Puzzle {
    matching_cards: Vec<usize>,
}

impl Puzzle {
    const fn new() -> Self {
        Self {
            matching_cards: vec![],
        }
    }

    /// Get the puzzle input.
    fn configure(&mut self, data: &str) {
        for line in data.lines() {
            let line = line[line.find(':').unwrap() + 1..]
                .split('|')
                .collect::<Vec<_>>();

            let winning = line[0]
                .split_whitespace()
                .map(|x| x.parse::<u32>().unwrap())
                .collect::<FxHashSet<_>>();

            let nums = line[1]
                .split_whitespace()
                .map(|x| x.parse::<u32>().unwrap())
                .collect::<FxHashSet<_>>();

            let result = winning.intersection(&nums).count();

            self.matching_cards.push(result);
        }
    }

    /// Solve part one.
    fn part1(&self) -> usize {
        let mut sum = 0;
        for n in &self.matching_cards {
            if n >= &1 {
                sum += 2usize.pow(u32::try_from(*n).unwrap() - 1);
            }
        }
        sum
    }

    /// Solve part two.
    fn part2(&self) -> usize {
        let mut copies = vec![0usize; self.matching_cards.len()];

        for i in 0..self.matching_cards.len() {
            copies[i] += 1;

            let m = self.matching_cards[i];
            for j in (i + 1)..(i + 1 + m) {
                copies[j] += copies[i];
            }
        }

        copies.iter().sum()
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
        assert_eq!(puzzle.part1(), 13);
    }

    #[test]
    fn test02() {
        let mut puzzle = Puzzle::new();
        puzzle.configure(&aoc::load_input_data("test.txt"));
        assert_eq!(puzzle.part2(), 30);
    }
}
