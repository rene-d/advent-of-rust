//! [Day 21: Step Counter](https://adventofcode.com/2023/day/21)

use num::Integer;
use rustc_hash::FxHashSet;

struct Puzzle {
    garden: aoc::Grid<u8>,
    start: aoc::Coord,
    n: i32,
}

impl Puzzle {
    fn new(data: &str) -> Self {
        let garden = aoc::Grid::<u8>::parse(data);

        let start = garden.iter().find(|(_, c)| **c == b'S').unwrap().0;

        let n = garden.width();
        assert_eq!(n, garden.height());

        Self { garden, start, n }
    }

    fn is_garden(&self, x: i32, y: i32) -> bool {
        let x = x.rem_euclid(self.n);
        let y = y.rem_euclid(self.n);
        self.garden[(x, y)] != b'#'
    }

    fn count(&self, n: i32) -> u64 {
        // nota: still not really optimized, could probably memoize something
        let mut p = FxHashSet::default();
        p.insert((self.start.x, self.start.y));

        for _ in 0..n {
            let mut np = FxHashSet::default();

            for (x, y) in p {
                if self.is_garden(x - 1, y) {
                    np.insert((x - 1, y));
                }
                if self.is_garden(x + 1, y) {
                    np.insert((x + 1, y));
                }
                if self.is_garden(x, y - 1) {
                    np.insert((x, y - 1));
                }
                if self.is_garden(x, y + 1) {
                    np.insert((x, y + 1));
                }
            }

            p = np;
        }

        p.len() as u64
    }

    fn big_count(&self, n: i32) -> u64 {
        // the step count curve is parabolic
        let (t, x0) = n.div_rem(&self.n);

        let y0 = self.count(x0);
        let y1 = self.count(x0 + self.n);
        let y2 = self.count(x0 + self.n * 2);

        // println!("f(x) = a⋅x² + b⋅x + c");
        // println!("x0={x0} → y0={y0}");
        // println!("x1={} → y1={y1}", x0 + self.n);
        // println!("x2={} → y2={y2}", x0 + self.n * 2);

        // let y3 = self.count(x0 + self.n * 3);
        // println!("x3={} → y3={y3}", x0 + self.n * 3);

        let a = y2 - 2 * y1 + y0;
        let b = y1 - y0;

        let t = u64::try_from(t).unwrap();

        a * t * (t - 1) / 2 + b * t + y0
    }

    /// Solve part one.
    fn part1(&self) -> u64 {
        self.count(64)
    }

    /// Solve part two.
    fn part2(&self) -> u64 {
        self.big_count(26_501_365)
    }
}

/// # Panics
#[must_use]
pub fn solve(data: &str) -> (u64, u64) {
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
        assert_eq!(puzzle.count(6), 16);
    }

    #[test]
    fn test02() {
        let puzzle = Puzzle::new(TEST_INPUT);
        assert_eq!(puzzle.count(10), 50);
        assert_eq!(puzzle.count(50), 1594);
        assert_eq!(puzzle.count(100), 6536);
        // assert_eq!(puzzle.big_count(500), 167004);
        // assert_eq!(puzzle.big_count(1000), 668697);
        // assert_eq!(puzzle.big_count(5000), 16733044);
    }
}
