//! [Day 19: Not Enough Minerals](https://adventofcode.com/2022/day/19)

use clap::Parser;
use regex::Regex;
use std::collections::HashSet;

#[derive(Parser)]
struct Args {
    /// Puzzle input
    #[arg(default_value = "input.txt")]
    path: String,
}

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
struct State {
    ore: u32,               // ressources
    clay: u32,              //
    obsidian: u32,          //
    geode: u32,             //
    ore_robots: u32,        // robots
    clay_robots: u32,       //
    obsidian_robots: u32,   //
    geode_robots: u32,      //
    minutes_remaining: u32, // remaining time
}

// Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.

struct Blueprint {
    id: u32,                     // Blueprint ID
    ore_cost_in_ore: u32,        // Each ore robot costs 4 ore.
    clay_cost_in_ore: u32,       // Each clay robot costs 2 ore.
    obsidian_cost_in_ore: u32,   // Each obsidian robot costs 3 ore and 14 clay.
    obsidian_cost_in_clay: u32,  //
    geode_cost_in_ore: u32,      // Each geode robot costs 2 ore and 7 obsidian.
    geode_cost_in_obsidian: u32, //
}

impl Blueprint {
    fn new(input: &str) -> Self {
        let re= Regex::new(r"Blueprint (\d+): Each ore robot costs (\d+) ore. Each clay robot costs (\d+) ore. Each obsidian robot costs (\d+) ore and (\d+) clay. Each geode robot costs (\d+) ore and (\d+) obsidian.").unwrap();

        let m = re.captures(input).unwrap();

        Self {
            id: m[1].parse::<u32>().unwrap(),
            ore_cost_in_ore: m[2].parse::<u32>().unwrap(),
            clay_cost_in_ore: m[3].parse::<u32>().unwrap(),
            obsidian_cost_in_ore: m[4].parse::<u32>().unwrap(),
            obsidian_cost_in_clay: m[5].parse::<u32>().unwrap(),
            geode_cost_in_ore: m[6].parse::<u32>().unwrap(),
            geode_cost_in_obsidian: m[7].parse::<u32>().unwrap(),
        }
    }

