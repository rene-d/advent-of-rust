//! [Day 23: Experimental Emergency Teleportation](https://adventofcode.com/2018/day/23)

use regex::Regex;
use std::ops::AddAssign;
use z3::ast::Int;

struct Nanobot {
    x: i64,
    y: i64,
    z: i64,
    r: u64,
}

impl Nanobot {
    fn from(text: &str) -> Self {
        let re = Regex::new(r"^pos=<(-?\d+),(-?\d+),(-?\d+)>, r=(-?\d+)$").unwrap();

        let caps = re.captures(text).unwrap();

        Self {
            x: caps.get(1).unwrap().as_str().parse().unwrap(),
            y: caps.get(2).unwrap().as_str().parse().unwrap(),
            z: caps.get(3).unwrap().as_str().parse().unwrap(),
            r: caps.get(4).unwrap().as_str().parse().unwrap(),
        }
    }

    fn dist(&self, other: &Self) -> u64 {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y) + self.z.abs_diff(other.z)
    }
}

struct Puzzle {
    nanobots: Vec<Nanobot>,
}

impl Puzzle {
    fn new() -> Puzzle {
        Puzzle { nanobots: vec![] }
    }

    /// Get the puzzle input.
    fn configure(&mut self, path: &str) {
        let data = std::fs::read_to_string(path).unwrap();
        for line in data.lines() {
            self.nanobots.push(Nanobot::from(line));
        }
    }

    /// Solve part one.
    fn part1(&self) -> usize {
        let strongest = self.nanobots.iter().max_by_key(|a| a.r).unwrap();

        self.nanobots
            .iter()
            .filter(|a| a.dist(strongest) <= strongest.r)
            .count()
    }

    /// Solve part two.
    fn part2(&self) -> i64 {
        let cfg = z3::Config::new();
        let ctx = z3::Context::new(&cfg);

        let o = z3::Optimize::new(&ctx);

        let x = Int::new_const(&ctx, "x");
        let y = Int::new_const(&ctx, "y");
        let z = Int::new_const(&ctx, "z");

        let one = Int::from_u64(&ctx, 1);
        let zero = Int::from_u64(&ctx, 0);
        let mut count = Int::from_u64(&ctx, 0);

        let dist = |px, py, pz| -> _ {
            let px = Int::from_i64(&ctx, px);
            let py = Int::from_i64(&ctx, py);
            let pz = Int::from_i64(&ctx, pz);

            let dx = x
                .ge(&px) // x-px if x>=px, px-x otherwises
                .ite(&Int::sub(&ctx, &[&x, &px]), &Int::sub(&ctx, &[&px, &x]));

            let dy = y
                .ge(&py)
                .ite(&Int::sub(&ctx, &[&y, &py]), &Int::sub(&ctx, &[&py, &y]));

            let dz = z
                .ge(&pz)
                .ite(&Int::sub(&ctx, &[&z, &pz]), &Int::sub(&ctx, &[&pz, &z]));

            Int::add(&ctx, &[&dx, &dy, &dz])
        };

        for bot in &self.nanobots {
            let d = dist(bot.x, bot.y, bot.z);

            count.add_assign(
                d // manhattan distance
                    .le(&Int::from_u64(&ctx, bot.r)) // <=r
                    .ite(&one, &zero), // count of
            );
        }

        o.maximize(&count);

        o.minimize(&dist(0, 0, 0));

        match o.check(&[]) {
            z3::SatResult::Sat => {
                if let Some(model) = o.get_model() {
                    let xx = model.eval(&x, true).unwrap();
                    let yy = model.eval(&y, true).unwrap();
                    let zz = model.eval(&z, true).unwrap();
                    return xx.as_i64().unwrap() + yy.as_i64().unwrap() + zz.as_i64().unwrap();
                }
            }
            z3::SatResult::Unsat => eprintln!("result Unsat"),
            z3::SatResult::Unknown => eprintln!("result Unknown"),
        }

        0
    }
}

fn main() {
    let args = aoc::parse_args();
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
        puzzle.configure("sample_1.txt");
        assert_eq!(puzzle.part1(), 7);
    }

    #[test]
    fn test02() {
        let mut puzzle = Puzzle::new();
        puzzle.configure("sample_2.txt");
        assert_eq!(puzzle.part2(), 36);
    }
}
