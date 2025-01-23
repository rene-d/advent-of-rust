//! [Day 6: Tuning Trouble](https://adventofcode.com/2022/day/6)

use rustc_hash::FxHashSet;

struct Puzzle<'a> {
    /// Puzzle input
    data: &'a str,
}

impl<'a> Puzzle<'a> {
    const fn new(data: &'a str) -> Self {
        Self {
            data: data.trim_ascii(),
        }
    }

    // Solves part one
    fn part1(&self) -> usize {
        find_marker(self.data, 4)
    }

    // Solve part two
    fn part2(&self) -> usize {
        find_marker(self.data, 14)
    }
}

/// find the position of the first marker of the given length
fn find_marker(data: &str, length: usize) -> usize {
    if data.len() >= length {
        for i in 0..=(data.len() - length) {
            let mut marker = FxHashSet::default();
            for k in 0..length {
                marker.insert(data.chars().nth(i + k));
            }
            if marker.len() == length {
                return i + length;
            }
        }
    }
    0
}

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
        assert_eq!(puzzle.part1(), 7);
        assert_eq!(puzzle.part2(), 19);
    }

    #[test]
    fn test02() {
        assert_eq!(find_marker("bvwbjplbgvbhsrlpgdmjqwftvncz", 4), 5);
        assert_eq!(find_marker("nppdvjthqldpwncqszvftbrmjlhg", 4), 6);
        assert_eq!(find_marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4), 10);
        assert_eq!(find_marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4), 11);
    }

    #[test]
    fn test03() {
        assert_eq!(find_marker("bvwbjplbgvbhsrlpgdmjqwftvncz", 14), 23);
        assert_eq!(find_marker("nppdvjthqldpwncqszvftbrmjlhg", 14), 23);
        assert_eq!(find_marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 14), 29);
        assert_eq!(find_marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 14), 26);
    }
}
