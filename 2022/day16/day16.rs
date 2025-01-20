//! [Day 16: Proboscidea Volcanium](https://adventofcode.com/2022/day/16)

#![allow(clippy::cast_possible_truncation)]

use regex::Regex;
use rustc_hash::FxHashMap;

struct Puzzle {
    valves: FxHashMap<String, u8>,
    flow_rates: FxHashMap<u8, u32>,
    tunnels: FxHashMap<u8, Vec<u8>>,
    distances: Box<[[u32; 128]]>,
}

impl Puzzle {
    fn new() -> Self {
        Self {
            valves: FxHashMap::default(),
            flow_rates: FxHashMap::default(),
            tunnels: FxHashMap::default(),
            distances: vec![[0u32; 128]; 128].into_boxed_slice(),
        }
    }

    /// Loads data from input (one line)
    fn configure(&mut self, data: &str) {
        let re = Regex::new(
            r"Valve (\w\w) has flow rate=(\d+); tunnels? leads? to valves? ((?:\w\w)(?:, \w\w)*)$",
        )
        .unwrap();

        for line in data.split('\n') {
            if let Some(m) = re.captures(line) {
                let valve = self.valve_id_new(&m[1]);

                let rate = m[2].parse::<u32>().unwrap();
                if rate != 0 {
                    self.flow_rates.insert(valve, rate);
                }

                let dest = m[3].split(", ").map(|x| self.valve_id_new(x)).collect();
                self.tunnels.insert(valve, dest);
            }
        }

        // since we use a u128 bitfield for list of valves, there can be no more than 64
        // and valve ID are between 0 and127 included
        assert!(self.valves.len() <= 128);

        // precompute distance between valves
        let max_id = self.valves.len();
        for a in 0..max_id {
            for b in 0..max_id {
                let d = self.calc_dist(a as u8, b as u8, 0);
                self.distances[a][b] = d;
            }
        }
    }

    const fn distance(&self, a: u8, b: u8) -> u32 {
        self.distances[a as usize][b as usize]
    }

    fn calc_dist(&self, a: u8, b: u8, visited: u128) -> u32 {
        let d = self.distance(a, b);
        if d != 0 {
            d
        } else if a == b {
            0
        } else if (1u128 << b) & visited != 0 {
            u32::MAX - 1
        } else {
            let b_visited = visited + (1u128 << b);

            self.tunnels[&b]
                .iter()
                .map(|x| self.calc_dist(a, *x, b_visited))
                .min()
                .unwrap()
                + 1
        }
    }

    /// Returns the valve ID with its name or create a new ID.
    fn valve_id_new(&mut self, name: &str) -> u8 {
        let next_id = self.valves.len() as u8;
        assert!(next_id <= 127);
        *self.valves.entry(name.to_string()).or_insert(next_id)
    }

    /// Returns the valve ID with its name. Valve must exist.
    fn valve_id(&self, name: &str) -> u8 {
        *self.valves.get(name).unwrap()
    }

    /// Returns the valve name with its ID. Used only in `show()`.
    fn valve_name(&self, valve_id: u8) -> &str {
        self.valves.iter().find(|x| *x.1 == valve_id).unwrap().0
    }

    /// Show the network of pipes (puzzle input), up the order to the valves.
    fn show(&self) {
        for (name, id) in &self.valves {
            let remotes = (self.tunnels[id]
                .iter()
                .map(|x| self.valve_name(*x))
                .collect::<Vec<&str>>())
            .join(", ");

            if self.tunnels[id].len() == 1 {
                println!(
                    "Valve {} has flow rate={}; tunnel leads to valve {}",
                    name,
                    self.flow_rates.get(id).unwrap_or(&0),
                    remotes
                );
            } else {
                println!(
                    "Valve {} has flow rate={}; tunnels lead to valves {}",
                    name,
                    self.flow_rates.get(id).unwrap_or(&0),
                    remotes
                );
            }
        }
    }

