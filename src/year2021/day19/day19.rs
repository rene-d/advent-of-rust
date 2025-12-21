//! [Day 19: Beacon Scanner](https://adventofcode.com/2021/day/19)

use rayon::prelude::*;
use rustc_hash::{FxHashMap, FxHashSet};
use std::collections::VecDeque;

#[derive(Hash, Eq, PartialEq, Clone, Copy)]
struct Point {
    x: i16,
    y: i16,
    z: i16,
}

impl Point {
    const fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }

    const fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }

    fn dist_sq(self, other: Self) -> u32 {
        let dx = u32::from(self.x.abs_diff(other.x));
        let dy = u32::from(self.y.abs_diff(other.y));
        let dz = u32::from(self.z.abs_diff(other.z));
        dx * dx + dy * dy + dz * dz
    }

    const fn manhattan(self, other: Self) -> u16 {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y) + self.z.abs_diff(other.z)
    }
}

pub fn main() {
    let args = aoc::parse_args();
    args.run(solve);
}

struct Scanner {
    beacons: Vec<Point>,
    fingerprint: FxHashSet<u32>,
}

impl Scanner {
    fn new(beacons: Vec<Point>) -> Self {
        let mut fingerprint = FxHashSet::default();
        for i in 0..beacons.len() {
            for j in (i + 1)..beacons.len() {
                fingerprint.insert(beacons[i].dist_sq(beacons[j]));
            }
        }
        Self {
            beacons,
            fingerprint,
        }
    }
}

/// # Panics
#[must_use]
pub fn solve(data: &str) -> (usize, u16) {
    let mut scanners = load_scanners(data);

    // Scanners that are positioned correctly relative to scanner 0
    // but haven't been used yet to find other scanners.
    let mut queue: VecDeque<Scanner> = VecDeque::new();

    // Final list of scanners with their absolute positions
    // We only need positions to answer part 2, but we also accumulate all beacons for part 1
    let mut scanner_coords: Vec<Point> = Vec::with_capacity(scanners.len());

    // Move scanner 0 to solved
    let s0 = scanners.remove(0);
    scanner_coords.push(Point { x: 0, y: 0, z: 0 });

    // Global set of unique beacons (absolute coordinates)
    let mut all_beacons: FxHashSet<Point> = s0.beacons.iter().copied().collect();

    queue.push_back(s0);

    while let Some(solved) = queue.pop_front() {
        let mut still_pending = Vec::new();

        for candidate in scanners {
            // Fingerprint check: overlap of distances must be at least binomial(12, 2) = 66
            let intersection_count = solved
                .fingerprint
                .intersection(&candidate.fingerprint)
                .count();
            if intersection_count < 66 {
                still_pending.push(candidate);
                continue;
            }

            // Try to align
            if let Some((transformed_beacons, scanner_pos)) = try_align(&solved, &candidate) {
                // Success!
                scanner_coords.push(scanner_pos);
                for b in &transformed_beacons {
                    all_beacons.insert(*b);
                }

                let new_solved = Scanner::new(transformed_beacons);
                queue.push_back(new_solved);
            } else {
                still_pending.push(candidate);
            }
        }
        scanners = still_pending;
    }

    let mut manhattan = 0;
    for i in 0..scanner_coords.len() {
        for j in (i + 1)..scanner_coords.len() {
            let dist = scanner_coords[i].manhattan(scanner_coords[j]);
            if dist > manhattan {
                manhattan = dist;
            }
        }
    }

    (all_beacons.len(), manhattan)
}

fn try_align(solved: &Scanner, candidate: &Scanner) -> Option<(Vec<Point>, Point)> {
    // Only try 24 rotations
    // For each rotation, check if we get >= 12 matches with translation

    let candidate_beacons = &candidate.beacons;
    let solved_beacons = &solved.beacons;

    (0..24).into_par_iter().find_map_any(|rot| {
        let rotated_candidate: Vec<Point> =
            candidate_beacons.iter().map(|&p| rotate(p, rot)).collect();

        let mut offset_counts: FxHashMap<Point, usize> = FxHashMap::default();

        for &rot_p in &rotated_candidate {
            for &solved_p in solved_beacons {
                let offset = solved_p.sub(rot_p);
                *offset_counts.entry(offset).or_default() += 1;
            }
        }

        for (offset, count) in offset_counts {
            if count >= 12 {
                let transformed: Vec<Point> =
                    rotated_candidate.iter().map(|&p| p.add(offset)).collect();
                return Some((transformed, offset));
            }
        }

        None
    })
}

fn load_scanners(data: &str) -> Vec<Scanner> {
    let mut scanners: Vec<Scanner> = Vec::new();
    let mut beacons: Vec<Point> = Vec::new();

    for line in data.lines() {
        if line.is_empty() {
            continue;
        }

        if line.starts_with("--- scanner ") {
            if !beacons.is_empty() {
                scanners.push(Scanner::new(beacons));
                beacons = Vec::new();
            }
            continue;
        }

        let parts: Vec<_> = line
            .split(',')
            .filter_map(|s| s.parse::<i16>().ok())
            .collect();
        if parts.len() == 3 {
            beacons.push(Point {
                x: parts[0],
                y: parts[1],
                z: parts[2],
            });
        }
    }
    if !beacons.is_empty() {
        scanners.push(Scanner::new(beacons));
    }

    scanners
}

const fn rotate(point: Point, rotation: u32) -> Point {
    let x = point.x;
    let y = point.y;
    let z = point.z;

    let (rx, ry, rz) = match rotation {
        0 => (-x, -y, z),
        1 => (-x, -z, -y),
        2 => (-x, y, -z),
        3 => (-x, z, y),
        4 => (-y, -x, -z),
        5 => (-y, -z, x),
        6 => (-y, x, z),
        7 => (-y, z, -x),
        8 => (-z, -x, y),
        9 => (-z, -y, -x),
        10 => (-z, x, -y),
        11 => (-z, y, x),
        12 => (x, -y, -z),
        13 => (x, -z, y),
        // 14 => (x, y, z),  // Identity: match arm
        15 => (x, z, -y),
        16 => (y, -x, z),
        17 => (y, -z, -x),
        18 => (y, x, -z),
        19 => (y, z, x),
        20 => (z, -x, -y),
        21 => (z, -y, x),
        22 => (z, x, y),
        23 => (z, y, -x),
        _ => (x, y, z),
    };

    Point {
        x: rx,
        y: ry,
        z: rz,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = include_str!("test.txt");

    #[test]
    fn test_puzzle() {
        let parts = solve(TEST_INPUT);

        assert_eq!(parts.0, 79);
        assert_eq!(parts.1, 3621);
    }
}
