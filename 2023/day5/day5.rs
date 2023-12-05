//! [Day 5: If You Give A Seed A Fertilizer](https://adventofcode.com/2023/day/5)

use std::time::{Duration, Instant};

use clap::Parser;

#[derive(Parser)]
struct Args {
    /// Puzzle input
    #[arg(default_value = "input.txt")]
    path: String,
}

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
    fn new() -> Puzzle {
        Puzzle {
            seeds: vec![],
            maps: vec![],
        }
    }

    /// Get the puzzle input.
    fn configure(&mut self, path: &str) {
        let data = std::fs::read_to_string(path).unwrap();
        let mut current_map = 0;

        self.maps.resize_with(7, || vec![]);

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
                if dsc.len() != 3 {
                    panic!("bad line {line}");
                }
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
            .map(|seed| self.grow(*seed))
            .min()
            .unwrap()
    }

    /// Solve part two.
    fn part2(&self) -> u64 {
        let start: Instant = Instant::now();

        let mut result = u64::MAX;
        let mut iterations = 0u64;

        for r in self.seeds.windows(2).step_by(2) {
            let start = r[0];
            let end = r[0] + r[1];
            for seed in start..end {
                result = result.min(self.grow(seed));
            }
            iterations += end - start;
        }

        // for _ in 0..1_000_000 {
        //     result = result.min(self.grow(self.seeds[0]));
        // }

        let duration: Duration = start.elapsed();

        eprintln!("Time elapsed (part2): {duration:?} for {iterations} iterations");

        result
    }
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

fn main() {
    let args = Args::parse();
    let mut puzzle = Puzzle::new();
    puzzle.configure(args.path.as_str());
    println!("{}", puzzle.part1());
    println!("{}", puzzle.part2());
}
