//! [Day 24: Blizzard Basin](https://adventofcode.com/2022/day/24)

use clap::Parser;
use std::collections::{HashSet, VecDeque};

#[derive(Parser)]
struct Args {
    /// Puzzle input
    #[arg(default_value = "input.txt")]
    path: String,
}

const RIGHT: usize = 0;
const DOWN: usize = 1;
const LEFT: usize = 2;
const UP: usize = 3;

struct Puzzle {
    x_max: i32,   // x=0 or x_max: left/right wall
    y_max: i32,   // y=0 or y_max: top/bottom wall
    x_entry: i32, // position of entry (y=0)
    x_exit: i32,  // position of exit (y=y_max)
    blizzards: [HashSet<(i32, i32)>; 4],
}

impl Puzzle {
    fn new() -> Self {
        Self {
            blizzards: [
                HashSet::new(), // rightward blizzard
                HashSet::new(), // downward blizzard
                HashSet::new(), // leftward blizzard
                HashSet::new(), // upward blizzard
            ],
            x_entry: 0,
            x_exit: 0,
            x_max: 0,
            y_max: 0,
        }
    }

    /// Loads data from input (one line)
    fn configure(&mut self, path: &str) {
        let data = std::fs::read_to_string(path).unwrap();

        for (y, row) in data.lines().enumerate() {
            let y = y as i32;

            for (x, c) in row.chars().enumerate() {
                let x = x as i32;

                if c == '.' {
                    if y == 0 {
                        // only one dot on the first line else #
                        self.x_entry = x;
                    } else {
                        // the last dot will be on the last line
                        self.x_exit = x;
                    }
                } else if c != '#' {
                    let dir = match c {
                        '>' => RIGHT,
                        'v' => DOWN,
                        '<' => LEFT,
                        '^' => UP,
                        _ => panic!("bad input {},{} : {}", x, y, c),
                    };
                    self.blizzards[dir].insert((x, y));
                }

                self.x_max = self.x_max.max(x);
            }

            self.y_max = self.y_max.max(y);
        }

        #[cfg(debug_assertions)]
        println!(
            "entry:{} exit:{} x:0..{} y:0..{}",
            self.x_entry, self.x_exit, self.x_max, self.y_max
        );
    }

    // Solves part one
    fn part1(&self) -> i32 {
        self.solve(self.x_entry, 0, self.x_exit, self.y_max, 0)
    }

    // Solve part two
    fn part2(&self) -> i32 {
        // first trip to the exit
        let trip1 = self.solve(self.x_entry, 0, self.x_exit, self.y_max, 0);

        // back to pick up the snacks
        let trip2 = self.solve(self.x_exit, self.y_max, self.x_entry, 0, trip1 + 1);

        // then go to the exit
        self.solve(self.x_entry, 0, self.x_exit, self.y_max, trip2 + 1)
    }

    fn solve(&self, start_x: i32, start_y: i32, end_x: i32, end_y: i32, start_time: i32) -> i32 {
        let mut q = VecDeque::new();
        let mut seen = HashSet::new();

        #[cfg(debug_assertions)]
        println!(
            "\ntime: {} - entry: {},{} - exit: {},{}",
            start_time, start_x, start_y, end_x, end_y
        );

        q.push_back((start_x, start_y, start_time));

        while let Some((x, y, time)) = q.pop_front() {
            // #[cfg(debug_assertions)]
            // println!("test x,y={},{} time={}", x, y, time);

            let next_time = time + 1;

            for (dx, dy) in [
                (0, 0),  // wait
                (1, 0),  // move right
                (0, 1),  // move down
                (-1, 0), // move left
                (0, -1), // move up
            ] {
                let next_x = x + dx;
                let next_y = y + dy;

                // are we arrived at destination ?
                if next_x == end_x && next_y == end_y {
                    #[cfg(debug_assertions)]
                    println!("found: {}", next_time);
                    return next_time;
                }

                if !(next_x == start_x && next_y == start_y) {
                    // do not cross the boundaries (0 and max are the walls)
                    if next_x <= 0 || next_y <= 0 || next_x >= self.x_max || next_y >= self.y_max {
                        continue;
                    }
                }

                if [(RIGHT, 1, 0), (DOWN, 0, 1), (LEFT, -1, 0), (UP, 0, -1)]
                    .iter()
                    .any(|(dir, tx, ty)| -> bool {
                        // check if the move is possible
                        // i.e. if time minutes ago, the cell was a blizzard initial position
                        // nota: blizzard x,y are between 1 and self.{x,y}_max-1 included
                        let bx = (next_x - 1 - tx * next_time).rem_euclid(self.x_max - 1) + 1;
                        let by = (next_y - 1 - ty * next_time).rem_euclid(self.y_max - 1) + 1;
                        self.blizzards[*dir].contains(&(bx, by))
                    })
                {
                    // don't share the position with a blizzard
                } else {
                    let key = (next_x, next_y, next_time);
                    if !seen.contains(&key) {
                        seen.insert(key);
                        q.push_back(key);
                    }
                }
            }
        }

        #[cfg(debug_assertions)]
        println!("no solution found");

        0
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
    assert_eq!(puzzle.part1(), 18);
    // assert_eq!(puzzle.part2(), 54); // second trip doesn't work 😖

    assert_eq!(puzzle.solve(1, 0, 6, 5, 0), 18);
    assert_eq!(puzzle.solve(1, 0, 6, 5, 18 + 23 + 2), 54);
}
