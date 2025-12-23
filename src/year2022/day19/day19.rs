//! [Day 19: Not Enough Minerals](https://adventofcode.com/2022/day/19)

use rayon::prelude::*;
use regex::Regex;

enum RobotType {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct State {
    ore: u16,               // resources
    clay: u16,              // .
    obsidian: u16,          // .
    geode: u16,             // .
    ore_robots: u16,        // robots
    clay_robots: u16,       // .
    obsidian_robots: u16,   // .
    geode_robots: u16,      // .
    minutes_remaining: u16, // remaining time
}

impl State {
    fn time_to_build(
        &self,
        cost_ore: u16,
        cost_clay: u16,
        cost_obsidian: u16,
        robot_type: &RobotType,
    ) -> (u16, Option<Self>) {
        let mut wait_time = 0;

        // Ore
        if self.ore < cost_ore {
            if self.ore_robots == 0 {
                return (0, None);
            }
            let needed = cost_ore - self.ore;
            let time = needed.div_ceil(self.ore_robots);
            wait_time = wait_time.max(time);
        }

        // Clay
        if cost_clay > 0 && self.clay < cost_clay {
            if self.clay_robots == 0 {
                return (0, None);
            }
            let needed = cost_clay - self.clay;
            let time = needed.div_ceil(self.clay_robots);
            wait_time = wait_time.max(time);
        }

        // Obsidian
        if cost_obsidian > 0 && self.obsidian < cost_obsidian {
            if self.obsidian_robots == 0 {
                return (0, None);
            }
            let needed = cost_obsidian - self.obsidian;
            let time = needed.div_ceil(self.obsidian_robots);
            wait_time = wait_time.max(time);
        }

        // +1 minute to build
        let total_time_spent = wait_time + 1;

        if total_time_spent > self.minutes_remaining {
            return (0, None);
        }

        let mut new_state = *self;
        new_state.minutes_remaining -= total_time_spent;
        new_state.ore = self.ore + self.ore_robots * total_time_spent - cost_ore;
        new_state.clay = self.clay + self.clay_robots * total_time_spent - cost_clay;
        new_state.obsidian =
            self.obsidian + self.obsidian_robots * total_time_spent - cost_obsidian;
        new_state.geode = self.geode + self.geode_robots * total_time_spent;

        match robot_type {
            RobotType::Ore => new_state.ore_robots += 1,
            RobotType::Clay => new_state.clay_robots += 1,
            RobotType::Obsidian => new_state.obsidian_robots += 1,
            RobotType::Geode => new_state.geode_robots += 1,
        }

        (wait_time, Some(new_state))
    }
}

// Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.

struct Blueprint {
    id: u32,                     // Blueprint ID
    ore_cost_in_ore: u16,        // Each ore robot costs 4 ore.
    clay_cost_in_ore: u16,       // Each clay robot costs 2 ore.
    obsidian_cost_in_ore: u16,   // Each obsidian robot costs 3 ore
    obsidian_cost_in_clay: u16,  // ... and 14 clay.
    geode_cost_in_ore: u16,      // Each geode robot costs 2 ore
    geode_cost_in_obsidian: u16, // ... and 7 obsidian.
    max_ore: u16,
    max_clay: u16,
    max_obsidian: u16,
}

impl Blueprint {
    fn new(input: &str) -> Self {
        static RE: std::sync::OnceLock<Regex> = std::sync::OnceLock::new();
        let re = RE.get_or_init(|| Regex::new(r"Blueprint (\d+): Each ore robot costs (\d+) ore. Each clay robot costs (\d+) ore. Each obsidian robot costs (\d+) ore and (\d+) clay. Each geode robot costs (\d+) ore and (\d+) obsidian.").unwrap());
        let cap = re.captures(input).unwrap();

        let ore_cost_in_ore = cap[2].parse::<u16>().unwrap();
        let clay_cost_in_ore = cap[3].parse::<u16>().unwrap();
        let obsidian_cost_in_ore = cap[4].parse::<u16>().unwrap();
        let obsidian_cost_in_clay = cap[5].parse::<u16>().unwrap();
        let geode_cost_in_ore = cap[6].parse::<u16>().unwrap();
        let geode_cost_in_obsidian = cap[7].parse::<u16>().unwrap();

        let max_ore = ore_cost_in_ore
            .max(clay_cost_in_ore)
            .max(obsidian_cost_in_ore)
            .max(geode_cost_in_ore);

        Self {
            id: cap[1].parse().unwrap(),
            ore_cost_in_ore,
            clay_cost_in_ore,
            obsidian_cost_in_ore,
            obsidian_cost_in_clay,
            geode_cost_in_ore,
            geode_cost_in_obsidian,
            max_ore,
            max_clay: obsidian_cost_in_clay,
            max_obsidian: geode_cost_in_obsidian,
        }
    }

