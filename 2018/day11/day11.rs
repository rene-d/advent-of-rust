//! [Day 11: Chronal Charge](https://adventofcode.com/2018/day/11)

use clap::Parser;

#[derive(Parser)]
struct Args {
    /// Puzzle input
    #[arg(default_value = "input.txt")]
    path: String,
}

struct Puzzle {
    serial_number: i32,
}

impl Puzzle {
    fn new() -> Puzzle {
        Puzzle { serial_number: 0 }
    }

    /// Get the puzzle input.
    fn configure(&mut self, path: &str) {
        let data = std::fs::read_to_string(path).unwrap();

        self.serial_number = data.trim().parse().unwrap();
    }

    fn square_power(&self, size: i32) -> (i32, i32, i32) {
        let power_level = |x, y| ((((x + 10) * y + self.serial_number) * (x + 10)) / 100) % 10 - 5;

        let mut max_cell = i32::MIN;
        let mut cell_x = 0;
        let mut cell_y = 0;

        for y in 1..=(300 - size + 1) {
            for x in 1..=(300 - size + 1) {
                let mut cell = 0;
                for xx in x..(x + size) {
                    for yy in y..(y + size) {
                        cell += power_level(xx, yy)
                    }
                }

                if max_cell < cell {
                    max_cell = cell;

                    cell_x = x;
                    cell_y = y;
                }
            }
        }

        (cell_x, cell_y, max_cell)
    }

    /// Solve part one.
    fn part1(&self) -> String {
        let (x, y, _) = self.square_power(3);
        format!("{x},{y}")
    }

    /// Solve part two.
    fn part2(&self) -> String {
        let mut max = (0, 0, 0, 0);

        for size in 3..300 {
            let (x, y, p) = self.square_power(size);

            if p < 0 {
                break;
            }

            if p > max.0 {
                max = (p, x, y, size);
            }
        }

        format!("{},{},{}", max.1, max.2, max.3)
    }
}

fn main() {
    let args = Args::parse();
    let mut puzzle = Puzzle::new();
    puzzle.configure(args.path.as_str());
    println!("{}", puzzle.part1());
    println!("{}", puzzle.part2());
}

/// Test from puzzle input
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test01() {
        let mut puzzle = Puzzle::new();
        puzzle.serial_number = 18;
        assert_eq!(puzzle.square_power(3), (33, 45, 29));
        assert_eq!(puzzle.part1(), "33,45");
        assert_eq!(puzzle.part2(), "90,269,16");
    }

    #[test]
    fn test02() {
        let mut puzzle = Puzzle::new();
        puzzle.serial_number = 42;
        assert_eq!(puzzle.square_power(3), (21, 61, 30));
        assert_eq!(puzzle.part1(), "21,61");
        assert_eq!(puzzle.part2(), "232,251,12");
    }
}
