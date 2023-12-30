//! [Day 24: Never Tell Me The Odds](https://adventofcode.com/2023/day/24)

use clap::Parser;
use fraction::{GenericFraction, Zero};

// i64 is not enough... 👺
// thus, it's quite useless to use rationals
// (it works in f64 too 😳)
type Q128 = GenericFraction<i128>;


#[derive(Parser)]
struct Args {
    /// Puzzle input
    #[arg(default_value = "input.txt")]
    path: String,
}

struct Hailstone {
    x: Q128, // for use in part 1
    y: Q128,
    vx: Q128,
    vy: Q128,
    p: [i64; 3], // for use in part 2
    v: [i64; 3],
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
            let values: Vec<_> = line
                .split([',', '@', ' '])
                .filter_map(|x| x.parse::<i64>().ok())
                .collect();

            self.hailstones.push(Hailstone {
                x: Q128::from(values[0]),
                y: Q128::from(values[1]),
                vx: Q128::from(values[3]),
                vy: Q128::from(values[4]),
                p: values[0..3].try_into().unwrap(),
                v: values[3..6].try_into().unwrap(),
            });
        }
    }

    #[allow(clippy::many_single_char_names)]
    fn collisions(&self, area_min: i64, area_max: i64) -> u32 {
        let area_min = Q128::from(area_min);
        let area_max = Q128::from(area_max);

        let mut result = 0;

        let n = self.hailstones.len();

        for i in 0..(n - 1) {
            for j in (i + 1)..n {
                let a = &self.hailstones[i];
                let b = &self.hailstones[j];

                let determinant = b.vy * a.vx - b.vx * a.vy;

                if !determinant.is_zero() {
                    // point of intersection

                    let x = (a.y * a.vx * b.vx - a.x * a.vy * b.vx - b.y * a.vx * b.vx
                        + b.x * a.vx * b.vy)
                        / determinant;

                    let y = (a.y * a.vx * b.vy - a.x * a.vy * b.vy - b.y * a.vy * b.vx
                        + b.x * a.vy * b.vy)
                        / determinant;

                    // oriented intersection
                    let intersect_a = (x > a.x) == (a.vx > Q128::zero());
                    let intersect_b = (x > b.x) == (b.vx > Q128::zero());

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
        self.collisions(200_000_000_000_000, 400_000_000_000_000)
    }

    /// Solve part two.
    fn part2(&self) -> i64 {
        use z3::ast::*;

        let cfg = z3::Config::new();
        let ctx = z3::Context::new(&cfg);

        let solver = z3::Solver::new(&ctx);

        let mut p = vec![];
        let mut v = vec![];
        let mut t = vec![];

        // Nota: some inputs mess the solver if I use the z3::ast::Int datatype
        for _ in 0..3 {
            p.push(Real::fresh_const(&ctx, "p")); // positions
            v.push(Real::fresh_const(&ctx, "v")); // velocity
        }

        for _ in 0..self.hailstones.len() {
            t.push(Real::fresh_const(&ctx, "t")); // time
        }

        let zero = Int::from_i64(&ctx, 0);
        let zero = Real::from_int(&zero);

        // normally, 3*3 constraints are sufficient
        // with 4, the model should still be satisfiable, except a problem...
        for (i, hail) in self.hailstones.iter().take(4).enumerate() {
            // constraint: t[i] >= 0
            solver.assert(&t[i].ge(&zero));

            // constraint: hail.p[i] + t[i] * hail.v[i] == p[i] + t[i] * v[i]
            for j in 0..3 {
                let p_j = Real::from_int(&Int::from_i64(&ctx, hail.p[j]));
                let v_j = Real::from_int(&Int::from_i64(&ctx, hail.v[j]));

                let left = &p_j + &t[i] * &v_j;
                let right = &p[j] + &t[i] * &v[j];
                solver.assert(&left._eq(&right));
            }
        }

        match solver.check() {
            z3::SatResult::Sat => {
                if let Some(model) = solver.get_model() {
                    let result: i64 = p
                        .iter()
                        .filter_map(|i| model.eval(i, true))
                        .filter_map(|i| i.as_real())
                        .filter_map(|(num, den)| (den == 1).then_some(num))
                        .sum();

                    return result;
                }
            }
            z3::SatResult::Unsat => eprintln!("z3::SatResult::Unsat"),
            z3::SatResult::Unknown => eprintln!("z3::SatResult::Unknown"),
        }
        0
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
        puzzle.configure("test.txt");
        assert_eq!(puzzle.collisions(7, 27), 2);
    }

    #[test]
    fn test02() {
        let mut puzzle = Puzzle::new();
        puzzle.configure("test.txt");
        assert_eq!(puzzle.part2(), 47);
    }
}