    fn solve(&self, time: u16) -> u32 {
        let initial_state = State {
            ore: 0,
            clay: 0,
            obsidian: 0,
            geode: 0,
            ore_robots: 1, // we start with 1 ore robot
            clay_robots: 0,
            obsidian_robots: 0,
            geode_robots: 0,
            minutes_remaining: time,
        };

        let mut max_geodes = 0;
        self.dfs(initial_state, &mut max_geodes);
        u32::from(max_geodes)
    }

    fn dfs(&self, state: State, max_geodes: &mut u16) {
        // Pruning: Upper bound check
        let remaining_time = state.minutes_remaining;
        if remaining_time == 0 {
            *max_geodes = (*max_geodes).max(state.geode);
            return;
        }

        let upper_bound = state.geode
            + state.geode_robots * remaining_time
            + (remaining_time * (remaining_time - 1)) / 2;

        if upper_bound <= *max_geodes {
            return;
        }

        // Try to build Geode Robot
        if state.ore_robots > 0 && state.obsidian_robots > 0 {
            let (_wait_time, new_state) = state.time_to_build(
                self.geode_cost_in_ore,
                0,
                self.geode_cost_in_obsidian, // obsidian is the second resource
                &RobotType::Geode,
            );
            if let Some(ns) = new_state {
                self.dfs(ns, max_geodes);
            }
        }

        // Try to build Obsidian Robot
        if state.obsidian_robots < self.max_obsidian
            && state.ore_robots > 0
            && state.clay_robots > 0
        {
            let (_, new_state) = state.time_to_build(
                self.obsidian_cost_in_ore,
                self.obsidian_cost_in_clay, // clay
                0,
                &RobotType::Obsidian,
            );
            if let Some(ns) = new_state {
                self.dfs(ns, max_geodes);
            }
        }

        // Try to build Clay Robot
        if state.clay_robots < self.max_clay && state.ore_robots > 0 {
            let (_, new_state) = state.time_to_build(self.clay_cost_in_ore, 0, 0, &RobotType::Clay);
            if let Some(ns) = new_state {
                self.dfs(ns, max_geodes);
            }
        }

        // Try to build Ore Robot
        if state.ore_robots < self.max_ore && state.ore_robots > 0 {
            let (_, new_state) = state.time_to_build(self.ore_cost_in_ore, 0, 0, &RobotType::Ore);
            if let Some(ns) = new_state {
                self.dfs(ns, max_geodes);
            }
        }

        let geodes_at_end = state.geode + state.geode_robots * state.minutes_remaining;
        *max_geodes = (*max_geodes).max(geodes_at_end);
    }
}

struct Puzzle {
    blueprints: Vec<Blueprint>,
}

impl Puzzle {
    fn new(data: &str) -> Self {
        Self {
            blueprints: data.lines().map(Blueprint::new).collect(),
        }
    }

    fn part1(&self) -> u32 {
        self.blueprints
            .par_iter()
            .map(|blueprint| blueprint.solve(24) * blueprint.id)
            .sum()
    }

    fn part2(&self) -> u32 {
        self.blueprints
            .iter()
            .take(3)
            .collect::<Vec<_>>()
            .par_iter()
            .map(|blueprint| blueprint.solve(32))
            .product()
    }
}

#[must_use]
pub fn solve(data: &str) -> (u32, u32) {
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

    const TEST_INPUT: &str = include_str!("test.txt");

    #[test]
    fn test_part1() {
        let puzzle = Puzzle::new(TEST_INPUT);
        assert_eq!(puzzle.part1(), 33);
    }

    #[test]
    fn test_part2() {
        let puzzle = Puzzle::new(TEST_INPUT);
        assert_eq!(puzzle.blueprints[0].solve(32), 56);
        assert_eq!(puzzle.blueprints[1].solve(32), 62);
    }
}
