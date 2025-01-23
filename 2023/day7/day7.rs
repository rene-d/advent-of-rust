//! [Day 7: Camel Cards](https://adventofcode.com/2023/day/7)

use rustc_hash::FxHashMap;

#[derive(Debug, Clone)]
struct Camel {
    hand: String,
    bid: usize,
}

struct Puzzle {
    camels: Vec<Camel>,
}

impl Puzzle {
    fn new(data: &str) -> Self {
        Self {
            camels: data
                .lines()
                .map(|line| {
                    let mut line = line.split_ascii_whitespace();

                    let hand = line.next().unwrap().to_string();
                    let bid = line.next().unwrap().parse::<usize>().unwrap();

                    Camel { hand, bid }
                })
                .collect(),
        }
    }

    /// Compute the rank of the hand.
    fn rank(hand: &str) -> usize {
        let hs = hand.chars().fold(FxHashMap::default(), |mut acc, c| {
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

        camels.sort_by_key(|camel| Self::rank(&camel.hand) * 0x10_0000 + Self::weight(&camel.hand));

        camels.iter().enumerate().map(|x| (x.0 + 1) * x.1.bid).sum()
    }

    /// Solve part two.
    fn part2(&self) -> usize {
        let mut camels = self.camels.clone();

        camels.sort_by_key(|camel| {
            Self::optimal_rank(&camel.hand) * 0x10_0000 + Self::weight_no_jack(&camel.hand)
        });

        camels.iter().enumerate().map(|x| (x.0 + 1) * x.1.bid).sum()
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

    const TEST_INPUT: &str = include_str!("test.txt");

    #[test]
    fn test01() {
        let puzzle = Puzzle::new(TEST_INPUT);
        assert_eq!(puzzle.part1(), 6440);
    }

    #[test]
    fn test02() {
        let puzzle = Puzzle::new(TEST_INPUT);
        assert_eq!(puzzle.part2(), 5905);
    }
}
