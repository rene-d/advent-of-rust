//! [Day 12: Hill Climbing Algorithm](https://adventofcode.com/2022/day/12)

use std::collections::{HashSet, VecDeque};

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
    const fn new() -> Self {
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
            let mut row = vec![0; self.nx];
            for (x, pos) in line.chars().enumerate() {
                if pos == 'S' {
                    self.start = (x, y);
                }
                row[x] = match pos {
                    'S' => {
                        // start position: height 0
                        self.start = (x, y);
                        1
                    }
                    'E' => {
                        // end position: height 26 (like 'z')
                        self.end = (x, y);
                        26
                    }
                    _ => (pos as u8) - 96, // other locations: height 1 to 26
                };
            }
            self.grid.push(row);
            y += 1;
        }
        self.ny = self.grid.len();
    }

    /// Breadth-first search
    fn bfs(&self, part: u8) -> usize {
        let mut q: VecDeque<Node> = VecDeque::new();
        let mut seen: HashSet<(usize, usize)> = HashSet::new();

        // set up the start positions:
        // 'S' if part 1
        // 'S' or any 'a' if part 2 (height 1)
        if part == 1 {
            q.push_back(Node {
                pos: self.start,
                steps: 0,
            });
        } else {
            for y in 0..self.ny {
                for x in 0..self.nx {
                    if self.grid[y][x] == 1 {
                        let n = Node {
                            pos: (x, y),
                            steps: 0,
                        };
                        q.push_back(n);
                    }
                }
            }
        }

        while !q.is_empty() {
            let n = q.pop_front().unwrap();
            let pos = n.pos;

            if pos == self.end {
                return n.steps;
            }

            if seen.contains(&pos) {
                continue;
            }
            seen.insert(pos);

            // Nota: need to figure out the best way to write this in Rust without ugly casts
            for step in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                let pos2 = (pos.0 as isize + step.0, pos.1 as isize + step.1);

                if 0 <= pos2.0
                    && pos2.0 < self.nx as isize
                    && 0 <= pos2.1
                    && pos2.1 < self.ny as isize
                {
                    let new_pos = (pos2.0 as usize, pos2.1 as usize);
                    if self.grid[new_pos.1][new_pos.0] <= 1 + self.grid[pos.1][pos.0] {
                        q.push_back(Node {
                            pos: new_pos,
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
        self.bfs(1)
    }

    // Solve part two
    fn part2(&self) -> usize {
        self.bfs(2)
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
    assert_eq!(puzzle.part1(), 31);
    assert_eq!(puzzle.part2(), 29);
}
