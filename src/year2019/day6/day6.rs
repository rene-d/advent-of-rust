//! [Day 6: Universal Orbit Map](https://adventofcode.com/2019/day/6)

use rustc_hash::FxHashMap;

struct Puzzle {
    orbits: FxHashMap<String, String>,
}

impl Puzzle {
    fn new(data: &str) -> Self {
        let mut orbits = FxHashMap::default();

        for line in data.lines() {
            if let Some((a, b)) = line.split_once(')') {
                orbits.insert(b.to_string(), a.to_string());
            }
        }

        Self { orbits }
    }

    /// Solve part one.
    fn part1(&self) -> u32 {
        let mut total = 0;

        for orbit in self.orbits.keys() {
            let mut orbit = orbit.as_str();

            while orbit != "COM" {
                orbit = &self.orbits[orbit];
                total += 1;
            }
        }

        total
    }

    /// Solve part two.
    fn part2(&self) -> usize {
        let mut you_orbits = Vec::new();
        let mut san_orbits = Vec::new();

        let mut orbit = "YOU";
        while orbit != "COM" {
            orbit = &self.orbits[orbit];
            you_orbits.push(orbit);
        }

        orbit = "SAN";
        while orbit != "COM" {
            orbit = &self.orbits[orbit];
            san_orbits.push(orbit);
        }

        let mut common = 0;
        while you_orbits[you_orbits.len() - 1 - common] == san_orbits[san_orbits.len() - 1 - common]
        {
            common += 1;
        }

        you_orbits.len() + san_orbits.len() - common * 2
    }
}

/// # Panics
#[must_use]
pub fn solve(data: &str) -> (u32, usize) {
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

    const SAMPLE_2: &str = include_str!("sample_2.txt");
    const SAMPLE_4: &str = include_str!("sample_4.txt");

    #[test]
    fn test01() {
        let puzzle = Puzzle::new(SAMPLE_2);
        assert_eq!(puzzle.part1(), 42);
    }

    #[test]
    fn test02() {
        let puzzle = Puzzle::new(SAMPLE_4);
        assert_eq!(puzzle.part2(), 4);
    }
}
