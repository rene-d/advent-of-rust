//! [Day 12: The N-Body Problem](https://adventofcode.com/2019/day/12)

use aoc::math::IntegerMathOps;

type Coord = [i32; 3];

const ZERO: Coord = [0, 0, 0];

// Apply gravity to the system of moons and update velocities
fn apply_gravity(moons: &mut [Coord], velocities: &mut [Coord]) {
    for (i, a) in moons.iter().enumerate() {
        for (j, b) in moons.iter().enumerate() {
            if i < j {
                for k in 0..3 {
                    if a[k] > b[k] {
                        velocities[i][k] -= 1;
                        velocities[j][k] += 1;
                        continue; // it's ugly but it shuts up clippy
                    }
                    if a[k] < b[k] {
                        velocities[i][k] += 1;
                        velocities[j][k] -= 1;
                    }
                }
            }
        }
    }

    for (moon, velocity) in moons.iter_mut().zip(velocities.iter()) {
        for k in 0..3 {
            moon[k] += velocity[k];
        }
    }
}

const fn energy(p: &Coord) -> i32 {
    p[0].abs() + p[1].abs() + p[2].abs()
}

fn system_energy(moons: &[Coord], velocities: &[Coord]) -> i32 {
    moons
        .iter()
        .zip(velocities)
        .map(|(moon, vel)| energy(moon) * energy(vel))
        .sum()
}

fn compute_energy(moons: &[Coord], steps: usize) -> i32 {
    let mut moons = moons.to_vec();
    let mut velocities = vec![ZERO; moons.len()];

    for _ in 0..steps {
        apply_gravity(&mut moons, &mut velocities);
    }

    system_energy(&moons, &velocities)
}

struct Puzzle {
    moons: Vec<Coord>,
}

impl Puzzle {
    /// Initialize from the puzzle input.
    fn new(data: &str) -> Self {
        let re = regex::Regex::new(r"<x=(-?\d+), y=(-?\d+), z=(-?\d+)>").unwrap();

        let moons = data
            .lines()
            .map(|line| {
                let caps = re.captures(line).unwrap();
                [
                    caps[1].parse().unwrap(),
                    caps[2].parse().unwrap(),
                    caps[3].parse().unwrap(),
                ]
            })
            .collect();
        Self { moons }
    }

    /// Solve part one.
    fn part1(&self) -> i32 {
        compute_energy(&self.moons, 1000)
    }

    /// Solve part two.
    fn part2(&self) -> u64 {
        let mut moons = self.moons.clone();
        let mut velocities = vec![ZERO; moons.len()];

        let mut step = 0;
        let mut steps = [0; 3];
        let mut remaining = 3;

        while remaining != 0 {
            apply_gravity(&mut moons, &mut velocities);
            step += 1;

            for k in 0..3 {
                if steps[k] == 0 {
                    // look for cycles for each coordinate since they are independent
                    if self
                        .moons
                        .iter()
                        .enumerate()
                        .all(|(i, initial)| initial[k] == moons[i][k] && velocities[i][k] == 0)
                    {
                        steps[k] = step;
                        remaining -= 1;
                        if remaining == 0 {
                            break;
                        }
                    }
                }
            }
        }

        steps[0].lcm(steps[1].lcm(steps[2]))
    }
}

/// # Panics
/// over malformed input
#[must_use]
pub fn solve(data: &str) -> (i32, u64) {
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

    const SAMPLE_1: &str = include_str!("sample_1.txt");
    const SAMPLE_4: &str = include_str!("sample_4.txt");

    #[test]
    fn part1() {
        let puzzle = Puzzle::new(SAMPLE_1);
        assert_eq!(compute_energy(&puzzle.moons, 10), 179);

        let puzzle = Puzzle::new(SAMPLE_4);
        assert_eq!(compute_energy(&puzzle.moons, 100), 1940);
    }

    #[test]
    fn part2() {
        let puzzle = Puzzle::new(SAMPLE_4);
        assert_eq!(puzzle.part2(), 4686774924);
    }
}
