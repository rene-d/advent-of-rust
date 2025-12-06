//! [Day 19: Beacon Scanner](https://adventofcode.com/2021/day/19)

use rustc_hash::FxHashMap;
use rustc_hash::FxHashSet;
use scan_fmt::scan_fmt;
use std::fmt;

#[derive(Hash, Eq, PartialEq, Clone)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

/// implement `fmt::Debug` for Point
impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("")
            .field(&self.x)
            .field(&self.y)
            .field(&self.z)
            .finish()
    }
}

pub fn main() {
    let args = aoc::parse_args();
    args.run(solve);
}

/// # Panics
#[must_use]
pub fn solve(data: &str) -> (usize, i32) {
    // load puzzle data
    let scanners = load_scanners(data);

    // the list of different beacons
    let mut beacons: FxHashSet<Point> = FxHashSet::default();
    for p in &scanners[0] {
        // add beacons from scanner 0
        beacons.insert(p.clone());
    }

    // relative coordinates to scanner 0 of other scanners
    let mut scanner_coords = vec![Point { x: 0, y: 0, z: 0 }; scanners.len()];

    // pending scanner analysis
    let mut pending: FxHashSet<usize> = FxHashSet::default();
    for i in 1..scanners.len() {
        // add beacons from all other scanners
        pending.insert(i);
    }

    // compute all rotated coordinates of beacons
    let mut scanner_rotated_list: Vec<Vec<Vec<Point>>> = Vec::new();
    for scanner in &scanners {
        let mut scanner_rotated: Vec<Vec<Point>> = Vec::new();

        for rotation in 0..24 {
            let r: Vec<Point> = scanner
                .iter()
                .map(|point| rotate(point, rotation))
                .collect();
            scanner_rotated.push(r);
        }
        scanner_rotated_list.push(scanner_rotated);
    }

    while !pending.is_empty() {
        let mut found = usize::MAX;

        for scanner_id in &pending {
            let mut g_scan: Vec<Point> = Vec::new();
            for p in &beacons {
                let point = Point {
                    x: p.x + scanner_coords[0].x,
                    y: p.y + scanner_coords[0].y,
                    z: p.z + scanner_coords[0].z,
                };
                g_scan.push(point);
            }

            for b_scan in scanner_rotated_list[*scanner_id].iter().take(24) {

                let mut match_points: FxHashMap<Point, i32> = FxHashMap::default();

                for bi in b_scan.iter().take(scanners[*scanner_id].len()) {
                    for gi in &g_scan {
                        let dx = -bi.x + gi.x;
                        let dy = -bi.y + gi.y;
                        let dz = -bi.z + gi.z;

                        let p = Point {
                            x: dx,
                            y: dy,
                            z: dz,
                        };

                        let n = match_points.get(&p).unwrap_or(&0);
                        match_points.insert(p, n + 1);
                    }
                }

                for (point, count) in match_points {
                    if count >= 12 {
                        scanner_coords[*scanner_id] = point.clone();

                        for p in b_scan {
                            let q = Point {
                                x: point.x + p.x,
                                y: point.y + p.y,
                                z: point.z + p.z,
                            };
                            beacons.insert(q);
                        }

                        found = *scanner_id;
                    }
                }
            }

            if found != usize::MAX {
                break;
            }
        }

        assert!(found != usize::MAX, "no beacon found");

        pending.remove(&found);
    }

    let mut manhattan = 0;
    for p1 in &scanner_coords {
        for p2 in &scanner_coords {
            let distance = (p1.x - p2.x).abs() + (p1.y - p2.y).abs() + (p1.z - p2.z).abs();
            if distance > manhattan {
                manhattan = distance;
            }
        }
    }

    (beacons.len(), manhattan)
}

fn load_scanners(data: &str) -> Vec<Vec<Point>> {
    let mut scanners: Vec<Vec<Point>> = Vec::new();
    let mut beacons: Vec<Point> = Vec::new();

    for line in data.lines() {
        if let Ok(_id) = scan_fmt!(&line, "--- scanner {} ---", i32)
            && !beacons.is_empty() {
                scanners.push(beacons);
                beacons = Vec::new();
            }

        if let Ok((x, y, z)) = scan_fmt!(&line, "{},{},{}", i32, i32, i32) {
            let p = Point { x, y, z };
            beacons.push(p);
        }
    }
    if !beacons.is_empty() {
        scanners.push(beacons);
    }

    scanners
}

const fn rotate(point: &Point, rotation: usize) -> Point {
    let x = point.x;
    let y = point.y;
    let z = point.z;

    let points = [
        (-x, -y, z),
        (-x, -z, -y),
        (-x, y, -z),
        (-x, z, y),
        (-y, -x, -z),
        (-y, -z, x),
        (-y, x, z),
        (-y, z, -x),
        (-z, -x, y),
        (-z, -y, -x),
        (-z, x, -y),
        (-z, y, x),
        (x, -y, -z),
        (x, -z, y),
        (x, y, z),
        (x, z, -y),
        (y, -x, z),
        (y, -z, -x),
        (y, x, -z),
        (y, z, x),
        (z, -x, -y),
        (z, -y, x),
        (z, x, y),
        (z, y, -x),
    ];

    let p = points[rotation];
    Point {
        x: p.0,
        y: p.1,
        z: p.2,
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
