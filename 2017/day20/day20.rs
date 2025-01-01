//! [Day 20: Particle Swarm](https://adventofcode.com/2017/day/20)

use rustc_hash::FxHashMap;

use regex::Regex;

#[derive(Copy, Clone)]
struct Particle {
    px: i64,
    py: i64,
    pz: i64,
    vx: i64,
    vy: i64,
    vz: i64,
    ax: i64,
    ay: i64,
    az: i64,
}

struct Puzzle {
    particles: Vec<Particle>,
}

impl Puzzle {
    /// Initialize from the puzzle input.
    fn new(data: &str) -> Self {
        let re = Regex::new(r"^p=<(-?\d+),(-?\d+),(-?\d+)>, v=<(-?\d+),(-?\d+),(-?\d+)>, a=<(-?\d+),(-?\d+),(-?\d+)>$").unwrap();

        let particles = data
            .lines()
            .map(|line| {
                let caps = re.captures(line).unwrap();

                Particle {
                    px: caps[1].parse().unwrap(),
                    py: caps[2].parse().unwrap(),
                    pz: caps[3].parse().unwrap(),
                    vx: caps[4].parse().unwrap(),
                    vy: caps[5].parse().unwrap(),
                    vz: caps[6].parse().unwrap(),
                    ax: caps[7].parse().unwrap(),
                    ay: caps[8].parse().unwrap(),
                    az: caps[9].parse().unwrap(),
                }
            })
            .collect();

        Self { particles }
    }

    /// Solve part one.
    fn part1(&self) -> usize {
        let t = 1000;

        let mut min_dist = i64::MAX;
        let mut min_dist_idx = 0;

        for (i, p) in self.particles.iter().enumerate() {
            let x = p.px + p.vx * t + p.ax * t * (t + 1) / 2;
            let y = p.py + p.vy * t + p.ay * t * (t + 1) / 2;
            let z = p.pz + p.vz * t + p.az * t * (t + 1) / 2;
            let dist = x.abs() + y.abs() + z.abs();
            if min_dist > dist {
                min_dist = dist;
                min_dist_idx = i;
            }
        }

        min_dist_idx
    }

    /// Solve part two.
    fn part2(&self) -> usize {
        let mut particles = self.particles.clone();

        for t in 0..1000 {
            let mut collisions: FxHashMap<(i64, i64, i64), Vec<usize>> = FxHashMap::default();

            for (i, p) in particles.iter().enumerate() {
                let x = p.px + p.vx * t + p.ax * t * (t + 1) / 2;
                let y = p.py + p.vy * t + p.ay * t * (t + 1) / 2;
                let z = p.pz + p.vz * t + p.az * t * (t + 1) / 2;
                collisions.entry((x, y, z)).or_default().push(i);
            }

            let new_particles = collisions
                .values()
                .filter(|v| v.len() == 1)
                .map(|v| particles[v[0]])
                .collect();
            particles = new_particles;
        }

        particles.len()
    }
}

fn main() {
    let args = aoc::parse_args();
    let puzzle = Puzzle::new(&args.input);
    println!("{}", puzzle.part1());
    println!("{}", puzzle.part2());
}

/// Test from puzzle input
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse() {
        let puzzle = Puzzle::new("p=<683,2541,-1586>, v=<103,363,-226>, a=<-4,-27,14>");
        assert_eq!(puzzle.particles.len(), 1);

        let p = puzzle.particles[0];
        assert_eq!(p.px, 683);
        assert_eq!(p.vy, 363);
        assert_eq!(p.az, 14);
    }
}
