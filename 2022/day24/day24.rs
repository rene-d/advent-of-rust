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
                        _ => panic!("bad input {x},{y} : {c}"),
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
        let trip2 = self.solve(self.x_exit, self.y_max, self.x_entry, 0, trip1);

        // then go to the exit
        self.solve(self.x_entry, 0, self.x_exit, self.y_max, trip2)
    }

    fn solve(&self, start_x: i32, start_y: i32, end_x: i32, end_y: i32, start_time: i32) -> i32 {
        let mut q = VecDeque::new();
        let mut seen = HashSet::new();

        #[cfg(debug_assertions)]
        println!(
            "\ntime: {start_time} - entry: {start_x},{start_y} - exit: {end_x},{end_y}"
        );

        q.push_back((start_x, start_y, start_time));

        #[cfg(debug_assertions)]
        self.show(start_x, start_y, start_time);

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
                    self.show(end_x, end_y, next_time);

                    #[cfg(debug_assertions)]
                    println!("found: {next_time}");

                    return next_time;
                }

                if !(next_x == start_x && next_y == start_y) {
                    // do not cross the boundaries (0 and max are the walls)
                    if next_x <= 0 || next_y <= 0 || next_x >= self.x_max || next_y >= self.y_max {
                        continue;
                    }

                    // test if elf will be blocked by blizzard
                    // nota: start position cannot be affected
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
                        continue;
                    }
                }

                let key = (next_x, next_y, next_time);
                if !seen.contains(&key) {
                    seen.insert(key);
                    q.push_back(key);
                }
            }
        }

        #[cfg(debug_assertions)]
        println!("no solution found");

        0
    }

    #[cfg(debug_assertions)]
    fn show(&self, elf_x: i32, elf_y: i32, time: i32) {
        println!("\ntime {time}");
        print!("{}", self.grid_str(elf_x, elf_y, time));
    }

    #[cfg(any(test, debug_assertions))]
    fn grid_str(&self, elf_x: i32, elf_y: i32, time: i32) -> String {
        let mut grid = String::new();

        for y in 0..=self.y_max {
            let mut line = String::new();
            for x in 0..=self.x_max {
                line += if x == elf_x && y == elf_y {
                    //"\x1B[1mE\x1B[0m"
                    "E"
                } else if x == 0 || x == self.x_max {
                    "#"
                } else if y == 0 {
                    if x == self.x_entry {
                        "."
                    } else {
                        "#"
                    }
                } else if y == self.y_max {
                    if x == self.x_exit {
                        "."
                    } else {
                        "#"
                    }
                } else {
                    let mut c = ".";
                    let mut b = 0usize;

                    if self.blizzards[RIGHT]
                        .contains(&((x - 1 - time).rem_euclid(self.x_max - 1) + 1, y))
                    {
                        c = ">";
                        b += 1;
                    }
                    if self.blizzards[LEFT]
                        .contains(&((x - 1 + time).rem_euclid(self.x_max - 1) + 1, y))
                    {
                        c = "<";
                        b += 1;
                    }
                    if self.blizzards[DOWN]
                        .contains(&(x, (y - 1 - time).rem_euclid(self.y_max - 1) + 1))
                    {
                        c = "v";
                        b += 1;
                    }
                    if self.blizzards[UP]
                        .contains(&(x, (y - 1 + time).rem_euclid(self.y_max - 1) + 1))
                    {
                        c = "^";
                        b += 1;
                    }

                    if b > 1 {
                        &"01234"[b..=b]
                    } else {
                        c
                    }
                };
            }

            grid += &line;
            grid += "\n";
        }

        grid
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

#[cfg(test)]
mod tests {
    use crate::Puzzle;

    #[test]
    fn test_part1() {
        let mut puzzle = Puzzle::new();
        puzzle.configure("test.txt");
        assert_eq!(puzzle.part1(), 18);
    }

    #[test]
    fn test_part2() {
        let mut puzzle = Puzzle::new();
        puzzle.configure("test.txt");
        assert_eq!(puzzle.part2(), 54);
    }

    #[test]
    fn test_part2_details() {
        let mut puzzle = Puzzle::new();
        puzzle.configure("test.txt");
        assert_eq!(puzzle.solve(1, 0, 6, 5, 0), 18); // this is part 1 actually
        assert_eq!(puzzle.solve(6, 5, 1, 0, 18), 18 + 23);
        assert_eq!(puzzle.solve(1, 0, 6, 5, 18 + 23), 54);
    }

    #[test]
    fn test_show_demo() {
        let mut puzzle = Puzzle::new();

        puzzle.configure("demo.txt");

        assert_eq!(
            puzzle.grid_str(-1, -1, 0),
            "\
#.#####
#.....#
#>....#
#.....#
#...v.#
#.....#
#####.#
"
        );

        assert_eq!(
            puzzle.grid_str(-1, -1, 1),
            "\
#.#####
#.....#
#.>...#
#.....#
#.....#
#...v.#
#####.#
"
        );

        assert_eq!(
            puzzle.grid_str(-1, -1, 2),
            "\
#.#####
#...v.#
#..>..#
#.....#
#.....#
#.....#
#####.#
"
        );

        assert_eq!(
            puzzle.grid_str(-1, -1, 3),
            "\
#.#####
#.....#
#...2.#
#.....#
#.....#
#.....#
#####.#
"
        );

        assert_eq!(
            puzzle.grid_str(-1, -1, 4),
            "\
#.#####
#.....#
#....>#
#...v.#
#.....#
#.....#
#####.#
"
        );

        assert_eq!(
            puzzle.grid_str(-1, -1, 5),
            "\
#.#####
#.....#
#>....#
#.....#
#...v.#
#.....#
#####.#
"
        );
    }

    #[test]
    fn test_show_test() {
        let mut puzzle = Puzzle::new();

        puzzle.configure("test.txt");

        assert_eq!(
            puzzle.grid_str(3, 1, 10),
            "\
#.######
#.2E.>2#
#<2v2^.#
#<>.>2.#
#..<>..#
######.#
"
        );

        assert_eq!(
            puzzle.grid_str(6, 5, 18),
            "\
#.######
#>2.<.<#
#.2v^2<#
#>..>2>#
#<....>#
######E#
"
        );
    }
}
