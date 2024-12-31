//! [Day 5: If You Give A Seed A Fertilizer](https://adventofcode.com/2023/day/5)

use std::mem::swap;

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
    destination: u64, // destination range start
    start: u64,       // source range start
    end: u64,         // start + range length
}

struct Puzzle {
    seeds: Vec<u64>,
    maps: Vec<Vec<Conv>>,
}

impl Puzzle {
    const fn new() -> Self {
        Self {
            seeds: Vec::new(),
            maps: Vec::new(),
        }
    }

    /// Get the puzzle input.
    fn configure(&mut self, data: &str) {
        let mut current_map_idx = 0;

        self.maps.resize_with(7, Vec::new);

        for line in data.lines() {
            if let Some(seeds) = line.strip_prefix("seeds:") {
                self.seeds = seeds
                    .split_whitespace()
                    .map(|x| x.parse::<u64>().unwrap())
                    .collect();
            } else if let Some(map) = line.strip_suffix(" map:") {
                current_map_idx = conv_step(map);
            } else if !line.is_empty() {
                let dsc: Vec<u64> = line
                    .split_ascii_whitespace()
                    .map(|x| x.parse::<u64>().unwrap())
                    .collect();
                assert!(dsc.len() == 3, "bad line {line}");
                let conv = Conv {
                    destination: dsc[0],
                    start: dsc[1],
                    end: dsc[1] + dsc[2],
                };
                self.maps[current_map_idx].push(conv);
            }
        }
    }

    fn convert(&self, map: usize, seed: u64) -> u64 {
        // functional approach:
        self.maps[map]
            .iter()
            .find(|conv| conv.start <= seed && seed < conv.end)
            .map_or(seed, |conv| seed + conv.destination - conv.start)

        // imperative equivalent:
        // let map = self.maps.get(map).unwrap();
        // for conv in map {
        //     if conv.start <= seed && seed < conv.end {
        //         return seed + conv.destination - conv.start;
        //     }
        // }
        // seed
    }

    fn grow(&self, seed: u64) -> u64 {
        // functional approach:
        (0..7).fold(seed, |seed, step| self.convert(step, seed))

        // imperative equivalent:
        // let mut seed = seed;
        // for step in 0..7 {
        //     seed = self.convert(step, seed);
        // }
        // seed
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
    pub fn part2(&self) -> u64 {
        let current = &mut Vec::new();
        let next = &mut Vec::new();
        let mut next_map = Vec::new();

        // convert pairs to ranges of seeds
        for pair in self.seeds.chunks(2) {
            current.push((pair[0], pair[0] + pair[1]));
        }

        for map in &self.maps {
            for conv in map {
                while let Some((start, end)) = current.pop() {
                    // find intersection of both segments [start; end] and [conv.start; conv.end]
                    let a = start.max(conv.start);
                    let b = end.min(conv.end);

                    if a < b {
                        // overlap (4 arrangements)

                        // -------[start conv end[---------
                        // ------------[start current end[-----
                        //             ^         ^       ^
                        //             a         b       |
                        //                       [remnant[

                        // convert the seed range and send it to the next map
                        next_map.push((
                            a + conv.destination - conv.start,
                            b + conv.destination - conv.start,
                        ));

                        // the left remnant of the intersection, if any
                        if start < a {
                            next.push((start, a));
                        }

                        // the right remnant of the intersection, if any
                        if b < end {
                            next.push((b, end));
                        }
                    } else {
                        // no overlap (2 arrangements)

                        // ---[start conv end[---------------------------
                        // -----------------------[start current end[----
                        //                   ^    ^
                        //                   b    a

                        next.push((start, end));
                    }
                }

                swap(current, next);
            }

            current.append(&mut next_map);
        }

        current.iter().map(|r| r.0).min().unwrap()
    }
}

fn main() {
    let args = aoc::parse_args();
    let mut puzzle = Puzzle::new();
    puzzle.configure(&args.input);
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
        puzzle.configure(&aoc::load_input_data("test.txt"));
        assert_eq!(puzzle.part1(), 35);
    }

    #[test]
    fn test02() {
        let mut puzzle = Puzzle::new();
        puzzle.configure(&aoc::load_input_data("test.txt"));
        assert_eq!(puzzle.part2(), 46);
    }
}
