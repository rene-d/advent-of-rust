//! [Day 24: Never Tell Me The Odds](https://adventofcode.com/2023/day/24)

use clap::Parser;

#[derive(Parser)]
struct Args {
    /// Puzzle input
    #[arg(default_value = "input.txt")]
    path: String,
}

struct Hailstone {
    x: f64, // would be better should use fraction::Fraction
    y: f64,
    _z: f64,
    vx: f64,
    vy: f64,
    _vz: f64,
}

struct Puzzle {
    hailstones: Vec<Hailstone>,
}

impl Puzzle {
    fn new() -> Puzzle {
        Puzzle { hailstones: vec![] }
    }

    /// Get the puzzle input.
    fn configure(&mut self, path: &str) {
        let data = std::fs::read_to_string(path).unwrap();

        for line in data.lines() {
            let line: Vec<_> = line
                .split([',', '@'])
                .map(|x| x.trim().parse::<f64>().unwrap())
                .collect();

            self.hailstones.push(Hailstone {
                x: line[0],
                y: line[1],
                _z: line[2],
                vx: line[3],
                vy: line[4],
                _vz: line[5],
            });
        }
    }

    #[allow(clippy::many_single_char_names)]
    fn collisions(&self, area_min: f64, area_max: f64) -> u32 {
        let mut result = 0;

        let n = self.hailstones.len();

        for i in 0..n {
            for j in (i + 1)..n {
                let a = &self.hailstones[i];
                let b = &self.hailstones[j];

                let determinant = b.vy * a.vx - b.vx * a.vy;

                if determinant != 0. {
                    // point of intersection
                    let y = ((b.x - a.x) + a.y * a.vx / a.vy - b.y * b.vx / b.vy)
                        / (a.vx / a.vy - b.vx / b.vy);
                    let x = (y - a.y) * a.vx / a.vy + a.x;

                    // oriented intersection
                    let intersect_a = (x > a.x) == (a.vx > 0.);
                    let intersect_b = (x > b.x) == (b.vx > 0.);

                    if area_min <= x
                        && x <= area_max
                        && area_min <= y
                        && y <= area_max
                        && intersect_a
                        && intersect_b
                    {
                        result += 1;
                    }
                }
            }
        }

        result
    }

    /// Solve part one.
    fn part1(&self) -> u32 {
        self.collisions(200_000_000_000_000.0, 400_000_000_000_000.0)
    }

    /// Solve part two.
    #[allow(clippy::unused_self)]
    #[allow(dead_code)]
    fn part2(&self) -> u32 {
        0
    }
}

fn main() {
    let args = Args::parse();
    let mut puzzle = Puzzle::new();
    puzzle.configure(args.path.as_str());
    println!("{}", puzzle.part1());
    // println!("{}", puzzle.part2());
}

/// Test from puzzle input
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test01() {
        let mut puzzle = Puzzle::new();
        puzzle.configure("test.txt");
        assert_eq!(puzzle.collisions(7.0, 27.0), 2);
    }

    #[test]
    fn test02() {
        let mut puzzle = Puzzle::new();
        puzzle.configure("test.txt");
        assert_eq!(puzzle.part2(), 0);
    }
}
