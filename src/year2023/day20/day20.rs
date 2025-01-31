//! [Day 20: Pulse Propagation](https://adventofcode.com/2023/day/20)

use num::Integer;
use rustc_hash::FxHashMap;
use std::collections::VecDeque;

fn lcm(values: &[u64]) -> u64 {
    let mut m = 1;
    for x in values {
        m = m.lcm(x);
    }
    m
}

#[derive(PartialEq, Clone)]
enum ModuleType {
    Broadcaster,
    Flipflop,
    Conjunction,
}

#[derive(PartialEq, Clone, Copy)]
enum Pulse {
    Low,
    High,
}

#[derive(PartialEq, Clone, Copy)]
enum State {
    Off,
    On,
}

impl std::ops::Not for State {
    type Output = Self;

    fn not(self) -> Self {
        match self {
            Self::On => Self::Off,
            Self::Off => Self::On,
        }
    }
}

#[derive(Clone)]
struct Module {
    id: u32,
    kind: ModuleType,
    outputs: Vec<u32>,
    state: State,                  // only for Flip-flop module
    memory: FxHashMap<u32, Pulse>, // only for Conjunction module
}

fn get_id(name: &str) -> u32 {
    if name == "broadcaster" {
        return 0;
    }
    assert!(name.chars().all(|c| c.is_ascii_lowercase()));
    name.chars().fold(0, |acc, c| acc * 26 + (c as u32) - 96)
}

struct Puzzle {
    modules: FxHashMap<u32, Module>,
}

impl Puzzle {
    fn new(data: &str) -> Self {
        let mut modules = FxHashMap::default();

        for line in data.lines() {
            let (name, dests) = line.split_once(" -> ").unwrap();
            let outputs: Vec<u32> = dests.split(", ").map(get_id).collect();

            let (name, mtype) = match &name[0..1] {
                "%" => (&name[1..], ModuleType::Flipflop),
                "&" => (&name[1..], ModuleType::Conjunction),
                _ => (name, ModuleType::Broadcaster),
            };

            let id = get_id(name);

            modules.insert(
                id,
                Module {
                    id,
                    kind: mtype,
                    outputs,
                    state: State::Off,
                    memory: FxHashMap::default(),
                },
            );
        }

        let values = modules.clone();

        // find modules that are connected to a Conjunction module:
        // they feed pulses the module remembers to send its pulse
        for module in modules.values_mut() {
            if module.kind == ModuleType::Conjunction {
                for m in values.values() {
                    //   ^== borrowing problem here
                    if m.outputs.contains(&module.id) {
                        module.memory.insert(m.id, Pulse::Low);
                    }
                }
            }
        }

        Self { modules }
    }

    /// Reset the modules to their initial state.
    fn reset(&mut self) {
        for module in self.modules.values_mut() {
            module.state = State::Off;
            for level in module.memory.values_mut() {
                *level = Pulse::Low;
            }
        }
    }

    /// Propagate a pulse.
    fn propagate(
        &mut self,
        source: u32,
        target: u32,
        pulse: Pulse,
        q: &mut VecDeque<(u32, u32, Pulse)>,
    ) {
        if let Some(module) = self.modules.get_mut(&target) {
            if module.kind == ModuleType::Broadcaster {
                for &output in &module.outputs {
                    q.push_back((target, output, pulse));
                }
            } else if module.kind == ModuleType::Flipflop {
                if pulse == Pulse::Low {
                    module.state = !module.state;

                    let outgoing = match module.state {
                        State::Off => Pulse::Low,
                        State::On => Pulse::High,
                    };
                    for e in &module.outputs {
                        q.push_back((module.id, *e, outgoing));
                    }
                }
            } else if module.kind == ModuleType::Conjunction {
                module.memory.insert(source, pulse);

                let outgoing = if module.memory.values().all(|&level| level == Pulse::High) {
                    Pulse::Low
                } else {
                    Pulse::High
                };

                for e in &module.outputs {
                    q.push_back((module.id, *e, outgoing));
                }
            }
        }
    }

    fn press(&mut self, f: &mut dyn core::ops::FnMut(u32, u32, Pulse) -> bool) -> bool {
        let mut q: VecDeque<(u32, u32, Pulse)> = VecDeque::new();
        q.push_back((0, get_id("broadcaster"), Pulse::Low));

        while let Some((source, target, pulse)) = q.pop_front() {
            if !f(source, target, pulse) {
                return false;
            }
            self.propagate(source, target, pulse, &mut q);
        }

        true
    }

    /// Solve part one.
    fn part1(&mut self) -> u32 {
        self.reset();

        let mut lo = 0;
        let mut hi = 0;

        for _ in 0..1000 {
            self.press(&mut |_, _, pulse| {
                match pulse {
                    Pulse::Low => lo += 1,
                    Pulse::High => hi += 1,
                }
                true // continue to propagate the pulses
            });
        }
        lo * hi
    }

    /// Solve part two.
    fn part2(&mut self) -> u64 {
        self.reset();

        let rx = get_id("rx");

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
        let rx_feed = rx_feed.unwrap().id;

        // all  modules that send to the rx_feed module
        let rx_feed_inputs = self
            .modules
            .values()
            .filter(|module| module.outputs.contains(&rx_feed))
            .count();

        // the count of presses to have a High pulse on each inputs of the rx_feed module
        let mut rx_feed_input_presses = FxHashMap::default();

        for presses in 1.. {
            let stopped = !self.press(&mut |source, target, pulse| {
                if target == rx_feed && pulse == Pulse::High {
                    // update the presses for the current input of the rx_feed module
                    rx_feed_input_presses
                        .entry(source.to_string())
                        .or_insert(presses);

                    // we have enough values
                    if rx_feed_input_presses.len() == rx_feed_inputs {
                        return false; // stop the circuit
                    }
                }
                true
            });

            if stopped {
                // the circuit is stopped: we have found the solution
                let v: Vec<u64> = rx_feed_input_presses.values().copied().collect();
                return lcm(&v);
            }
        }

        0
    }
}

/// # Panics
#[must_use]
pub fn solve(data: &str) -> (u32, u64) {
    let mut puzzle = Puzzle::new(data);
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
    const TEST_INPUT_2: &str = include_str!("test2.txt");

    #[test]
    fn test01() {
        let mut puzzle = Puzzle::new(TEST_INPUT);
        assert_eq!(puzzle.part1(), 32_000_000);
    }

    #[test]
    fn test02() {
        let mut puzzle = Puzzle::new(TEST_INPUT_2);
        assert_eq!(puzzle.part1(), 11_687_500);
    }
}
