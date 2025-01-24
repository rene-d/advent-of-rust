//! [Day 11: Chronal Charge](https://adventofcode.com/2018/day/11)

struct Puzzle {
    serial_number: i32,
}

impl Puzzle {
    fn new(data: &str) -> Self {
        Self {
            serial_number: data.trim().parse().unwrap(),
        }
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
                        cell += power_level(xx, yy);
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

/// # Panics
#[must_use]
pub fn solve(data: &str) -> (String, String) {
    let puzzle = Puzzle::new(data);
    (puzzle.part1(), puzzle.part2())
}

pub fn main() {
    let args = aoc::parse_args();
    args.run(solve);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test01() {
        let puzzle = Puzzle::new("18");

        assert_eq!(puzzle.square_power(3), (33, 45, 29));
        assert_eq!(puzzle.part1(), "33,45");
        assert_eq!(puzzle.part2(), "90,269,16");
    }

    #[test]
    fn test02() {
        let puzzle = Puzzle::new("42");

        assert_eq!(puzzle.square_power(3), (21, 61, 30));
        assert_eq!(puzzle.part1(), "21,61");
        assert_eq!(puzzle.part2(), "232,251,12");
    }
}
