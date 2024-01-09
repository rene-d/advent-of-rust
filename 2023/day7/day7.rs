//! [Day 7: Camel Cards](https://adventofcode.com/2023/day/7)

use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Camel {
    hand: String,
    bid: usize,
}

struct Puzzle {
    camels: Vec<Camel>,
}

impl Puzzle {
    fn new() -> Puzzle {
        Puzzle { camels: vec![] }
    }

    /// Get the puzzle input.
    fn configure(&mut self, path: &str) {
        let data = std::fs::read_to_string(path).unwrap();

        for line in data.lines() {
            let mut line = line.split_ascii_whitespace();

            let hand = line.next().unwrap().to_string();
            let bid = line.next().unwrap().parse::<usize>().unwrap();

            self.camels.push(Camel { hand, bid });
        }
    }

    /// Compute the rank of the hand.
    fn rank(hand: &str) -> usize {
        let hs = hand.chars().fold(HashMap::new(), |mut acc, c| {
            *acc.entry(c).or_insert(0) += 1;
            acc
        });
        let mut hs = hs.into_values().collect::<Vec<_>>();
        hs.sort_unstable();

        match hs.as_slice() {
            [5] => 7,             // five of a kind
            [1, 4] => 6,          // four of a kind
            [2, 3] => 5,          // full house
            [1, 1, 3] => 4,       // full
            [1, 2, 2] => 3,       // two pairs
            [1, 1, 1, 2] => 2,    // one pair
            [1, 1, 1, 1, 1] => 1, // high card
            _ => panic!(),
        }
    }

    /// Find the max rank by changing the Jocker by any other card.
    fn optimal_rank(hand: &str) -> usize {
        if !hand.contains('J') {
            return Self::rank(hand);
        }

        "23456789TQKA"
            .chars()
            .map(|j| {
                let mut new_hand = String::new();
                new_hand.reserve(5);
                for c in hand.chars() {
                    new_hand.push(if c == 'J' { j } else { c });
                }
                Self::rank(&new_hand)
            })
            .max()
            .unwrap()
    }

    /// The weight of the hand.
    fn weight(hand: &str) -> usize {
        hand.chars()
            .fold(0, |acc, x| acc * 16 + "23456789TJQKA".find(x).unwrap())
    }

    /// The weight without the Jack/Jocker.
    fn weight_no_jack(hand: &str) -> usize {
        hand.chars()
            .fold(0, |acc, x| acc * 16 + "J23456789TQKA".find(x).unwrap())
    }

    /// Solve part one.
    fn part1(&self) -> usize {
        let mut camels = self.camels.clone();

        camels
            .sort_by_key(|camel| Self::rank(&camel.hand) * 0x10_0000 + Puzzle::weight(&camel.hand));

        camels.iter().enumerate().map(|x| (x.0 + 1) * x.1.bid).sum()
    }

    /// Solve part two.
    fn part2(&self) -> usize {
        let mut camels = self.camels.clone();

        camels.sort_by_key(|camel| {
            Self::optimal_rank(&camel.hand) * 0x10_0000 + Puzzle::weight_no_jack(&camel.hand)
        });

        camels.iter().enumerate().map(|x| (x.0 + 1) * x.1.bid).sum()
    }
}

/// Test from puzzle input
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test01() {
        let mut puzzle = Puzzle::new();
        puzzle.configure("test.txt");
        assert_eq!(puzzle.part1(), 6440);
    }

    #[test]
    fn test02() {
        let mut puzzle = Puzzle::new();
        puzzle.configure("test.txt");
        assert_eq!(puzzle.part2(), 5905);
    }
}

fn main() {
    let args = aoc::parse_args();
    let mut puzzle = Puzzle::new();
    puzzle.configure(args.path.as_str());
    println!("{}", puzzle.part1());
    println!("{}", puzzle.part2());
}