    fn solve(&self, time: u32) -> u32 {
        let mut max_geode = 0;

        let mut q = Vec::new();
        let mut seen = HashSet::new();

        let max_ore = *[
            self.ore_cost_in_ore,
            self.clay_cost_in_ore,
            self.obsidian_cost_in_ore,
            self.geode_cost_in_ore,
        ]
        .iter()
        .max()
        .unwrap();

        q.push(State {
            ore: 0,
            clay: 0,
            obsidian: 0,
            geode: 0,
            ore_robots: 1, // we start with 1 ore robot
            clay_robots: 0,
            obsidian_robots: 0,
            geode_robots: 0,
            minutes_remaining: time,
        });

        while let Some(e) = q.pop() {
            max_geode = max_geode.max(e.geode);

            if e.minutes_remaining == 0 {
                continue;
            }

            let mut e = e;

            //
            if e.ore_robots >= max_ore {
                e.ore_robots = max_ore;
            }

            if e.clay_robots >= self.obsidian_cost_in_clay {
                e.clay_robots = self.obsidian_cost_in_clay;
            }

            if e.obsidian_robots >= self.geode_cost_in_obsidian {
                e.obsidian_robots = self.geode_cost_in_obsidian;
            }

            //
            if e.ore >= e.minutes_remaining * max_ore - e.ore_robots * (e.minutes_remaining - 1) {
                e.ore = e.minutes_remaining * max_ore - e.ore_robots * (e.minutes_remaining - 1);
            }

            if e.clay
                >= e.minutes_remaining * self.obsidian_cost_in_clay
                    - e.clay_robots * (e.minutes_remaining - 1)
            {
                e.clay = e.minutes_remaining * self.obsidian_cost_in_clay
                    - e.clay_robots * (e.minutes_remaining - 1);
            }

            if e.obsidian
                >= e.minutes_remaining * self.geode_cost_in_obsidian
                    - e.obsidian_robots * (e.minutes_remaining - 1)
            {
                e.obsidian = e.minutes_remaining * self.geode_cost_in_obsidian
                    - e.obsidian_robots * (e.minutes_remaining - 1);
            }

            if seen.contains(&e) {
                continue;
            }
            seen.insert(e);

            // build nothing
            q.push(State {
                ore: e.ore + e.ore_robots,
                clay: e.clay + e.clay_robots,
                obsidian: e.obsidian + e.obsidian_robots,
                geode: e.geode + e.geode_robots,

                ore_robots: e.ore_robots,
                clay_robots: e.clay_robots,
                obsidian_robots: e.obsidian_robots,
                geode_robots: e.geode_robots,

                minutes_remaining: e.minutes_remaining - 1,
            });

            // build an ore robot
            if e.ore >= self.ore_cost_in_ore {
                q.push(State {
                    ore: e.ore - self.ore_cost_in_ore + e.ore_robots,
                    clay: e.clay + e.clay_robots,
                    obsidian: e.obsidian + e.obsidian_robots,
                    geode: e.geode + e.geode_robots,

                    ore_robots: e.ore_robots + 1,
                    clay_robots: e.clay_robots,
                    obsidian_robots: e.obsidian_robots,
                    geode_robots: e.geode_robots,

                    minutes_remaining: e.minutes_remaining - 1,
                });
            }

            // build a clay robot
            if e.ore >= self.clay_cost_in_ore {
                q.push(State {
                    ore: e.ore - self.clay_cost_in_ore + e.ore_robots,
                    clay: e.clay + e.clay_robots,
                    obsidian: e.obsidian + e.obsidian_robots,
                    geode: e.geode + e.geode_robots,
                    ore_robots: e.ore_robots,
                    clay_robots: e.clay_robots + 1,
                    obsidian_robots: e.obsidian_robots,
                    geode_robots: e.geode_robots,
                    minutes_remaining: e.minutes_remaining - 1,
                });
            }

            // build an obsidian robot
            if e.ore >= self.obsidian_cost_in_ore && e.clay >= self.obsidian_cost_in_clay {
                q.push(State {
                    ore: e.ore - self.obsidian_cost_in_ore + e.ore_robots,
                    clay: e.clay - self.obsidian_cost_in_clay + e.clay_robots,
                    obsidian: e.obsidian + e.obsidian_robots,
                    geode: e.geode + e.geode_robots,
                    ore_robots: e.ore_robots,
                    clay_robots: e.clay_robots,
                    obsidian_robots: e.obsidian_robots + 1,
                    geode_robots: e.geode_robots,
                    minutes_remaining: e.minutes_remaining - 1,
                });
            }

            // build a geode robot
            if e.ore >= self.geode_cost_in_ore && e.obsidian >= self.geode_cost_in_obsidian {
                q.push(State {
                    ore: e.ore - self.geode_cost_in_ore + e.ore_robots,
                    clay: e.clay + e.clay_robots,
                    obsidian: e.obsidian - self.geode_cost_in_obsidian + e.obsidian_robots,
                    geode: e.geode + e.geode_robots,
                    ore_robots: e.ore_robots,
                    clay_robots: e.clay_robots,
                    obsidian_robots: e.obsidian_robots,
                    geode_robots: e.geode_robots + 1,
                    minutes_remaining: e.minutes_remaining - 1,
                });
            }
        }

        max_geode
    }
}

struct Puzzle {
    blueprints: Vec<Blueprint>,
}

impl Puzzle {
    fn new() -> Self {
        Self { blueprints: vec![] }
    }

    /// Loads data from input (one line)
    fn configure(&mut self, path: &str) {
        let data = std::fs::read_to_string(path).unwrap();

        self.blueprints.extend(
            data.split('\n')
                .filter(|x| !x.is_empty())
                .map(Blueprint::new),
        );
    }

    // Solves part one
    fn part1(&self) -> u32 {
        self.blueprints
            .iter()
            .map(|blueprint| blueprint.solve(24) * blueprint.id)
            .sum()
    }

    // Solve part two
    fn part2(&self) -> u32 {
        self.blueprints[0..3.min(self.blueprints.len())]
            .iter()
            .map(|blueprint| blueprint.solve(32))
            .product()
    }
}

/// main function
fn main() {
    let args = Args::parse();
    let mut puzzle = Puzzle::new();
    puzzle.configure(&args.path);
    println!("{}", puzzle.part1());
    println!("{}", puzzle.part2());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let mut puzzle = Puzzle::new();
        puzzle.configure("test.txt");
        assert_eq!(puzzle.part1(), 33);
    }

    // part 2 is too slow when non optimized
    #[cfg(not(debug_assertions))]
    #[test]
    fn test_part2() {
        let mut puzzle = Puzzle::new();
        puzzle.configure("test.txt");

        // assert_eq!(puzzle.blueprints[0].solve(32), 56);
        assert_eq!(puzzle.blueprints[1].solve(32), 62);
    }
}
