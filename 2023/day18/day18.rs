//! [Day 18: Lavaduct Lagoon](https://adventofcode.com/2023/day/18)

/// Use the Shoelace and Pick formulas to compute polygon area.
fn shoelace(points: &[(i64, i64)], contour_length: i64) -> i64 {
    let n = points.len();
    let mut area = 0;
    for i in 0..n {
        let x1 = points[i].0;
        let y0 = points[(i + n - 1) % n].1;
        let y2 = points[(i + 1) % n].1;

        area += x1 * (y0 - y2);
    }
    area.abs() / 2 + contour_length / 2 + 1
}

struct Puzzle {
    data: String,
}

impl Puzzle {
    const fn new() -> Self {
        Self {
            data: String::new(),
        }
    }

    /// Get the puzzle input.
    fn configure(&mut self, data: &str) {
        self.data = data.to_string();
    }

    /// Solve part one.
    fn part1(&self) -> i64 {
        let mut x = 0i64;
        let mut y = 0i64;
        let mut points = vec![];
        let mut length = 0;

        points.push((x, y));

        for line in self.data.lines() {
            let mut line = line.split_ascii_whitespace();
            let direction = line.next().unwrap();
            let steps: i64 = line.next().unwrap().parse().unwrap();

            match direction {
                "U" => y += steps,
                "D" => y -= steps,
                "R" => x += steps,
                "L" => x -= steps,
                _ => panic!(),
            }

            length += steps;
            points.push((x, y));
        }

        shoelace(&points, length)
    }

    /// Solve part two.
    fn part2(&self) -> i64 {
        let mut x = 0i64;
        let mut y = 0i64;
        let mut points = vec![];
        let mut length = 0;

        points.push((x, y));

        for line in self.data.lines() {
            let color = line.split_ascii_whitespace().nth(2).unwrap();
            let color = &color[2..8];
            let color = i64::from_str_radix(color, 16).unwrap();

            let direction = color % 16;
            let steps = color / 16;

            match direction {
                0 => x += steps, // R
                1 => y -= steps, // D
                2 => x -= steps, // L
                3 => y += steps, // U
                _ => panic!(),
            }

            length += steps;
            points.push((x, y));
        }

        shoelace(&points, length)
    }
}

fn main() {
    let args = aoc::parse_args();
    let mut puzzle = Puzzle::new();
    puzzle.configure(&args.input);
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
        puzzle.configure(&aoc::load_input_data("test.txt"));
        assert_eq!(puzzle.part1(), 62);
    }

    #[test]
    fn test02() {
        let mut puzzle = Puzzle::new();
        puzzle.configure(&aoc::load_input_data("test.txt"));
        assert_eq!(puzzle.part2(), 952408144115);
    }
}
