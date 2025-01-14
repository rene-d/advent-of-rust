//! [Day 19: Tractor Beam](https://adventofcode.com/2019/day/19)

use intcode::{Computer, State};

const SQUARE: i64 = 100;
const SQUARE_USIZE: usize = 100;
const N: i64 = 50;

struct Scanner {
    xmax: i64,
    x1: i64,
    x2: i64,
    drone: Computer,
}

impl Scanner {
    fn new(drone: &Computer, xmax: i64) -> Self {
        Self {
            xmax,
            x1: 0,
            x2: -1,
            drone: drone.clone(),
        }
    }

    fn scan_cell(&mut self, x: i64, y: i64) -> i64 {
        self.drone.reset();
        self.drone.push(x);
        self.drone.push(y);

        if let State::Output(num) = self.drone.run() {
            num
        } else {
            panic!("failed to scan cell {x},{y}")
        }
    }

    fn scan_row(&mut self, y: i64) -> Option<i64> {
        let mut x1 = self.x1;

        while self.scan_cell(x1, y) == 0 && x1 < self.xmax {
            x1 += 1;
        }

        if x1 == self.xmax {
            return None;
        }

        self.x1 = x1;

        if x1 > self.x2 {
            self.x2 = x1;
        }

        while self.x2 < self.xmax && self.scan_cell(self.x2, y) == 1 {
            self.x2 += 1;
        }

        Some(self.x2 - self.x1)
    }
}

struct Puzzle {
    drone: Computer,
}

impl Puzzle {
    /// Initialize from the puzzle input.
    fn new(data: &str) -> Self {
        Self {
            drone: Computer::load(data),
        }
    }

    /// Solve part one.
    fn part1(&self) -> i64 {
        let mut scanner = Scanner::new(&self.drone, N);

        (0..50).filter_map(|y| scanner.scan_row(y)).sum()
    }

    /// Solve part two.
    fn part2(&self) -> i64 {
        let mut scanner = Scanner::new(&self.drone, 50);

        for y in 0..N {
            scanner.scan_row(y);
        }

        let mut grid = Vec::new();

        scanner.xmax = 10000;
        for y in N..10000 {
            scanner.scan_row(y);

            let row = (y, scanner.x1, scanner.x2);
            grid.push(row);

            if y < N + SQUARE {
                continue;
            }

            // let's consider the square OABC:
            // ................#################.......
            // .................########O--------A..... <= 100th row before
            // ..................#######|        |#....
            // ...................######|        |###..
            // ....................#####|        |#####
            // .....................####|        |#####
            // .....................####|        |#####
            // ......................###|        |#####
            // .......................##|        |#####
            // ........................#|        |#####
            // .........................C--------B##### <- current row
            // ..........................##############
            // - A should be at the very right of the 100th row before the current one
            // - B should be at the very left of the current row
            // - and, obviously, all points should be into the beam

            let upper_row = grid[grid.len() - SQUARE_USIZE];

            let y_b = row.0;
            // let y_c = row.0;
            let y_o = upper_row.0;
            // let y_a = upper_row.0;

            // the scan should not miss any row
            assert_eq!(y_b - y_o + 1, SQUARE);

            let x_a = upper_row.2;
            let x_c = x_a - SQUARE;
            let x_o = x_a - SQUARE;
            let x_b = x_a;

            if x_o < upper_row.1 || x_c < row.1 || x_b > row.2 {
                continue;
            }

            return x_o * 10000 + y_o;
        }

        0
    }
}

fn main() {
    let args = aoc::parse_args();
    let puzzle = Puzzle::new(&args.input);
    println!("{}", puzzle.part1());
    println!("{}", puzzle.part2());
}
