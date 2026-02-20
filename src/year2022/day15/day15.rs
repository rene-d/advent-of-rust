//! [Day 15: Beacon Exclusion Zone](https://adventofcode.com/2022/day/15)

use rustc_hash::FxHashSet;

/// Computes the [Manhattan distance](https://en.wikipedia.org/wiki/Taxicab_geometry) between two points
const fn manhattan(ax: i64, ay: i64, bx: i64, by: i64) -> i64 {
    (ax - bx).abs() + (ay - by).abs()
}

/// Extracts all integers from a string, including negative ones.
fn extract_i64(s: &str) -> impl Iterator<Item = i64> + '_ {
    s.split(|c: char| !(c.is_ascii_digit() || c == '-'))
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<i64>().unwrap())
}

struct Puzzle {
    sensors: Vec<(i64, i64, i64)>, // list of (x,y,distance from nearest beacon)
    beacons: FxHashSet<(i64, i64)>, // set of beacons
    field_size: i64,               // 20 or 4000000 depends on test or puzzle
}

impl Puzzle {
    fn new(data: &str, is_test: bool) -> Self {
        let field_size = if is_test { 20 } else { 4_000_000 };

        let mut sensors = Vec::new();
        let mut beacons = FxHashSet::default();

        for line in data.lines() {
            let mut iter = extract_i64(line);
            if let (Some(sx), Some(sy), Some(bx), Some(by)) =
                (iter.next(), iter.next(), iter.next(), iter.next())
            {
                let d = manhattan(sx, sy, bx, by);
                sensors.push((sx, sy, d));
                beacons.insert((bx, by));
            }
        }

        Self {
            sensors,
            beacons,
            field_size,
        }
    }

    // Solves part one
    fn part1(&self) -> i64 {
        let y = self.field_size / 2;
        let mut intervals: Vec<(i64, i64)> = Vec::with_capacity(self.sensors.len());

        for (sx, sy, d) in &self.sensors {
            let dy = (sy - y).abs();
            if dy <= *d {
                let dx = d - dy;
                intervals.push((sx - dx, sx + dx));
            }
        }

        intervals.sort_unstable_by(|a, b| a.0.cmp(&b.0));

        let mut merged: Vec<(i64, i64)> = Vec::with_capacity(intervals.len());
        for interval in intervals {
            if merged.is_empty() {
                merged.push(interval);
            } else {
                let last = merged.last_mut().unwrap();
                // Merge if overlapping or adjacent
                if interval.0 <= last.1 + 1 {
                    last.1 = std::cmp::max(last.1, interval.1);
                } else {
                    merged.push(interval);
                }
            }
        }

        let mut count: i64 = 0;
        for (start, end) in &merged {
            count += end - start + 1;
        }

        // Subtract beacons that are in the coverage area
        for (bx, by) in &self.beacons {
            if *by == y {
                for (start, end) in &merged {
                    if *bx >= *start && *bx <= *end {
                        count -= 1;
                        break;
                    }
                }
            }
        }

        count
    }

    // Solve part two
    fn part2(&self) -> i64 {
        // The idea is to find the distress beacon which is the only point not covered by any sensor.
        // The beacon must be at distance `d + 1` from at least one sensor (where `d` is the sensor's range).
        // For each sensor, the boundary at distance `d + 1` consists of 4 line segments:
        //   y = x + a (positive slope)
        //   y = -x + b (negative slope)
        // Since the beacon is unique, it's likely (or guaranteed) to be at the intersection of such lines from different sensors.
        // Or at least, the intersection points are very few candidates compared to scanning the whole grid.

        let mut acoeffs = Vec::with_capacity(self.sensors.len() * 2); // coefficients for y = x + a
        let mut bcoeffs = Vec::with_capacity(self.sensors.len() * 2); // coefficients for y = -x + b

        for (sx, sy, d) in &self.sensors {
            // y = x + (sy - sx) +/- (d + 1)
            acoeffs.push(sy - sx - d - 1);
            acoeffs.push(sy - sx + d + 1);

            // y = -x + (sy + sx) +/- (d + 1)
            bcoeffs.push(sy + sx - d - 1);
            bcoeffs.push(sy + sx + d + 1);
        }

        acoeffs.sort_unstable();
        bcoeffs.sort_unstable();
        acoeffs.dedup();
        bcoeffs.dedup();

        for &a in &acoeffs {
            for &b in &bcoeffs {
                // Intersection of y = x + a and y = -x + b
                // x + a = -x + b => 2x = b - a
                // 2y = a + b

                let p = b - a;
                let q = a + b;

                // Coordinates must be integers
                if p % 2 != 0 || q % 2 != 0 {
                    continue;
                }

                let x = p / 2;
                let y = q / 2;

                // Check bounds [0, field_size]
                if x < 0 || x > self.field_size || y < 0 || y > self.field_size {
                    continue;
                }

                // Check if this point is covered by ANY sensor
                // If not covered, we found the distress beacon!
                let mut covered = false;
                for (sx, sy, d) in &self.sensors {
                    if manhattan(x, y, *sx, *sy) <= *d {
                        covered = true;
                        break;
                    }
                }

                if !covered {
                    return x * 4_000_000 + y;
                }
            }
        }

        0
    }
}

#[must_use]
pub fn solve(data: &str) -> (i64, i64) {
    let puzzle = Puzzle::new(data, false);
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
        let puzzle = Puzzle::new(TEST_INPUT, true);
        assert_eq!(puzzle.part1(), 26);
        assert_eq!(puzzle.part2(), 56_000_011);
    }
}
