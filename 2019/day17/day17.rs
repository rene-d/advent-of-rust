//! [Day 17: Set and Forget](https://adventofcode.com/2019/day/17)

use aoc::{Coord, Grid};
use intcode::{Computer, State};

/// Little tribute to [Mars Pathfinder](https://en.wikipedia.org/wiki/Mars_Pathfinder)
/// and the [JPL](https://www.jpl.nasa.gov).
///
/// Dare Mighty Things 🚀
fn pathfinder(grid: &Grid<u8>) -> String {
    let (position, direction_symbol) = grid
        .iter()
        .find(|(_, c)| [b'^', b'>', b'v', b'<'].contains(c))
        .unwrap();

    let mut direction = [b'^', b'>', b'v', b'<']
        .iter()
        .position(|d| d == direction_symbol)
        .unwrap();

    // build the scaffolding
    let moves = &[Coord::UP, Coord::RIGHT, Coord::DOWN, Coord::LEFT];
    let mut path = String::new();
    let mut xy = position;
    loop {
        let mut dxy = Coord::ZERO;
        let mut d = 0;
        for tmp_d in [1, 3] {
            let tmp_dxy = moves[(direction + tmp_d) % 4];
            if grid[xy + tmp_dxy] == b'#' {
                dxy = tmp_dxy;
                d = tmp_d;
                break;
            }
        }
        if dxy == Coord::ZERO || d == 0 {
            break;
        }

        let mut length = 0;
        while grid[xy + dxy] == b'#' {
            length += 1;
            xy += dxy;
        }

        direction = (direction + d) % 4;

        let letter_d = match d {
            1 => 'R',
            3 => 'L',
            _ => unreachable!(),
        };
        path.push_str(&format!("{letter_d},{length};"));
    }

    path
}

struct PathCompressor<'a> {
    routines: Vec<u8>,
    functions: [&'a str; 3],
}

impl<'a> PathCompressor<'a> {
    fn new(path: &'a str) -> Self {
        let mut pc = Self {
            routines: Vec::new(),
            functions: [""; 3],
        };
        pc.compress(path);
        pc
    }

    fn compress(&mut self, path: &'a str) -> bool {
        if path.is_empty() {
            return false;
        }

        if self.routines.len() > 10 {
            // 10 function calls + comma at most
            return true;
        }

        for (i, name) in [b'A', b'B', b'C'].iter().enumerate() {
            self.routines.push(*name);

            let needle = self.functions[i];

            if needle.is_empty() {
                for (p, _) in path.match_indices(';') {
                    let needle = &path[..=p];
                    let remaining = &path[(p + 1)..];

                    if needle.len() > 21 || needle.is_empty() {
                        break;
                    }

                    self.functions[i] = needle;
                    if !self.compress(remaining) {
                        return false;
                    }
                    self.functions[i] = "";
                }
            } else if let Some(zz) = path.strip_prefix(needle) {
                if !self.compress(zz) {
                    return false;
                }
            }

            self.routines.pop();
        }

        true
    }
}

struct Puzzle {
    aft: Computer,
    grid: Grid<u8>,
}

impl Puzzle {
    /// Initialize from the puzzle input.
    fn new(data: &str) -> Self {
        let mut aft = Computer::load(data);

        let mut scaffold = Vec::new();

        while let State::Output(num) = aft.run() {
            scaffold.push(u8::try_from(num).unwrap());
        }
        aft.reset();

        let output: String = scaffold
            .iter()
            .filter_map(|c| char::from_u32(u32::from(*c)))
            .collect();
        let mut grid = Grid::<u8>::parse(&output);
        grid.set_exterior(0); // '#' by default...

        Self { aft, grid }
    }

    /// Solve part one.
    fn part1(&self) -> i32 {
        let mut grid = self.grid.clone();
        let mut result = 0;

        for y in 1..grid.height() - 1 {
            for x in 1..grid.width() - 1 {
                if grid[(x, y)] == b'#'
                    && grid[(x + 1, y)] == b'#'
                    && grid[(x - 1, y)] == b'#'
                    && grid[(x, y + 1)] == b'#'
                    && grid[(x, y - 1)] == b'#'
                {
                    result += x * y;
                    grid[(x, y)] = b'O';
                }
            }
        }

        result
    }

    /// Solve part two.
    fn part2(&self) -> i64 {
        let mars = pathfinder(&self.grid);

        let pc = PathCompressor::new(&mars);

        let mut aft = self.aft.clone();

        aft.reset();
        aft.poke(0, 2); // wake up the robot

        // program the robot

        // "Main:" prompt (sequence of functions)
        for c in aoc::util::join_with_final(&pc.routines, b',', b'\n') {
            aft.push_byte(c);
        }

        // "Function x:" prompts (x3)
        for function in &pc.functions {
            for c in function.trim_end_matches(';').bytes() {
                aft.push_byte(if c == b';' { b',' } else { c });
            }
            aft.push_byte(b'\n');
        }

        // answer to "Continuous video feed?"
        aft.push_byte(b'n');
        aft.push_byte(b'\n');

        let mut robot_report = 0;
        loop {
            match aft.run() {
                State::Output(num) => {
                    // print!("{}", (num as u8) as char);
                    robot_report = num;
                }
                State::Input => panic!("missing input ?!"),
                State::Halted => break,
            };
        }

        robot_report
    }
}

fn main() {
    let args = aoc::parse_args();
    let puzzle = Puzzle::new(&args.input);
    println!("{}", puzzle.part1());
    println!("{}", puzzle.part2());
}
