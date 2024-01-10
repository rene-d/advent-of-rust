//! [Day 15: Chiton](https://adventofcode.com/2021/day/15)

use std::collections::BinaryHeap;

struct Cost {
    cost: u32,
    x: usize,
    y: usize,
}

impl Ord for Cost {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Cost {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Cost {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}

impl Eq for Cost {}

fn min_cost(grid: &[Vec<u32>]) -> u32 {
    let n = grid.len();
    let mut d = vec![vec![0u32; n]; n];

    let mut heap = BinaryHeap::new();

    heap.push(Cost {
        cost: 0,
        x: 0,
        y: 0,
    });

    while !heap.is_empty() {
        let cur = heap.pop().unwrap();

        let new_cost = cur.cost + grid[cur.y][cur.x];

        if new_cost >= d[cur.y][cur.x] && d[cur.y][cur.x] != 0 {
            continue;
        }

        d[cur.y][cur.x] = new_cost;

        if cur.x == n - 1 && cur.y == n - 1 {
            break;
        }

        if cur.x + 1 < n {
            heap.push(Cost {
                cost: new_cost,
                x: cur.x + 1,
                y: cur.y,
            });
        }
        if cur.y + 1 < n {
            heap.push(Cost {
                cost: new_cost,
                x: cur.x,
                y: cur.y + 1,
            });
        }
        if cur.x > 0 {
            heap.push(Cost {
                cost: new_cost,
                x: cur.x - 1,
                y: cur.y,
            });
        }
        if cur.y > 0 {
            heap.push(Cost {
                cost: new_cost,
                x: cur.x,
                y: cur.y - 1,
            });
        }
    }

    d[n - 1][n - 1]
}

struct Puzzle {
    grid: Vec<Vec<u32>>,
}

impl Puzzle {
    fn new() -> Puzzle {
        Puzzle { grid: vec![] }
    }

    /// Get the puzzle input.
    fn configure(&mut self, path: &str) {
        let data = std::fs::read_to_string(path).unwrap();

        // read the grid
        for line in data.lines() {
            let row: Vec<_> = line.chars().filter_map(|c| c.to_digit(10)).collect();
            self.grid.push(row);
        }
    }

    /// Solve part one.
    fn part1(&self) -> u32 {
        min_cost(&self.grid) - self.grid[0][0]
    }

    /// Solve part two.
    fn part2(&self) -> u32 {
        let n = self.grid.len();

        // build the five times larger grid
        let mut grid5 = vec![vec![0u32; 5 * n]; 5 * n];
        for y in 0..n {
            for x in 0..n {
                let v = self.grid[y][x];

                for yy in 0..5 {
                    for xx in 0..5 {
                        grid5[y + n * yy][x + n * xx] =
                            (v - 1 + u32::try_from(xx + yy).unwrap()) % 9 + 1;
                    }
                }
            }
        }

        min_cost(&grid5) - self.grid[0][0]
    }
}

fn main() {
    let args = aoc::parse_args();
    let mut puzzle = Puzzle::new();
    puzzle.configure(args.path.as_str());
    println!("{}", puzzle.part1());
    println!("{}", puzzle.part2());
}

/// Test from puzzle input
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test01() {
        let mut puzzle = Puzzle::new();
        puzzle.configure("sample_1.txt");
        assert_eq!(puzzle.part1(), 40);
    }

    #[test]
    fn test02() {
        let mut puzzle = Puzzle::new();
        puzzle.configure("sample_1.txt");
        assert_eq!(puzzle.part2(), 315);
    }
}
