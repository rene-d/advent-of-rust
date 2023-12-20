//! [Day 20: Pulse Propagation](https://adventofcode.com/2023/day/20)

use num::Integer;
use std::collections::{HashMap, VecDeque};

use clap::Parser;

#[derive(Parser)]
struct Args {
    /// Puzzle input
    #[arg(default_value = "input.txt")]
    path: String,
}

fn lcm(values: &[u64]) -> u64 {
    let mut m = 1;
    for x in values {
        m = m.lcm(x);
    }
    m
}

#[derive(PartialEq, Clone, Debug)]
enum ModuleType {
    Broadcaster,
    Flipflop,
    Conjunction,
}

#[derive(PartialEq, Clone, Debug, Copy)]
enum Pulse {
    Low,
    High,
}

#[derive(PartialEq, Clone, Debug, Copy)]
enum State {
    Off,
    On,
}

impl std::ops::Not for State {
    type Output = Self;

    fn not(self) -> State {
        match self {
            State::On => State::Off,
            State::Off => State::On,
        }
    }
}

#[derive(Debug, Clone)]
struct Module {
    name: String,
    kind: ModuleType,
    outputs: Vec<String>,
    state: State,
    memory: HashMap<String, Pulse>,
}

struct Puzzle {
    modules: HashMap<String, Module>,
}

impl Puzzle {
    fn new() -> Puzzle {
        Puzzle {
            modules: HashMap::new(),
        }
    }

    /// Get the puzzle input.
    fn configure(&mut self, path: &str) {
        let data = std::fs::read_to_string(path).unwrap();

        for line in data.lines() {
            let (name, dests) = line.split_once(" -> ").unwrap();
            let outputs: Vec<String> = dests.split(", ").map(String::from).collect();

            let (name, mtype) = match &name[0..1] {
                "%" => (&name[1..], ModuleType::Flipflop),
                "&" => (&name[1..], ModuleType::Conjunction),
                _ => (name, ModuleType::Broadcaster),
            };

            self.modules.insert(
                name.to_string(),
                Module {
                    name: name.to_string(),
                    kind: mtype,
                    outputs,
                    state: State::Off,
                    memory: HashMap::new(),
                },
            );
        }

        let toto = self.modules.clone();

        for module in self.modules.values_mut() {
            if module.kind == ModuleType::Conjunction {
                for m in toto.values() {
                    // <== borrowing problem here
                    if m.outputs.contains(&module.name) {
                        module.memory.insert(m.name.clone(), Pulse::Low);
                    }
                }
            }
        }
    }

    fn reset(&mut self) {
        for module in self.modules.values_mut() {
            module.state = State::Off;
            for level in module.memory.values_mut() {
                *level = Pulse::Low;
            }
        }
    }

    fn propagate(
        &mut self,
        source: &str,
        target: &String,
        pulse: Pulse,
        q: &mut VecDeque<(String, String, Pulse)>,
    ) {
        if let Some(module) = self.modules.get_mut(target) {
            if module.kind == ModuleType::Broadcaster {
                for output in &module.outputs {
                    q.push_back((target.clone(), output.clone(), pulse));
                }
            } else if module.kind == ModuleType::Flipflop {
                if pulse == Pulse::Low {
                    module.state = !module.state;

                    let outgoing = match module.state {
                        State::Off => Pulse::Low,
                        State::On => Pulse::High,
                    };
                    for e in &module.outputs {
                        q.push_back((module.name.clone(), e.clone(), outgoing));
                    }
                }
            } else if module.kind == ModuleType::Conjunction {
                module.memory.insert(source.to_string(), pulse);

                let outgoing = if module.memory.values().all(|&level| level == Pulse::High) {
                    Pulse::Low
                } else {
                    Pulse::High
                };

                for e in &module.outputs {
                    q.push_back((module.name.clone(), e.clone(), outgoing));
                }
            }
        }
    }

    /// Solve part one.
    fn part1(&mut self) -> u32 {
        self.reset();

        let mut lo = 0;
        let mut hi = 0;

        for _ in 0..1000 {
            let mut q: VecDeque<(String, String, Pulse)> = VecDeque::new();
            q.push_back(("button".to_string(), "broadcaster".to_string(), Pulse::Low));

            while let Some((source, target, pulse)) = q.pop_front() {
                match pulse {
                    Pulse::Low => lo += 1,
                    Pulse::High => hi += 1,
                };

                self.propagate(&source, &target, pulse, &mut q);
            }
        }
        lo * hi
    }

    /// Solve part two.
    fn part2(&mut self) -> u64 {
        self.reset();

        let rx = "rx".to_string();

        // find the module that feeds rx module
        // &module -> rx
        let rx_feed = self
            .modules
            .values()
            .filter(|&module| module.outputs.contains(&rx))
            .nth(0);

        // it must be a Conjunction module (otherwise the problem is worthless)
        if rx_feed.is_none() || rx_feed.unwrap().kind != ModuleType::Conjunction {
            return 0;
        }

        // get its name
        let rx_feed = rx_feed.unwrap().name.to_string();

        // all  modules that send to the rx_feed module
        let rx_feed_inputs = self
            .modules
            .values()
            .filter(|module| module.outputs.contains(&rx_feed))
            .count();

        // the count of presses to have a High pulse on each inputs of the rx_feed module
        let mut rx_feed_input_presses = HashMap::new();

        for presses in 1.. {
            let mut q: VecDeque<(String, String, Pulse)> = VecDeque::new();
            q.push_back(("button".to_string(), "broadcaster".to_string(), Pulse::Low));

            while let Some((source, target, pulse)) = q.pop_front() {
                if target == rx_feed && pulse == Pulse::High {
                    // update the presses for the current input of the rx_feed module
                    rx_feed_input_presses
                        .entry(source.clone())
                        .or_insert(presses);

                    // we have enough values
                    if rx_feed_input_presses.len() == rx_feed_inputs {
                        let v: Vec<u64> = rx_feed_input_presses.values().copied().collect();
                        return lcm(&v);
                    }
                }

                self.propagate(&source, &target, pulse, &mut q);
            }
        }

        0
    }
}

fn main() {
    let args = Args::parse();
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
        assert_eq!(puzzle.part1(), 32_000_000);
    }

    #[test]
    fn test02() {
        let mut puzzle = Puzzle::new();
        puzzle.configure("test2.txt");
        assert_eq!(puzzle.part1(), 11_687_500);
    }
}
