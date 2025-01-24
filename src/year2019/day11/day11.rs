//! [Day 11: Space Police](https://adventofcode.com/2019/day/11)

use aoc::{ocr, Coord, Grid};
use intcode::{Computer, State};
use rustc_hash::FxHashMap;

#[derive(PartialEq)]
enum Color {
    Black,
    White,
}

impl From<&Color> for i64 {
    fn from(value: &Color) -> Self {
        match value {
            Color::Black => 0,
            Color::White => 1,
        }
    }
}

struct Robot {
    brain: Computer,
    panel: FxHashMap<Coord, Color>,
}

impl Robot {
    const TURN_LEFT: i64 = 0;
    const TURN_RIGHT: i64 = 1;

    const MOVES: &[Coord] = &[Coord::UP, Coord::RIGHT, Coord::DOWN, Coord::LEFT];

    fn new(brain: &Computer) -> Self {
        Self {
            brain: brain.clone(),
            panel: FxHashMap::default(),
        }
    }

    fn paint(&mut self, initial_color: Color) {
        self.brain.reset();

        let mut pos = Coord::ZERO;
        let mut direction = 0;

        self.panel.insert(pos, initial_color);

        loop {
            // by default (coords are missing), panel is black
            let color = self.panel.get(&pos).unwrap_or(&Color::Black);

            // provide the current panel color to the computer
            self.brain.push(i64::from(color));

            // first output is the color to paint the panel
            let paint = match self.brain.run() {
                State::Output(value) => {
                    if value == 0 {
                        Color::Black
                    } else {
                        Color::White
                    }
                }
                State::Halted => break,
                State::Input => panic!(),
            };

            // second output is the direction the robot should turn
            let State::Output(turn) = self.brain.run() else {
                panic!()
            };

            // paint the panel
            self.panel.insert(pos, paint);

            // update the robot direction
            direction = match turn {
                Self::TURN_RIGHT => (direction + 1) % 4,
                Self::TURN_LEFT => (direction + 3) % 4,
                _ => panic!(),
            };

            // move the robot
            pos += Self::MOVES[direction];
        }
    }

    fn to_grid(&self) -> Grid<char> {
        let mut ll = Coord::new(i32::MAX, i32::MAX);
        let mut ur = Coord::new(i32::MIN, i32::MIN);
        for pos in self.panel.keys() {
            ll.x = ll.x.min(pos.x);
            ll.y = ll.y.min(pos.y);
            ur.x = ur.x.max(pos.x);
            ur.y = ur.y.max(pos.y);
        }

        let mut grid = Grid::<char>::with_size(ur.x - ll.x + 1, ur.y - ll.y + 1, '.', ' ');
        for (&pos, c) in &self.panel {
            if c == &Color::White {
                grid[pos + ll] = '#';
            }
        }

        grid
    }

    fn drawing(&self) -> String {
        ocr::scan_5x6(&self.to_grid().to_string())
    }
}

struct Puzzle {
    brain: Computer,
}

impl Puzzle {
    /// Initialize from the puzzle input.
    fn new(data: &str) -> Self {
        Self {
            brain: Computer::load(data),
        }
    }

    /// Solve part one.
    fn part1(&self) -> usize {
        let mut robot = Robot::new(&self.brain);
        robot.paint(Color::Black);
        robot.panel.len()
    }

    /// Solve part two.
    fn part2(&self) -> String {
        let mut robot = Robot::new(&self.brain);
        robot.paint(Color::White);
        robot.drawing()
    }
}

/// # Panics
#[must_use]
pub fn solve(data: &str) -> (usize, String) {
    let puzzle = Puzzle::new(data);
    (puzzle.part1(), puzzle.part2())
}

pub fn main() {
    let args = aoc::parse_args();
    args.run(solve);
}
