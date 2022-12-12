//! [Day 12: Hill Climbing Algorithm](https://adventofcode.com/2022/day/12)

use clap::Parser;
use std::collections::{HashSet, VecDeque};

#[derive(Parser)]
struct Args {
    /// Puzzle input
    #[arg(default_value = "input.txt")]
    path: String,
}

#[derive(Debug)]
struct Node {
    pos: (usize, usize),
    steps: usize,
}

struct Puzzle {
    grid: Vec<Vec<u8>>,
    nx: usize,
    ny: usize,
    start: (usize, usize),
    end: (usize, usize),
}

impl Puzzle {
    fn new() -> Self {
        Self {
            grid: vec![],
            start: (0, 0),
            end: (0, 0),
            nx: 0,
            ny: 0,
        }
    }

    /// Loads data from input (one line)
    fn configure(&mut self, path: &str) {
        let data = std::fs::read_to_string(path).unwrap();
        let lines = data.split('\n').collect::<Vec<_>>();

        self.nx = lines.first().unwrap().len();
        let mut y = 0;
        for line in lines {
            if line.is_empty() {
                continue;
            }
            let mut row = vec![];
            row.resize(self.nx, 0);
            for (x, pos) in line.chars().enumerate() {
                if pos == 'S' {
                    self.start = (x, y);
                }
                row[x] = match pos {
                    'S' => {
                        self.start = (x, y);
                        0
                    }
                    'E' => {
                        self.end = (x, y);
                        26
                    }
                    _ => (pos as u8) - 96,
                };
            }
            self.grid.push(row);
            y += 1;
        }
        self.ny = self.grid.len();
    }

    fn bfs(&self, part: u8) -> usize {
        let mut q: VecDeque<Node> = VecDeque::new();
        let mut seen: HashSet<(usize, usize)> = HashSet::new();

        for y in 0..self.ny {
            for x in 0..self.nx {
                if self.grid[y][x] == part {
                    let n = Node {
                        pos: (x, y),
                        steps: 0,
                    };

                    q.push_back(n);
                }
            }
        }

        while !q.is_empty() {
            let n = q.pop_front().unwrap();

            if n.pos == self.end {
                return n.steps;
            }

            if seen.contains(&n.pos) {
                continue;
            }
            seen.insert(n.pos);

            for m in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                let pos2 = (n.pos.0 as isize + m.0, n.pos.1 as isize + m.1);

                if 0 <= pos2.0
                    && pos2.0 < self.nx as isize
                    && 0 <= pos2.1
                    && pos2.1 < self.ny as isize
                {
                    let pos2 = (pos2.0 as usize, pos2.1 as usize);
                    if self.grid[pos2.1][pos2.0] <= 1 + self.grid[n.pos.1][n.pos.0] {
                        q.push_back(Node {
                            pos: pos2,
                            steps: n.steps + 1,
                        });
                    }
                }
            }
        }

        0
    }

    // Solves part one
    fn part1(&self) -> usize {
        self.bfs(0)
    }

    // Solve part two
    fn part2(&self) -> usize {
        self.bfs(1)
    }
}

/// main function
fn main() {
    let args = Args::parse();
    let mut puzzle = Puzzle::new();
    puzzle.configure(&args.path);
    println!("{}", puzzle.part1());
    println!("{}", puzzle.part2());
}

#[test]
fn test01() {
    let mut puzzle = Puzzle::new();
    puzzle.configure("test.txt");
    assert_eq!(puzzle.part1(), 31);
    assert_eq!(puzzle.part2(), 29);
}
