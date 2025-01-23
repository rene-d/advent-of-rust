//! [Day 9: Rope Bridge](https://adventofcode.com/2022/day/9)

use rustc_hash::FxHashSet;

struct Puzzle {
    moves: Vec<(char, i32)>,
}

impl Puzzle {
    fn new(data: &str) -> Self {
        Self {
            moves: data
                .lines()
                .map(|line| {
                    let mut split = line.split(' ');
                    let direction = split.next().unwrap().chars().next().unwrap();
                    let n = split.next().unwrap().parse().unwrap();

                    (direction, n)
                })
                .collect(),
        }
    }

    // Solves part one
    fn part1(&self) -> usize {
        let mut tails = FxHashSet::default();
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
        let mut tails = FxHashSet::default();
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
    const TEST_INPUT_2: &str = include_str!("test02.txt");

    #[test]
    fn test01() {
        let puzzle = Puzzle::new(TEST_INPUT);
        assert_eq!(puzzle.part1(), 13);
        assert_eq!(puzzle.part2(), 1);
    }

    #[test]
    fn test02() {
        let puzzle = Puzzle::new(TEST_INPUT_2);
        assert_eq!(puzzle.part2(), 36);
    }
}
