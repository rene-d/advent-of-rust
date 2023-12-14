//! [Day 14: Parabolic Reflector Dish](https://adventofcode.com/2023/day/14)

use std::collections::HashMap;

use clap::Parser;

#[derive(Parser)]
struct Args {
    /// Puzzle input
    #[arg(default_value = "input.txt")]
    path: String,
}

/// Parabolic Reflector Dish.
struct Dish {
    grid: Vec<Vec<char>>,
    sx: usize,
    sy: usize,
}

impl Dish {
    fn new(data: &str) -> Self {
        let mut dish = Self {
            grid: vec![],
            sx: 0,
            sy: 0,
        };

        for line in data.lines() {
            let mut row = vec![];

            for c in line.chars() {
                row.push(c);
            }
            dish.grid.push(row);
        }

        dish.sx = dish.grid[0].len();
        dish.sy = dish.grid.len();

        dish
    }

    /// Tilt the dish northwards.
    fn north(&mut self) {
        for x in 0..self.sx {
            for y in 0..self.sy {
                if self.grid[y][x] == '.' {
                    for y2 in y..self.sy {
                        match self.grid[y2][x] {
                            'O' => {
                                self.grid[y][x] = 'O';
                                self.grid[y2][x] = '.';
                                break;
                            }
                            '#' => break,
                            _ => (),
                        }
                    }
                }
            }
        }
    }

    /// Tilt the dish southwards.
    fn south(&mut self) {
        for x in 0..self.sx {
            for y in (0..self.sy).rev() {
                if self.grid[y][x] == '.' {
                    for y2 in (0..y).rev() {
                        match self.grid[y2][x] {
                            'O' => {
                                self.grid[y][x] = 'O';
                                self.grid[y2][x] = '.';
                                break;
                            }
                            '#' => break,
                            _ => (),
                        }
                    }
                }
            }
        }
    }

    /// Tilt the dish westwards.
    fn west(&mut self) {
        for y in 0..self.sy {
            for x in 0..(self.sx) {
                if self.grid[y][x] == '.' {
                    for x2 in (x)..self.sx {
                        match self.grid[y][x2] {
                            'O' => {
                                self.grid[y][x] = 'O';
                                self.grid[y][x2] = '.';
                                break;
                            }
                            '#' => break,
                            _ => (),
                        }
                    }
                }
            }
        }
    }

    /// Tilt the dish eastwards.
    fn east(&mut self) {
        for y in 0..self.sy {
            for x in (0..(self.sx)).rev() {
                if self.grid[y][x] == '.' {
                    for x2 in (0..x).rev() {
                        match self.grid[y][x2] {
                            'O' => {
                                self.grid[y][x] = 'O';
                                self.grid[y][x2] = '.';
                                break;
                            }
                            '#' => break,
                            _ => (),
                        }
                    }
                }
            }
        }
    }

    /// Compute the load.
    fn load(&self) -> usize {
        let mut result = 0;
        for y in (0..self.sy).rev() {
            let mut n = 0;
            for x in 0..self.sx {
                if self.grid[y][x] == 'O' {
                    n += 1;
                }
            }
            result += n * (self.sy - y);
        }
        result
    }

    /// Return a hashable value that represents the actual state of the dish.
    fn state(&self) -> Vec<Vec<char>> {
        self.grid.clone()
    }
}

struct Puzzle {
    data: String,
}

impl Puzzle {
    fn new() -> Puzzle {
        Puzzle {
            data: String::new(),
        }
    }

    /// Get the puzzle input.
    fn configure(&mut self, path: &str) {
        let data = std::fs::read_to_string(path).unwrap();

        self.data = data;
    }

    /// Solve part one.
    fn part1(&mut self) -> usize {
        let mut dish = Dish::new(&self.data);
        dish.north();
        dish.load()
    }

    /// Solve part two.
    fn part2(&mut self) -> usize {
        let mut dish = Dish::new(&self.data);

        let cycles = 1_000_000_000;
        let mut seen = HashMap::new();

        for mut i in 1..=cycles {
            dish.north();
            dish.west();
            dish.south();
            dish.east();

            let key = dish.state();

            if seen.contains_key(&key) {
                // same configuration detected: we have a cycle
                let cycle_length = i - seen.get(&key).unwrap();

                // skip as many cycles as possible
                i += ((cycles - i) / cycle_length) * cycle_length;

                // then continue to reach the wanted cycle number
                while i < cycles {
                    i += 1;
                    dish.north();
                    dish.west();
                    dish.south();
                    dish.east();
                }

                // we've done
                return dish.load();
            }

            seen.insert(key, i);
        }

        0
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
        assert_eq!(puzzle.part1(), 136);
    }

    #[test]
    fn test02() {
        let mut puzzle = Puzzle::new();
        puzzle.configure("test.txt");
        assert_eq!(puzzle.part2(), 64);
    }
}

fn main() {
    let args = Args::parse();
    let mut puzzle = Puzzle::new();
    puzzle.configure(args.path.as_str());
    println!("{}", puzzle.part1());
    println!("{}", puzzle.part2());
}
