//! [Day 15: Beacon Exclusion Zone](https://adventofcode.com/2022/day/15)

use regex::Regex;
use rustc_hash::FxHashSet;

/// Computes the [Manhattan distance](https://en.wikipedia.org/wiki/Taxicab_geometry) between two points
const fn manhattan(ax: i64, ay: i64, bx: i64, by: i64) -> i64 {
    (ax - bx).abs() + (ay - by).abs()
}

struct Puzzle {
    sensors: Vec<(i64, i64, i64)>, // list of (x,y,distance from nearest beacon)
    beacons: FxHashSet<(i64, i64)>, // set of beacons
    max_d: i64,                    // max distance sensor-beacon
    field_size: i64,               // 20 or 4000000 depends on test or puzzle
}

impl Puzzle {
    fn new(data: &str, is_test: bool) -> Self {
        let field_size = if is_test { 20 } else { 4_000_000 };

        let re = Regex::new(
            r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)",
        )
        .unwrap();

        let lines = data.split('\n').collect::<Vec<_>>();

        let mut sensors = Vec::new();
        let mut beacons = FxHashSet::default();
        let mut max_d = 0;

        for line in lines {
            if let Some(m) = re.captures(line) {
                let sx = m[1].parse::<i64>().unwrap();
                let sy = m[2].parse::<i64>().unwrap();
                let bx = m[3].parse::<i64>().unwrap();
                let by = m[4].parse::<i64>().unwrap();

                let d = manhattan(sx, sy, bx, by);

                if d > max_d {
                    max_d = d;
                }

                sensors.push((sx, sy, d));
                beacons.insert((bx, by));
            }
        }

        Self {
            sensors,
            beacons,
            max_d,
            field_size,
        }
    }

    // Solves part one
    fn part1(&self) -> u32 {
        let bx_min = self.beacons.iter().map(|x| x.0).min().unwrap() - self.max_d;
        let bx_max = self.beacons.iter().map(|x| x.0).max().unwrap() + self.max_d;

        let y = self.field_size / 2;

        let mut part1 = 0;

        for x in bx_min..=bx_max {
            if self.beacons.contains(&(x, y)) {
                continue;
            }

            for (sx, sy, nearest_beacon) in &self.sensors {
                let d = manhattan(x, y, *sx, *sy);
                if d <= *nearest_beacon {
                    // the sensors always report the nearest beacon
                    // if the distance is less than the distance measured by the sensor,
                    // there cannot be a beacon at this position

                    part1 += 1;
                    break;
                }
            }
        }
        part1
    }

    // Solve part two
    fn part2(&self) -> i64 {
        for y in 0..=self.field_size {
            // each sensor defines a zone where there is only one beacon
            // this zone is all points at a distance less than or equal to the Manhattan distance to its beacon
            // (i.e. a disk for this distance, not the Euclidian one)

            // computes the intersection of the blank zone of each sensor and the row y => a 'segment'
            // example: intersection with the blank zone of 'radius' 3 of sensor S with column 3
            //          the segment with three X
            //   01234567
            //   ........
            //   ....#...
            //   ...X##..
            //   ..#XS##.
            //   ...X##,.
            //   ....#...
            //
            let mut segments = vec![];
            for (sx, sy, sd) in &self.sensors {
                let delta = sd - (sy - y).abs();
                if delta >= 0 {
                    segments.push((*sx - delta, *sx + delta + 1));
                }
            }

            // the union of all intersecions: it should overlap the entire row [0, 4000000]
            // except for only one row: a point should be not covered and this is the solution
            // in this case, the intersection is two disjointed segments
            segments.sort_by_key(|a| a.0);

            let mut column: Vec<(i64, i64)> = vec![];
            let mut it = segments.iter();

            column.push(*it.next().unwrap());
            for curr in it {
                let tail = column.last_mut().unwrap();
                if tail.1 < curr.0 {
                    column.push(*curr);
                } else if tail.1 < curr.1 {
                    *tail = (tail.0, curr.1);
                }
            }

            // we eventually can verify that
            //  - column[0].0 <= 0 && column[-1].1 >= self.field_size
            //  - column.len() == 1 or 2
            //  - if 2, column[0].1 + 1 == column[1].0

            if column.len() > 1 {
                let x = column.first().unwrap().1;
                return x * 4_000_000 + y;
            }
        }

        0
    }
}

#[must_use]
pub fn solve(data: &str) -> (u32, i64) {
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