    /// Recursively search for the best flow rate.
    fn max_flow(
        &self,
        valve: u8,
        opened: u128,
        time_left: u32,
        seen: &mut FxHashMap<(u8, u128, u32), u32>,
    ) -> u32 {
        if let Some(e) = seen.get(&(valve, opened, time_left)) {
            return *e;
        }

        if time_left <= 1 {
            return 0;
        }

        let mut best = 0;

        if (opened & (1u128 << valve)) == 0 {
            if let Some(flow) = self.flow_rates.get(&valve) {
                best = (time_left - 1) * flow
                    + self.max_flow(valve, opened | (1u128 << valve), time_left - 1, seen);
            }
        }

        best = best.max(
            self.tunnels[&valve]
                .iter()
                .map(|x| self.max_flow(*x, opened, time_left - 1, seen))
                .max()
                .unwrap(),
        );

        seen.insert((valve, opened, time_left), best);

        best
    }

    // Solves part one
    fn part1(&self) -> u32 {
        let mut seen = FxHashMap::default();
        self.max_flow(self.valve_id("AA"), 0, 30, &mut seen)
    }

    // Solve part two
    fn part2(&self) -> u32 {
        let start_valve = self.valve_id("AA");

        let mut best = 0;

        // make two lists of distinct valves to open by me or the elephant
        // the lists are bitfields of valve ID actually
        // (note: me and elephant are interchangeable, hence the -1: there are twice less partitions)
        let partitions = 1u32 << (self.flow_rates.len() - 1);
        for partition in 0..partitions {
            // bit value 1 is for me
            let me: u128 = self
                .flow_rates
                .iter()
                .enumerate()
                .filter(|(bit, _)| (partition & (1 << bit) != 0))
                .map(|(_, (valve, _))| 1u128 << *valve)
                .sum();

            // bit value 0 is for the elehant
            let elephant: u128 = self
                .flow_rates
                .iter()
                .enumerate()
                .filter(|(bit, _)| (partition & (1 << bit) == 0))
                .map(|(_, (valve, _))| 1u128 << *valve)
                .sum();

            let best_me = self.max_flow_valves(start_valve, 26, me);
            let best_elephant = self.max_flow_valves(start_valve, 26, elephant);

            best = best.max(best_me + best_elephant);
        }
        best
    }

    fn max_flow_valves(&self, valve: u8, time_left: u32, nodes: u128) -> u32 {
        if time_left <= 1 {
            return 0;
        }

        let mut best = 0; // max flow for the set of nodes
        let mut node = 0u8; // dest valve ID
        let mut nodes_bitfield = nodes; // excessively complicated, I admit

        while nodes_bitfield != 0 {
            if nodes_bitfield & 1 == 1 {
                // i.e. nodes has the bit 'node' set

                let time_dist = self.distance(valve, node);
                assert_ne!(time_dist, 0);

                if time_left - 1 > time_dist {
                    let time = time_left - 1 - time_dist;

                    best = best.max(
                        time * self.flow_rates[&node]
                            + self.max_flow_valves(node, time, nodes & !(1u128 << node)),
                    );
                }
            }

            nodes_bitfield /= 2;
            node += 1;
        }
        best
    }
}

/// main function
fn main() {
    let args = aoc::parse_args();
    let mut puzzle = Puzzle::new();
    puzzle.configure(&args.input);

    if args.verbose {
        puzzle.show();
    }

    println!("{}", puzzle.part1());
    println!("{}", puzzle.part2());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let mut puzzle = Puzzle::new();
        puzzle.configure(&aoc::load_input_data("test.txt"));
        assert_eq!(puzzle.part1(), 1651);
    }

    #[test]
    fn test_part2() {
        let mut puzzle = Puzzle::new();
        puzzle.configure(&aoc::load_input_data("test.txt"));
        assert_eq!(puzzle.part2(), 1707);
    }
}
