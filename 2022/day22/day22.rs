//! [Day 22: Monkey Map](https://adventofcode.com/2022/day/22)

use regex::Regex;

const RIGHT: u8 = 0;
const DOWN: u8 = 1;
const LEFT: u8 = 2;
const UP: u8 = 3;

struct Puzzle {
    grid: Vec<String>,
    path: Vec<(u32, char)>,
}

impl Puzzle {
    const fn new() -> Self {
        Self {
            grid: vec![],
            path: vec![],
        }
    }

    /// Loads data from input (one line)
    fn configure(&mut self, data: &str) {
        let (grid, path) = data.split_once("\n\n").unwrap();

        for line in grid.lines() {
            self.grid.push(line.to_string());
        }

        let re = Regex::new(r"(\d+)([RDLU])?").unwrap();

        for m in re.captures_iter(path) {
            let n = m[1].parse::<u32>().unwrap();

            if let Some(d) = m.get(2) {
                let d = d.as_str().chars().next().unwrap();
                self.path.push((n, d));
            } else {
                self.path.push((n, ' '));
            }
        }
    }

    fn start(&self, y: usize) -> usize {
        let mut x = 0;
        for c in self.grid[y].chars() {
            if c != ' ' {
                break;
            }
            x += 1;
        }
        x
    }

    fn step(&self, orig_x: &mut usize, orig_y: &mut usize, d: &mut u8) {
        let mut x = *orig_x;
        let mut y = *orig_y;

        match *d {
            RIGHT => {
                x += 1;
                if self.out(x, y) {
                    x = self.start(y);
                }
            }
            DOWN => {
                y += 1;
                if self.out(x, y) {
                    y = 0;
                    while self.out(x, y) {
                        y += 1;
                    }
                }
            }
            LEFT => {
                let width = self.grid[y].len();
                x = (x + width - 1) % width;
                if self.out(x, y) {
                    x = width - 1;
                }
            }
            UP => {
                let height = self.grid.len();
                y = (y + height - 1) % height;
                while self.out(x, y) {
                    y = (y + height - 1) % height;
                }
            }
            _ => panic!(""),
        };

        if self.grid[y].chars().nth(x).unwrap() == '.' {
            *orig_x = x;
            *orig_y = y;
        }
    }

    fn step_cube(&self, orig_x: &mut usize, orig_y: &mut usize, orig_d: &mut u8) {
        let mut x = *orig_x;
        let mut y = *orig_y;
        let mut d = *orig_d;

        // the puzzle input layout (and NOT the demo) is :
        //
        //                        1    1
        //               4 5    9 0    4
        //          0    9 0    9 0    9
        //          .         f      g
        //          .     +------+------+
        //   0      .     |      |      |
        //          .    D|      |      |A
        //  49      .     |      |      |
        //          .     +------+------+
        //  50      .     |      |   b
        //          .    E|      |B
        //  99      .  e  |      |
        //         +------+------+
        // 100     |      |      |
        //        d|      |      |a
        // 149     |      |      |
        //         +------+------+
        // 150     |      |   c
        //        F|      |C
        // 199     |      |
        //         +------+
        //             G
        //
        // when the map is folded into a cube, edge A goes on edge a, etc.
        // coordinates of faces are (0..49 + 50*M, 0..49 + 50*N)

        match d {
            RIGHT => {
                x += 1;
                if self.out(x, y) {
                    if y < 50 {
                        // edge A to edge a
                        x = 99;
                        y = 149 - y;
                        d = LEFT;
                    } else if y < 100 {
                        // edge B to edge b
                        x = 100 + (y - 50);
                        y = 49;
                        d = UP;
                    } else if y < 150 {
                        // edge a to edge A
                        x = 149;
                        y = 149 - y;
                        d = LEFT;
                    } else if y < 200 {
                        // edge C to edge c
                        x = 50 + (y - 150);
                        y = 149;
                        d = UP;
                    }
                }
            }
            DOWN => {
                y += 1;
                if self.out(x, y) {
                    if x < 50 {
                        // edge G to edge g
                        x += 100;
                        y = 0;
                        d = DOWN;
                    } else if x < 100 {
                        // edge c to edge C
                        y = 150 + (x - 50);
                        x = 49;
                        d = LEFT;
                    } else if x < 150 {
                        // edge b to edge B
                        y = 50 + (x - 100);
                        x = 99;
                        d = LEFT;
                    }
                }
            }
            LEFT => {
                let width = self.grid[y].len();
                let x0 = x;
                x = (x + width - 1) % width;
                if x0 == 0 || self.out(x, y) {
                    if y < 50 {
                        // edge D to edge d
                        x = 0;
                        y = 149 - y;
                        d = RIGHT;
                    } else if y < 100 {
                        // edge E to edge e
                        x = y - 50;
                        y = 100;
                        d = DOWN;
                    } else if y < 150 {
                        // edge d to edge D
                        x = 50;
                        y = 149 - y;
                        d = RIGHT;
                    } else if y < 200 {
                        // edge F to edge f
                        x = 50 + (y - 150);
                        y = 0;
                        d = DOWN;
                    }
                }
            }
            UP => {
                let height = self.grid.len();
                let y0 = y;
                y = (y + height - 1) % height;
                if y0 == 0 || self.out(x, y) {
                    if x < 50 {
                        // edge e to edge E
                        y = 50 + x;
                        x = 50;
                        d = RIGHT;
                    } else if x < 100 {
                        // edge f to edge F
                        y = 150 + (x - 50);
                        x = 0;
                        d = RIGHT;
                    } else if x < 150 {
                        // edge g to edge G
                        x -= 100;
                        y = 199;
                        d = UP;
                    }
                }
            }
            _ => panic!(""),
        }

        if self.grid[y].chars().nth(x).unwrap() == '.' {
            *orig_x = x;
            *orig_y = y;
            *orig_d = d;
        }
    }

    fn out(&self, x: usize, y: usize) -> bool {
        if y >= self.grid.len() {
            return true;
        }
        if x >= self.grid[y].len() {
            return true;
        }
        self.grid[y].chars().nth(x).unwrap() == ' '
    }

    fn walk(&self, advance: fn(&Self, &mut usize, &mut usize, &mut u8)) -> usize {
        let mut x = self.start(0);
        let mut y = 0;
        let mut d = 0;

        for (n, t) in &self.path {
            for _ in 0..*n {
                advance(self, &mut x, &mut y, &mut d);
            }
            if *t == 'L' {
                d = (d + 3) % 4;
            } else if *t == 'R' {
                d = (d + 1) % 4;
            }
        }
        1000 * (y + 1) + 4 * (x + 1) + (d as usize)
    }

    // Solves part one
    fn part1(&self) -> usize {
        self.walk(Self::step)
    }

    // Solve part two
    fn part2(&self) -> usize {
        // NOT suitable for demo map 🙁
        assert_eq!(self.grid[0].len() / 3, 50);
        assert_eq!(self.grid.len() / 4, 50);

        self.walk(Self::step_cube)
    }
}

/// main function
fn main() {
    let args = aoc::parse_args();
    let mut puzzle = Puzzle::new();
    puzzle.configure(&args.input);
    println!("{}", puzzle.part1());
    println!("{}", puzzle.part2());
}

#[test]
fn test01() {
    let mut puzzle = Puzzle::new();
    puzzle.configure(&aoc::load_input_data("test.txt"));
    assert_eq!(puzzle.part1(), 6032);
    // assert_eq!(puzzle.part2(), 0);
}
