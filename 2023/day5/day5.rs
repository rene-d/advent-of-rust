//! [Day 5: If You Give A Seed A Fertilizer](https://adventofcode.com/2023/day/5)

use rayon::prelude::*;
use std::time::{Duration, Instant};

fn conv_step(name: &str) -> usize {
    match name {
        "seed-to-soil" => 0,
        "soil-to-fertilizer" => 1,
        "fertilizer-to-water" => 2,
        "water-to-light" => 3,
        "light-to-temperature" => 4,
        "temperature-to-humidity" => 5,
        "humidity-to-location" => 6,
        _ => panic!("bad name: {name}"),
    }
}

struct Conv {
    destination: u64,
    source: u64,
    end: u64, // actually source+count
}

struct Puzzle {
    seeds: Vec<u64>,
    maps: Vec<Vec<Conv>>,
}

impl Puzzle {
    const fn new() -> Self {
        Self {
            seeds: vec![],
            maps: vec![],
        }
    }

    /// Get the puzzle input.
    fn configure(&mut self, path: &str) {
        let data = std::fs::read_to_string(path).unwrap();
        let mut current_map = 0;

        self.maps.resize_with(7, Vec::new);

        for line in data.lines() {
            if let Some(seeds) = line.strip_prefix("seeds:") {
                self.seeds = seeds
                    .split_whitespace()
                    .map(|x| x.parse::<u64>().unwrap())
                    .collect();
            } else if let Some(map) = line.strip_suffix(" map:") {
                current_map = conv_step(map);
            } else if !line.is_empty() {
                let dsc: Vec<u64> = line
                    .split_ascii_whitespace()
                    .map(|x| x.parse::<u64>().unwrap())
                    .collect();
                assert!(dsc.len() == 3, "bad line {line}");
                let conv = Conv {
                    destination: dsc[0],
                    source: dsc[1],
                    end: dsc[1] + dsc[2],
                };
                self.maps[current_map].push(conv);
            }
        }
    }

    fn convert(&self, map: usize, seed: u64) -> u64 {
        let map = self.maps.get(map).unwrap();

        for conv in map {
            if conv.source <= seed && seed < conv.end {
                return seed + conv.destination - conv.source;
            }
        }

        seed
    }

    fn grow(&self, seed: u64) -> u64 {
        let mut seed = seed;

        for step in 0..7 {
            seed = self.convert(step, seed);
        }
        seed
    }

    /// Solve part one.
    fn part1(&self) -> u64 {
        self.seeds
            .iter()
            .map(|&seed| self.grow(seed))
            .min()
            .unwrap()
    }

    /// Solve part two.
    fn part2(&self) -> u64 {
        let start: Instant = Instant::now();

        let result = self
            .seeds
            .chunks(2)
            .par_bridge() // <-- just added that! 😎
            .filter_map(|chunk| {
                let start = chunk[0];
                let end = chunk[0] + chunk[1];
                (start..end).map(|seed| self.grow(seed)).min()
            })
            .min()
            .unwrap_or(0);

        let iterations: u64 = self.seeds.iter().skip(1).step_by(2).sum();

        let duration: Duration = start.elapsed();

        eprintln!("Time elapsed (part2): {duration:?} for {iterations} iterations");

        result
    }
}

fn main() {
    let args = aoc::parse_args();
    let mut puzzle = Puzzle::new();
    puzzle.configure(args.path.as_str());
    println!("{}", puzzle.part1());
    println!("{}", puzzle.part2());
}

/// Test from puzzle input
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test01() {
        let mut puzzle = Puzzle::new();
        puzzle.configure("test.txt");
        assert_eq!(puzzle.part1(), 35);
    }

    #[test]
    fn test02() {
        let mut puzzle = Puzzle::new();
        puzzle.configure("test.txt");
        assert_eq!(puzzle.part2(), 46);
    }
}
