//! [Day 9: Rope Bridge](https://adventofcode.com/2022/day/9)

use std::collections::HashSet;

struct Puzzle {
    moves: Vec<(char, i32)>,
}

impl Puzzle {
    const fn new() -> Self {
        Self { moves: vec![] }
    }

    /// Loads data from input (one line)
    fn configure(&mut self, path: &str) {
        let data = std::fs::read_to_string(path).unwrap();
        for line in data.split('\n') {
            if !line.is_empty() {
                let mut split = line.split(' ');
                let direction = split.next().unwrap().chars().next().unwrap();
                let n = split.next().unwrap().parse().unwrap();
                self.moves.push((direction, n));
            }
        }
    }

    // Solves part one
    fn part1(&self) -> usize {
        let mut tails = HashSet::new();
        let mut head = (0, 0);
        let mut tail = (0, 0);

        for m in &self.moves {
            let direction = m.0;
            let n = m.1;
            for _ in 0..n {
                Self::move_segment(direction, &mut head, &mut tail);

                tails.insert(tail);
            }
        }
        tails.len()
    }

    // Solve part two
    fn part2(&self) -> usize {
        let mut tails = HashSet::new();
        let mut rope = [
            (0, 0),
            (0, 0),
            (0, 0),
            (0, 0),
            (0, 0),
            (0, 0),
            (0, 0),
            (0, 0),
            (0, 0),
            (0, 0),
        ];

        for m in &self.moves {
            let direction = m.0;
            let n = m.1;
            for _ in 0..n {
                Self::move_rope(direction, &mut rope);
                tails.insert(rope.last().copied().unwrap());
            }
        }
        tails.len()
    }

    /// Move a multi-segment rope
    fn move_rope(direction: char, rope: &mut [(i32, i32)]) {
        for k in 0..rope.len() - 1 {
            let mut head = rope[k];
            let mut tail = rope[k + 1];

            Self::move_segment(if k == 0 { direction } else { ' ' }, &mut head, &mut tail);

            rope[k] = head;
            rope[k + 1] = tail;
        }
    }

    /// Move a segment
    fn move_segment(direction: char, head: &mut (i32, i32), tail: &mut (i32, i32)) {
        // head movement
        let (dx, dy) = match direction {
            'R' => (1, 0),
            'L' => (-1, 0),
            'U' => (0, 1),
            'D' => (0, -1),
            _ => (0, 0),
        };
        head.0 += dx;
        head.1 += dy;

        // tail movement
        let dx = if head.0 - tail.0 > 0 { 1 } else { -1 };
        let dy = if head.1 - tail.1 > 0 { 1 } else { -1 };
        if tail.0 == head.0 {
            if (tail.1 - head.1).abs() > 1 {
                tail.1 += dy;
            }
        } else if tail.1 == head.1 {
            if (tail.0 - head.0).abs() > 1 {
                tail.0 += dx;
            }
        } else if (tail.0 - head.0).abs() + (tail.1 - head.1).abs() > 2 {
            tail.0 += dx;
            tail.1 += dy;
        }
    }
}

/// main function
fn main() {
    let args = aoc::parse_args();
    let mut puzzle = Puzzle::new();
    puzzle.configure(&args.path);
    println!("{}", puzzle.part1());
    println!("{}", puzzle.part2());
}

#[test]
fn test01() {
    let mut puzzle = Puzzle::new();
    puzzle.configure("test.txt");
    assert_eq!(puzzle.part1(), 13);
    assert_eq!(puzzle.part2(), 1);
}

#[test]
fn test02() {
    let mut puzzle = Puzzle::new();
    puzzle.configure("test02.txt");
    assert_eq!(puzzle.part2(), 36);
}
