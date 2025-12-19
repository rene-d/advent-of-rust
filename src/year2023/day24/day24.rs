//! [Day 24: Never Tell Me The Odds](https://adventofcode.com/2023/day/24)

use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::{Signed, ToPrimitive, Zero};
use rayon::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq)]
struct Vec3 {
    x: i64,
    y: i64,
    z: i64,
}

struct Vec2 {
    x: i128,
    y: i128,
}

impl Vec2 {
    fn from(v: &Vec3) -> Self {
        Self {
            x: i128::from(v.x),
            y: i128::from(v.y),
        }
    }
}

struct VecBR {
    x: BigRational, // Nota: GenericFraction<i128> is not always sufficient
    y: BigRational,
    z: BigRational,
}

impl VecBR {
    fn from(v: &Vec3) -> Self {
        Self {
            x: BigRational::from_integer(BigInt::from(v.x)),
            y: BigRational::from_integer(BigInt::from(v.y)),
            z: BigRational::from_integer(BigInt::from(v.z)),
        }
    }

    /// Cross product
    fn cross(self, other: &Self) -> Self {
        Self {
            x: (&self.y * &other.z) - (&self.z * &other.y),
            y: (&self.z * &other.x) - (&self.x * &other.z),
            z: (&self.x * &other.y) - (&self.y * &other.x),
        }
    }

    /// Substraction
    fn sub(&self, other: &Self) -> Self {
        Self {
            x: &self.x - &other.x,
            y: &self.y - &other.y,
            z: &self.z - &other.z,
        }
    }
}

#[derive(Debug, Clone)]
struct Hailstone {
    p: Vec3,
    v: Vec3,
}

impl Hailstone {
    fn parse(line: &str) -> Option<Self> {
        let mut parts = line
            .split([',', '@', ' '])
            .filter_map(|x| x.parse::<i64>().ok());

        Some(Self {
            p: Vec3 {
                x: parts.next()?,
                y: parts.next()?,
                z: parts.next()?,
            },
            v: Vec3 {
                x: parts.next()?,
                y: parts.next()?,
                z: parts.next()?,
            },
        })
    }
}

struct Puzzle {
    hailstones: Vec<Hailstone>,
}

impl Puzzle {
    fn new(data: &str) -> Self {
        Self {
            hailstones: data.lines().filter_map(Hailstone::parse).collect(),
        }
    }

    fn collisions(&self, area_min: i64, area_max: i64) -> usize {
        let area_min = i128::from(area_min);
        let area_max = i128::from(area_max);
        let n = self.hailstones.len();

        (0..(n - 1))
            .par_bridge() // to gain a few microseconds 🥸
            .map(|i| {
                ((i + 1)..n)
                    .filter(|&j| {
                        let a = Vec2::from(&self.hailstones[i].p);
                        let b = Vec2::from(&self.hailstones[j].p);

                        let av = Vec2::from(&self.hailstones[i].v);
                        let bv = Vec2::from(&self.hailstones[j].v);

                        let det = bv.y * av.x - bv.x * av.y;

                        if det.is_zero() {
                            false
                        } else {
                            // point of intersection

                            // det * x
                            let det_x = a.y * av.x * bv.x - a.x * av.y * bv.x - b.y * av.x * bv.x
                                + b.x * av.x * bv.y;

                            // det * y
                            let det_y = a.y * av.x * bv.y - a.x * av.y * bv.y - b.y * av.y * bv.x
                                + b.x * av.y * bv.y;

                            // to simplify comparisons:
                            //  min ≤ x÷det  ⇔  min•|det| ≤ x•sign(det)
                            //  etc.
                            let det_x = det_x * det.signum();
                            let det_y = det_y * det.signum();
                            let det = det.abs();

                            // oriented intersection
                            let intersect_a = (det_x > det * a.x) == (av.x > 0);
                            let intersect_b = (det_x > det * b.x) == (bv.x > 0);

                            area_min * det <= det_x
                                && det_x <= area_max * det
                                && area_min * det <= det_y
                                && det_y <= area_max * det
                                && intersect_a
                                && intersect_b
                        }
                    })
                    .count()
            })
            .sum()
    }

