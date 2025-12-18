//! [Day 21: Step Counter](https://adventofcode.com/2023/day/21)

const DIRS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

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

    #[inline]
    fn is_garden(&self, x: i32, y: i32) -> bool {
        let x = x.rem_euclid(self.n);
        let y = y.rem_euclid(self.n);
        self.garden[(x, y)] != b'#'
    }

    fn count(&self, n: i32) -> u64 {
        // BFS optimization:
        // Since we only need up to ~330 steps for the interpolation,
        // we can use a dense array with an offset instead of a HashSet.
        // Max steps is 327 (for x0 + 2*n).
        // Grid is 131x131. Start is at center.
        // Range is approx -330 to +460.
        // A 1000x1000 grid is sufficient and fits in cache.

        let offset = 500;
        let dim = 1000;
        let mut visited = vec![i32::MAX; usize::try_from(dim * dim).unwrap()];
        let mut queue = std::collections::VecDeque::new();

        // Start position
        let start_idx = (self.start.x + offset) + (self.start.y + offset) * dim;
        let start_idx_u = usize::try_from(start_idx).unwrap();
        visited[start_idx_u] = 0;
        queue.push_back((self.start.x, self.start.y, 0));

        let mut count = 0;

        // Parity we are looking for (same parity as n)
        let target_parity = n % 2;

        while let Some((x, y, dist)) = queue.pop_front() {
            if dist > n {
                continue;
            }

            // If this cell has the correct parity and is within range, count it
            if dist % 2 == target_parity {
                count += 1;
            }

            if dist == n {
                continue;
            }

            for (dx, dy) in DIRS {
                let nx = x + dx;
                let ny = y + dy;

                // Map to dense array index
                let idx = (nx + offset) + (ny + offset) * dim;
                let idx_u = usize::try_from(idx).unwrap();

                if visited[idx_u] == i32::MAX && self.is_garden(nx, ny) {
                    visited[idx_u] = dist + 1;
                    queue.push_back((nx, ny, dist + 1));
                }
            }
        }

        count
    }

    fn big_count(&self, n: i32) -> u64 {
        // the step count curve is parabolic
        let t = n / self.n;
        let x0 = n % self.n;

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
