//! [Day 9: Marble Mania](https://adventofcode.com/2018/day/9)

use rustc_hash::FxHashMap;
use std::collections::VecDeque;

fn play_slow(nb_players: u32, nb_marbles: u32) -> u32 {
    let mut marbles = vec![0u32];
    let mut scores = vec![0u32; nb_players as usize];

    let mut current = 1;

    for i in 1..=nb_marbles {
        if i % 23 == 0 {
            current = (current + marbles.len() - 7) % marbles.len();
            scores[(i % nb_players) as usize] += i + marbles[current];
            marbles.remove(current);
        } else {
            current = (current + 2) % marbles.len();
            marbles.insert(current, i);
        }
    }
    *scores.iter().max().unwrap()
}

fn play_fast(nb_players: u32, marbles: u32) -> u32 {
    let mut scores = FxHashMap::default();
    let mut circle = VecDeque::with_capacity(usize::try_from(marbles).unwrap());
    circle.push_back(0);
    for (m, p) in (1..=marbles).zip((0..nb_players).cycle()) {
        if m % 23 == 0 {
            circle.rotate_right(8);
            *scores.entry(p).or_default() += m + circle.pop_front().unwrap();
            circle.rotate_left(1);
        } else {
            circle.rotate_left(1);
            circle.push_back(m);
        }
    }
    scores.values().copied().max().unwrap()
}

struct Puzzle {
    elves: u32,
    points: u32,
}

impl Puzzle {
    fn new(data: &str) -> Self {
        let row = data.split_ascii_whitespace().collect::<Vec<_>>();
        match &row[..] {
            [n, _, _, _, _, _, p, _] => Self {
                elves: n.parse().unwrap(),
                points: p.parse().unwrap(),
            },
            _ => panic!("bad input: {data:?}"),
        }
    }

    /// Solve part one.
    fn part1(&self) -> u32 {
        play_slow(self.elves, self.points)
    }

    /// Solve part two.
    fn part2(&self) -> u32 {
        play_fast(self.elves, self.points * 100)
    }
}

/// # Panics
#[must_use]
pub fn solve(data: &str) -> (u32, u32) {
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

    #[test]
    fn test01() {
        let puzzle = Puzzle::new("9 x x x x x 25 x");
        assert_eq!(puzzle.part1(), 32);
    }

    #[test]
    fn test02() {
        assert_eq!(play_fast(10, 1618), 8317);
        assert_eq!(play_fast(13, 7999), 146_373);
        assert_eq!(play_fast(21, 6111), 54718);
        assert_eq!(play_fast(30, 5807), 37305);
    }

    #[test]
    fn test03() {
        assert_eq!(play_slow(10, 1618), 8317);
        assert_eq!(play_slow(13, 7999), 146_373);
        assert_eq!(play_slow(21, 6111), 54718);
        assert_eq!(play_slow(30, 5807), 37305);
    }
}
