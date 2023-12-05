//! [Day 5: If You Give A Seed A Fertilizer](https://adventofcode.com/2023/day/5)

use std::collections::HashMap;
use std::time::{Duration, Instant};

use clap::Parser;

#[derive(Eq, Hash, PartialEq, Clone)]
enum ConvType {
    Unknown,
    SeedToSoil,
    SoilToFertilizer,
    FertilizerToWater,
    WaterTolight,
    LightTotemperature,
    TemperatureTohumidity,
    HumidityTolocation,
}

impl ConvType {
    fn new(name: &str) -> Self {
        match name {
            "seed-to-soil" => Self::SeedToSoil,
            "soil-to-fertilizer" => Self::SoilToFertilizer,
            "fertilizer-to-water" => Self::FertilizerToWater,
            "water-to-light" => Self::WaterTolight,
            "light-to-temperature" => Self::LightTotemperature,
            "temperature-to-humidity" => Self::TemperatureTohumidity,
            "humidity-to-location" => Self::HumidityTolocation,
            _ => panic!("bad name: {name}"),
        }
    }
}

#[derive(Parser)]
struct Args {
    /// Puzzle input
    #[arg(default_value = "input.txt")]
    path: String,
}

struct Conv {
    destination: u64,
    source: u64,
    end: u64, // actually source+count
}

struct Puzzle {
    seeds: Vec<u64>,
    maps: HashMap<ConvType, Vec<Conv>>,
}

impl Puzzle {
    fn new() -> Puzzle {
        Puzzle {
            seeds: vec![],
            maps: HashMap::new(),
        }
    }

    /// Get the puzzle input.
    fn configure(&mut self, path: &str) {
        let data = std::fs::read_to_string(path).unwrap();
        let mut current_map = ConvType::Unknown;
        for line in data.lines() {
            if let Some(seeds) = line.strip_prefix("seeds:") {
                self.seeds = seeds
                    .split_whitespace()
                    .map(|x| x.parse::<u64>().unwrap())
                    .collect();
            } else if let Some(map) = line.strip_suffix(" map:") {
                current_map = ConvType::new(map);
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
                let current_map = current_map.clone();
                self.maps.entry(current_map).or_default().push(conv);
            }
        }
    }

    fn convert(&self, map: ConvType, seed: u64) -> u64 {
        let map = self.maps.get(&map).unwrap();

        for conv in map {
            if conv.source <= seed && seed < conv.end {
                return seed + conv.destination - conv.source;
            }
        }

        seed
    }

    fn grow(&self, seed: u64) -> u64 {
        let mut seed = seed;
        seed = self.convert(ConvType::SeedToSoil, seed);
        seed = self.convert(ConvType::SoilToFertilizer, seed);
        seed = self.convert(ConvType::FertilizerToWater, seed);
        seed = self.convert(ConvType::WaterTolight, seed);
        seed = self.convert(ConvType::LightTotemperature, seed);
        seed = self.convert(ConvType::TemperatureTohumidity, seed);
        seed = self.convert(ConvType::HumidityTolocation, seed);
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
