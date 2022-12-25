//! [Day 16: Proboscidea Volcanium](https://adventofcode.com/2022/day/16)

use clap::Parser;
use regex::Regex;
use std::collections::HashMap;

#[derive(Parser)]
struct Args {
    /// Puzzle input
    #[arg(default_value = "input.txt")]
    path: String,

    #[arg(short, long)]
    verbose: bool,
}

struct Puzzle {
    valves: HashMap<String, u16>,
    flow_rates: HashMap<u16, u32>,
    tunnels: HashMap<u16, Vec<u16>>,
}

impl Puzzle {
    fn new() -> Self {
        Self {
            valves: HashMap::new(),
            flow_rates: HashMap::new(),
            tunnels: HashMap::new(),
        }
    }

    /// Loads data from input (one line)
    fn configure(&mut self, path: &str) {
        let data = std::fs::read_to_string(path).unwrap();

        let re = Regex::new(
            r"Valve (\w\w) has flow rate=(\d+); tunnels? leads? to valves? ((?:\w\w)(?:, \w\w)*)$",
        )
        .unwrap();

        for line in data.split('\n').collect::<Vec<_>>() {
            if let Some(m) = re.captures(line) {
                let valve = self.valve_id_new(&m[1]);

                self.flow_rates.insert(valve, m[2].parse::<u32>().unwrap());

                let dest = m[3].split(", ").map(|x| self.valve_id_new(x)).collect();
                self.tunnels.insert(valve, dest);
            }
        }
    }

    fn valve_id_new(&mut self, name: &str) -> u16 {
        let next_id = self.valves.len() as u16 + 1;
        *self.valves.entry(name.to_string()).or_insert(next_id)
    }

    fn valve_id(&self, name: &str) -> u16 {
        *self.valves.get(name).unwrap()
    }

    fn valve_name(&self, valve_id: u16) -> &str {
        for (name, id) in &self.valves {
            if valve_id == *id {
                return name;
            }
        }
        ""
    }

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
                    name, self.flow_rates[id], remotes
                );
            } else {
                println!(
                    "Valve {} has flow rate={}; tunnels lead to valves {}",
                    name, self.flow_rates[id], remotes
                );
            }
        }
    }

    fn max_flow(
        &self,
        current: u16,
        opened: u64,
        time_left: u32,
        seen: &mut HashMap<(u16, u64, u32), u32>,
    ) -> u32 {
        if let Some(e) = seen.get(&(current, opened, time_left)) {
            return *e;
        }

        if time_left <= 1 {
            return 0;
        }

        let mut best = 0;
        let flow = self.flow_rates[&current];

        if (opened & (1u64 << current)) == 0 && flow != 0 {
            best = (time_left - 1) * flow
                + self.max_flow(current, opened | (1u64 << current), time_left - 1, seen);
        }

        best = best.max(
            self.tunnels[&current]
                .iter()
                .map(|x| self.max_flow(*x, opened, time_left - 1, seen))
                .max()
                .unwrap(),
        );

        seen.insert((current, opened, time_left), best);

        best
    }

    // Solves part one
    fn part1(&self) -> u32 {
        let mut memoize = HashMap::new();
        self.max_flow(self.valve_id("AA"), 0, 30, &mut memoize)
    }

    // Solve part two
    fn part2(&self) -> u32 {
        // in progress...
        0
    }
}

/// main function
fn main() {
    let args = Args::parse();
    let mut puzzle = Puzzle::new();
    puzzle.configure(&args.path);

    if args.verbose {
        puzzle.show();
    }

    println!("{}", puzzle.part1());
    println!("{}", puzzle.part2());
}

#[test]
fn test01() {
    let mut puzzle = Puzzle::new();
    puzzle.configure("test.txt");
    assert_eq!(puzzle.part1(), 1651);
    assert_eq!(puzzle.part2(), 0);
}
