//! [Day 10: Monitoring Station](https://adventofcode.com/2019/day/10)

use core::f64;
use std::cmp::Ordering;

use aoc::math::IntegerMathOps;
use aoc::Coord;
use rustc_hash::{FxHashMap, FxHashSet};

// Target of the giant rotating laser
struct Target {
    angle: f64,
    distance: i32,
    asteroid: Coord,
}

/// Compute the direction vector between two asteroids.
/// Asteroids are considred aligned if their coords are multiple of the same vector.
/// So, only one asteroid can be detected for this 'irreductible' vector (i.e. with gcd(x,y)=1).
fn insight_vector(asteroid: Coord, other: Coord) -> Coord {
    let vector = other - asteroid;
    let d = vector.x.gcd(vector.y).abs();
    Coord {
        x: vector.x / d,
        y: vector.y / d,
    }
}

/// Return the square of the distance between two asteroids.
const fn d_square(asteroid: Coord, other: Coord) -> i32 {
    (other.x - asteroid.x).pow(2) + (other.y - asteroid.y).pow(2)
}

///  Returns the angle in radians between two asteroids, 0 is north.
fn angle(asteroid: Coord, other: Coord) -> f64 {
    let a = f64::from(other.x - asteroid.x);
    let b = f64::from(asteroid.y - other.y);

    let a = a.atan2(b);

    if a < 0. {
        f64::consts::PI.mul_add(2., a) // a + 2π
    } else {
        a
    }
}

fn load_asteroids(data: &str) -> Vec<Coord> {
    let mut asteroids = Vec::new();
    for (xy, c) in &aoc::Grid::<u8>::parse(data) {
        if *c != b'.' {
            asteroids.push(xy);
        }
    }
    asteroids
}

/// Find the best location for a new monitoring station.
fn find_station(asteroids: &[Coord]) -> (usize, Coord) {
    let mut detected = Vec::new();
    for &asteroid in asteroids {
        let mut in_sight = FxHashSet::default();

        for &other in asteroids {
            if other != asteroid {
                in_sight.insert(insight_vector(asteroid, other));
            }
        }
        detected.push((in_sight.len(), asteroid));
    }

    *detected.iter().max_by_key(|x| x.0).unwrap()
}

/// Find the nth asteroid vaporized by the giant laser.
fn find_nth_vaporized(asteroids: &[Coord], station: Coord, mut vaporized: u32) -> Coord {
    let mut remaining_asteroids: FxHashSet<&Coord> = asteroids.iter().collect();

    while asteroids.len() > 1 {
        let mut targets: FxHashMap<Coord, Target> = FxHashMap::default();

        for &&asteroid in &remaining_asteroids {
            if asteroid != station {
                let v = insight_vector(station, asteroid);
                let distance = d_square(station, asteroid);
                let angle = angle(station, asteroid);

                // save or update the nearest target to the station
                targets
                    .entry(v)
                    .and_modify(|e| {
                        if e.distance > distance {
                            e.angle = angle;
                            e.distance = distance;
                            e.asteroid = asteroid;
                        }
                    })
                    .or_insert(Target {
                        angle,
                        distance,
                        asteroid,
                    });
            }
        }

        // Sort targets by angle
        let mut target_list: Vec<_> = targets.values().collect();
        target_list.sort_by(|a, b| a.angle.partial_cmp(&b.angle).unwrap_or(Ordering::Equal));

        for target in target_list {
            vaporized -= 1;
            if vaporized == 0 {
                return target.asteroid;
            }
            remaining_asteroids.remove(&target.asteroid);
        }
    }

    Coord { x: 0, y: 0 }
}

/// # Panics
/// over malformed input
#[must_use]
pub fn solve(data: &str) -> (usize, i32) {
    let asteroids = load_asteroids(data);

    // part 1
    let (part1, station) = find_station(&asteroids);

    // part 2
    let pos = find_nth_vaporized(&asteroids, station, 200);

    (part1, 100 * pos.x + pos.y)
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
    const SAMPLE_5: &str = include_str!("sample_5.txt");
    const SAMPLE_6: &str = include_str!("sample_6.txt");
    const SAMPLE_7: &str = include_str!("sample_7.txt");

    #[test]
    fn part1() {
        let asteroids = load_asteroids(SAMPLE_1);
        assert_eq!(find_station(&asteroids).1, Coord { x: 3, y: 4 });

        let asteroids = load_asteroids(SAMPLE_4);
        assert_eq!(find_station(&asteroids), (33, Coord { x: 5, y: 8 }));

        let asteroids = load_asteroids(SAMPLE_5);
        assert_eq!(find_station(&asteroids), (35, Coord { x: 1, y: 2 }));

        let asteroids = load_asteroids(SAMPLE_6);
        assert_eq!(find_station(&asteroids), (41, Coord { x: 6, y: 3 }));
    }

    #[test]
    fn part2() {
        let asteroids = load_asteroids(SAMPLE_7);

        let (max_detected, station) = find_station(&asteroids);
        assert_eq!(max_detected, 210);
        assert_eq!(station, Coord { x: 11, y: 13 });

        for (n, x, y) in [
            (1, 11, 12),
            (2, 12, 1),
            (3, 12, 2),
            (10, 12, 8),
            (20, 16, 0),
            (50, 16, 9),
            (100, 10, 16),
            (199, 9, 6),
            (200, 8, 2),
            (201, 10, 9),
            (299, 11, 1),
        ] {
            let vaporized = find_nth_vaporized(&asteroids, station, n);
            assert_eq!(vaporized, Coord { x, y });
        }
    }
}
