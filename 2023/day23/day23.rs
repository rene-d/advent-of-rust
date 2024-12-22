//! [Day 23: A Long Walk](https://adventofcode.com/2023/day/23)

use std::collections::{HashSet, VecDeque};
use std::time::{Duration, Instant};

const N: usize = 141;

fn walk(g: &mut [u8; N * N], max_cost: &mut u32, cost: u32, x: usize, y: usize) {
    if g[N * y + x] != 0 {
        return;
    }

    // through the recursion [storm],
    if x == 1 && y == 0 {
        // we reach the shore
        if cost > *max_cost {
            // you give it all but I want more
            *max_cost = cost;
        }
    } else {
        g[N * y + x] = 1;

        if x > 0 && g[N * y + (x - 1)] == 0 {
            walk(g, max_cost, cost + 1, x - 1, y);
        }
        if x < N - 1 && g[N * y + (x + 1)] == 0 {
            walk(g, max_cost, cost + 1, x + 1, y);
        }
        if y > 0 && g[N * (y - 1) + x] == 0 {
            walk(g, max_cost, cost + 1, x, y - 1);
        }
        if y < N - 1 && g[N * (y + 1) + x] == 0 {
            walk(g, max_cost, cost + 1, x, y + 1);
        }

        g[N * y + x] = 0;
    }
}

fn naive_part2(input: &str) -> u32 {
    let mut grid = [1u8; N * N];
    let mut target = 0;

    for (y, row) in input.lines().enumerate() {
        for (x, c) in row.chars().enumerate() {
            grid[N * y + x] = u8::from(c == '#');
        }
        target = y + 1;
    }

    let mut max_cost = 0;

    // Nota: start the ride at the arrival location:
    // the finish test is therefore the same for all grid sizes
    walk(&mut grid, &mut max_cost, 0, target - 2, target - 1);
    max_cost
}

struct Puzzle {
    data: String,
    grid: Vec<Vec<char>>,
}

impl Puzzle {
    const fn new() -> Self {
        Self {
            data: String::new(),
            grid: vec![],
        }
    }

    /// Get the puzzle input.
    fn configure(&mut self, path: &str) {
        let data = std::fs::read_to_string(path).unwrap();

        for line in data.lines() {
            self.grid.push(line.chars().collect());
        }

        assert!(self.grid.iter().all(|row| row.len() == self.grid.len()));

        self.data = data;
    }

    /// Solve part one.
    fn part1(&self) -> u32 {
        // size of the square grid
        let n = self.grid.len();

        // start posiiton
        let sx = 1;
        let sy = 0;

        // target position
        let tx = n - 2;
        let ty = n - 1;

        // min steps hike
        let mut m = 0;

        let mut q = VecDeque::new();
        let mut seen = HashSet::new();

        q.push_back((0, sx, sy, 0, 0));
        while let Some((c, x, y, px, py)) = q.pop_front() {
            if seen.contains(&(c, x, y)) {
                continue;
            }
            seen.insert((c, x, y));

            if x == tx && y == ty {
                m = m.max(c);
            }

            // if x+1 is inside the grid,
            // and the move is authorized (path or slop),
            // and we're not going backwards,
            // then queue the move
            if x < n - 1 && px != x + 1 && ".>".contains(self.grid[y][x + 1]) {
                q.push_back((c + 1, x + 1, y, x, y));
            }

            if x > 0 && px != x - 1 && ".<".contains(self.grid[y][x - 1]) {
                q.push_back((c + 1, x - 1, y, x, y));
            }

            if y < n - 1 && py != y + 1 && ".v".contains(self.grid[y + 1][x]) {
                q.push_back((c + 1, x, y + 1, x, y));
            }

            if y > 0 && py != y - 1 && ".^".contains(self.grid[y - 1][x]) {
                q.push_back((c + 1, x, y - 1, x, y));
            }
        }

        m
    }

    /// Solve part two.
    #[allow(clippy::unused_self)]
    fn part2(&self) -> u32 {
        // for the moment, I use this naive search, which runs in a reasonable amount of time,
        // much much less than the time needed to write a longest path algorithm (about 90s).
        naive_part2(&self.data)
    }
}

fn main() {
    let args = aoc::parse_args();
    let mut puzzle = Puzzle::new();
    puzzle.configure(args.path.as_str());
    if args.verbose {
        let start = Instant::now();

        println!("{}", puzzle.part2());
        let duration: Duration = start.elapsed();

        eprintln!("Time elapsed: {duration:?}");
    } else {
        println!("{}", puzzle.part1());
        println!("{}", puzzle.part2());
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
        assert_eq!(puzzle.part1(), 94);
    }

    #[test]
    fn test02() {
        let mut puzzle = Puzzle::new();
        puzzle.configure("test.txt");
        assert_eq!(puzzle.part2(), 154);
    }
}
