//! [Day 22: Reactor Reboot](https://adventofcode.com/2021/day/22)

use regex::Regex;
use rustc_hash::FxHashSet;
use std::cmp::max;
use std::cmp::min;

struct Cube {
    x1: i64,
    x2: i64,
    y1: i64,
    y2: i64,
    z1: i64,
    z2: i64,
    holes: Vec<Cube>,
}

impl Cube {
    fn new(line: &str) -> Self {
        let re = Regex::new(r"^x=(-?\d+)\.\.(-?\d+),y=(-?\d+)\.\.(-?\d+),z=(-?\d+)\.\.(-?\d+)$")
            .unwrap();

        let coords = re.captures(line).unwrap();

        Self {
            x1: coords[1].parse().unwrap(),
            x2: coords[2].parse().unwrap(),
            y1: coords[3].parse().unwrap(),
            y2: coords[4].parse().unwrap(),
            z1: coords[5].parse().unwrap(),
            z2: coords[6].parse().unwrap(),

            holes: vec![],
        }
    }

    fn volume(&self) -> i64 {
        let vol = (self.x2 - self.x1 + 1) * (self.y2 - self.y1 + 1) * (self.z2 - self.z1 + 1);

        vol - self.holes.iter().map(Cube::volume).sum::<i64>()
    }

    fn intersect(&self, other: &Self) -> Option<Self> {
        let result = Self {
            x1: self.x1.max(other.x1),
            x2: self.x2.min(other.x2),
            y1: self.y1.max(other.y1),
            y2: self.y2.min(other.y2),
            z1: self.z1.max(other.z1),
            z2: self.z2.min(other.z2),
            holes: vec![],
        };

        if result.x2 >= result.x1 && result.y2 >= result.y1 && result.z2 >= result.z1 {
            Some(result)
        } else {
            None
        }
    }

    fn substract(&mut self, other: &Self) {
        if let Some(isect) = self.intersect(other) {
            for hole in &mut self.holes {
                hole.substract(other);
            }
            self.holes.push(isect);
        }
    }
}

/// main function
fn main() {
    let args = aoc::parse_args();
    let data = args
        .input
        .lines()
        .map(std::string::ToString::to_string)
        .collect::<Vec<String>>();

    println!("{}", part1(&data));
    println!("{}", part2(&data));
}

fn part2(data: &[String]) -> i64 {
    let mut cubes: Vec<Cube> = vec![];

    for line in data {
        let (flag_on, coords) = line.split_once(' ').unwrap();

        let cube = Cube::new(coords);

        for other in &mut cubes {
            other.substract(&cube);
        }

        if flag_on == "on" {
            cubes.push(cube);
        }
    }

    cubes.iter().map(Cube::volume).sum()
}

fn part1(data: &[String]) -> usize {
    let re =
        Regex::new(r"^(on|off) x=(-?\d+)\.\.(-?\d+),y=(-?\d+)\.\.(-?\d+),z=(-?\d+)\.\.(-?\d+)$")
            .unwrap();

    let mut cubes_on: FxHashSet<(i32, i32, i32)> = FxHashSet::default();

    for line in data {
        let cube = re.captures(line).unwrap();

        let flag_on = cube[1].to_string() == "on";

        let x1 = cube[2].parse::<i32>().unwrap();
        let x2 = cube[3].parse::<i32>().unwrap();
        let y1 = cube[4].parse::<i32>().unwrap();
        let y2 = cube[5].parse::<i32>().unwrap();
        let z1 = cube[6].parse::<i32>().unwrap();
        let z2 = cube[7].parse::<i32>().unwrap();

        for x in max(-50, x1)..=min(50, x2) {
            for y in max(-50, y1)..=min(50, y2) {
                for z in max(-50, z1)..=min(50, z2) {
                    if flag_on {
                        cubes_on.insert((x, y, z));
                    } else {
                        cubes_on.remove(&(x, y, z));
                    }
                }
            }
        }
    }

    cubes_on.len()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        let data = std::fs::read_to_string("sample_1.txt").unwrap();
        let data: Vec<_> = data.lines().map(String::from).collect();

        assert_eq!(part1(&data), 39);
    }

    #[test]
    fn test2() {
        let data = std::fs::read_to_string("sample_2.txt").unwrap();
        let data: Vec<_> = data.lines().map(String::from).collect();

        assert_eq!(part1(&data), 590784);
    }

    #[test]
    fn test3() {
        let data = std::fs::read_to_string("sample_3.txt").unwrap();
        let data: Vec<_> = data.lines().map(String::from).collect();

        assert_eq!(part2(&data), 2758514936282235);
        assert_eq!(part1(&data), 474140);
    }
}
