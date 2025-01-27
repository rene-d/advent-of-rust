//! [Day 20: Grove Positioning System](https://adventofcode.com/2022/day/20)

use std::collections::VecDeque;

struct Puzzle {
    numbers: Vec<i64>,
}

impl Puzzle {
    fn new(data: &str) -> Self {
        Self {
            numbers: data.lines().map(|x| x.parse().unwrap()).collect(),
        }
    }

    // Solves part one
    fn part1(&self) -> i64 {
        self.decrypt(1, 1)
    }

    // Solve part two
    fn part2(&self) -> i64 {
        self.decrypt(811_589_153, 10)
    }

    fn decrypt(&self, key: i64, rounds: usize) -> i64 {
        let mut q = VecDeque::new();

        q.extend(self.numbers.iter().map(|x| (*x) * key).zip(0..));

        let nb = self.numbers.len();

        for _ in 0..rounds {
            for i in 0..nb {
                let mut shift = (0, 0);

                while let Some(e) = q.pop_front() {
                    if e.1 == i {
                        shift = e;
                        break;
                    }
                    q.push_back(e);
                }

                match shift.0 {
                    o if o > 0 => q.rotate_left(
                        usize::try_from(o % i64::try_from(q.len()).unwrap()).unwrap() % (nb - 1),
                    ),
                    o if o < 0 => q.rotate_right(
                        usize::try_from((-o) % i64::try_from(q.len()).unwrap()).unwrap() % (nb - 1),
                    ),
                    _ => (),
                }

                q.push_back(shift);
            }
        }

        for (v, i) in q.iter().zip(0..) {
            if v.0 == 0 {
                return q.get((i + 1000) % nb).unwrap().0
                    + q.get((i + 2000) % nb).unwrap().0
                    + q.get((i + 3000) % nb).unwrap().0;
            }
        }
        0
    }
}

#[must_use]
pub fn solve(data: &str) -> (i64, i64) {
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
        assert_eq!(puzzle.part1(), 3);
        assert_eq!(puzzle.part2(), 1_623_178_306);
    }
}
