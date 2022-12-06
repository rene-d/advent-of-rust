//! [Day 6: Tuning Trouble](https://adventofcode.com/2022/day/6)

use std::collections::HashSet;

struct Puzzle {
    data: String,
}

impl Puzzle {
    fn new() -> Puzzle {
        Puzzle {
            data: String::new(),
        }
    }

    /// load data from input
    fn configure(&mut self, path: &str) {
        self.data = std::fs::read_to_string(path).unwrap();
    }

    fn part1(&self) -> usize {
        find_marker(&self.data, 4)
    }

    fn part2(&self) -> usize {
        find_marker(&self.data, 14)
    }
}

/// find the position of the first marker of the given length
fn find_marker(data: &str, length: usize) -> usize {
    for i in 0..=(data.len() - length) {
        let mut marker = HashSet::new();
        for k in 0..length {
            marker.insert(data.chars().nth(i + k));
        }
        if marker.len() == length {
            return i + length;
        }
    }
    0
}

/// main function
fn main() {
    let mut puzzle = Puzzle::new();
    puzzle.configure("input.txt");
    println!("{}", puzzle.part1());
    println!("{}", puzzle.part2());
}

#[test]
fn test01() {
    let mut puzzle = Puzzle::new();
    puzzle.configure("test.txt");
    assert_eq!(puzzle.part1(), 7);
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
    assert_eq!(find_marker("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14), 19);
    assert_eq!(find_marker("bvwbjplbgvbhsrlpgdmjqwftvncz", 14), 23);
    assert_eq!(find_marker("nppdvjthqldpwncqszvftbrmjlhg", 14), 23);
    assert_eq!(find_marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 14), 29);
    assert_eq!(find_marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 14), 26);
}