    /// Solve part one.
    fn part1(&self) -> usize {
        self.collisions(200_000_000_000_000, 400_000_000_000_000)
    }

    /// Solve part two.
    fn part2(&self) -> i64 {
        if self.hailstones.len() < 3 {
            return 0;
        }

        let mut matrix = vec![vec![BigRational::zero(); 7]; 6];

        let build_rows =
            |h1: &Hailstone, h2: &Hailstone, row_offset: usize, m: &mut Vec<Vec<BigRational>>| {
                let h1_p = VecBR::from(&h1.p);
                let h2_p = VecBR::from(&h2.p);

                let h1_v = VecBR::from(&h1.v);
                let h2_v = VecBR::from(&h2.v);

                let dv = h1_v.sub(&h2_v);
                let dp = h1_p.sub(&h2_p);

                let c1 = h1_p.cross(&h1_v);
                let c2 = h2_p.cross(&h2_v);
                let c = c1.sub(&c2);

                // [  0,      dv.z,  -dv.y,      0,  -dp.z,   dp.y,   c.x ]
                // [ -dv.z,      0,   dv.x,   dp.z,      0,  -dp.x,   c.y ]
                // [  dv.y,  -dv.x,      0,  -dp.y,   dp.x,      0,   c.z ]

                m[row_offset][1] = dv.z.clone();
                m[row_offset][2] = -dv.y.clone();
                m[row_offset][4] = -dp.z.clone();
                m[row_offset][5] = dp.y.clone();
                m[row_offset][6] = c.x;

                m[row_offset + 1][0] = -dv.z;
                m[row_offset + 1][2] = dv.x.clone();
                m[row_offset + 1][3] = dp.z;
                m[row_offset + 1][5] = -dp.x.clone();
                m[row_offset + 1][6] = c.y;

                m[row_offset + 2][0] = dv.y;
                m[row_offset + 2][1] = -dv.x;
                m[row_offset + 2][3] = -dp.y;
                m[row_offset + 2][4] = dp.x;
                m[row_offset + 2][6] = c.z;
            };

        build_rows(&self.hailstones[0], &self.hailstones[1], 0, &mut matrix);
        build_rows(&self.hailstones[0], &self.hailstones[2], 3, &mut matrix);

        solve_gaussian(&mut matrix);

        let px = &matrix[0][6];
        let py = &matrix[1][6];
        let pz = &matrix[2][6];

        let sum = px + py + pz;
        sum.to_integer().to_i64().expect("Result too large for i64")
    }
}

#[allow(clippy::needless_range_loop)]
fn solve_gaussian(a: &mut [Vec<BigRational>]) {
    let n = 6;
    for i in 0..n {
        let mut pivot = i;
        for j in i + 1..n {
            if a[j][i].abs() > a[pivot][i].abs() {
                pivot = j;
            }
        }
        a.swap(i, pivot);

        for j in i + 1..n {
            if a[j][i].is_zero() {
                continue;
            }
            let factor = &a[j][i] / &a[i][i];
            for k in i..=n {
                let val = &factor * &a[i][k];
                a[j][k] = &a[j][k] - val;
            }
        }
    }

    for i in (0..n).rev() {
        let mut sum = BigRational::zero();
        for j in i + 1..n {
            sum += &a[i][j] * &a[j][6];
        }
        a[i][6] = (&a[i][6] - sum) / &a[i][i];
    }
}

pub fn solve(data: &str) -> (usize, i64) {
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

    const TEST_INPUT: &str = include_str!("test.txt");

    #[test]
    fn test01() {
        let puzzle = Puzzle::new(TEST_INPUT);
        assert_eq!(puzzle.collisions(7, 27), 2);
    }

    #[test]
    fn test02() {
        let puzzle = Puzzle::new(TEST_INPUT);
        assert_eq!(puzzle.part2(), 47);
    }
}
