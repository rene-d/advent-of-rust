//! [Day 14: Parabolic Reflector Dish](https://adventofcode.com/2023/day/14)

use std::collections::{HashMap, HashSet};

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

    fn get(&self, x: usize, y: usize) -> char {
        if x < self.sx && y < self.sy {
            *self.grid[y].get(x).unwrap()
        } else {
            '@'
        }
    }

    /// Tilt the dish northwards.
    fn north(&mut self) {
        for x in 0..self.sx {
            for y in 0..self.sy {
                if self.grid[y][x] == '.' {
                    for y2 in y..self.sy {
                        match self.get(x, y2) {
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
                        match self.get(x, y2) {
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
                        match self.get(x2, y) {
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
                        match self.get(x2, y) {
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

impl std::fmt::Display for Dish {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for row in &self.grid {
            for c in row {
                match c {
                    'O' => f.write_str("\x1b[95mo\x1b[0m")?,
                    _ => write!(f, "{c}")?,
                }
            }
            writeln!(f)?;
        }
        Ok(())
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
    fn part1(&self) -> usize {
        let mut dish = Dish::new(&self.data);
        dish.north();
        dish.load()
    }

    /// Solve part two.
    fn part2(&self) -> usize {
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

                eprintln!("{dish}");

                // we've done
                return dish.load();
            }

            seen.insert(key, i);
        }

        0
    }

    /// Displays an ASCII animation of the platform's tilt.
    /// Rather useless.
    fn anim(&self) {
        let mut dish = Dish::new(&self.data);
        let mut seen = HashSet::new();

        let tempo = std::time::Duration::from_millis(100);

        let show = |dish: &Dish| {
            println!("\x1b[H\x1b[2J{dish}");
            std::thread::sleep(tempo);
        };

        loop {
            dish.north();
            show(&dish);

            dish.west();
            show(&dish);

            dish.south();
            show(&dish);

            dish.east();
            show(&dish);

            let key = dish.state();

            if seen.contains(&key) {
                break;
            }

            seen.insert(key);
        }
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
    let args = aoc::parse_args();
    let mut puzzle = Puzzle::new();
    puzzle.configure(args.path.as_str());

    if args.verbose {
        puzzle.anim();
    } else {
        println!("{}", puzzle.part1());
        println!("{}", puzzle.part2());
    }
}
