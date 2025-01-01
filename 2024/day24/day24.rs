//! [Day 24: Crossed Wires](https://adventofcode.com/2024/day/24)

// Circuit diagrams are copied from:
// https://www.researchgate.net/publication/349727409_PyQUBO_Python_Library_for_Mapping_Combinatorial_Optimization_Problems_to_QUBO_Form
//
// @unknown{unknown,
// author = {Zaman, Mashiyat and Tanahashi, Kotaro and Tanaka, Shu},
// year = {2021},
// month = {03},
// pages = {},
// title = {PyQUBO: Python Library for Mapping Combinatorial Optimization Problems to QUBO Form},
// doi = {10.48550/arXiv.2103.01708}
// }
//

use rustc_hash::{FxHashMap, FxHashSet};

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
enum Role {
    CarryOut,     // the Cout wire
    IntXorXor,    // intermediate wire between the two XOR gates
    ABAndGate,    // intermediate wires between AB and the (bottom) AND gate
    AndGateWires, // wiring of the AND gates
    SumOut,       // the S wire
}

#[derive(Eq, PartialEq, Hash, Clone)]
enum Operation {
    And,
    Or,
    Xor,
}

impl Operation {
    fn from(s: &str) -> Self {
        match s {
            "AND" => Self::And,
            "OR" => Self::Or,
            "XOR" => Self::Xor,
            _ => panic!("unknown op {s}"),
        }
    }

    const fn eval(&self, a: u8, b: u8) -> u8 {
        match self {
            Self::And => a & b,
            Self::Or => a | b,
            Self::Xor => a ^ b,
        }
    }
}

#[derive(Eq, PartialEq, Hash, Clone)]
struct Gate {
    a: String,     // input wire
    b: String,     // input wire
    op: Operation, // type of gate
    r: String,     // output wire
}

fn is_role(set: &FxHashSet<Role>, f: &Role) -> bool {
    set.len() == 1 && (set.iter().next().unwrap() == f)
}

fn is_roles(set: &FxHashSet<Role>, f1: &Role, f2: &Role) -> bool {
    if set.len() != 2 {
        return false;
    }

    let roles: Vec<_> = set.iter().collect();
    (roles[0] == f1 && roles[1] == f2) || (roles[0] == f2 && roles[1] == f1)
}

struct Puzzle {
    // data: String,
    wires: FxHashMap<String, u8>,
    gates: Vec<Gate>,
}

impl Puzzle {
    fn new() -> Self {
        Self {
            wires: FxHashMap::default(),
            gates: Vec::new(),
        }
    }

    /// Get the puzzle input.
    fn configure(&mut self, data: &str) {
        for line in data.lines() {
            if line.contains(": ") {
                let (wire, value) = line.split_once(": ").unwrap();
                self.wires.insert(wire.to_string(), value.parse().unwrap());
            }
            if line.contains(" -> ") {
                let v = line.split_ascii_whitespace().collect::<Vec<_>>();

                let gate = Gate {
                    a: v[0].to_string(),
                    op: Operation::from(v[1]),
                    b: v[2].to_string(),
                    r: v[4].to_string(),
                };

                self.gates.push(gate);
            }
        }
    }

    /// Solve part one.
    fn part1(&self) -> u64 {
        let mut waiting_gates = self.gates.iter().collect::<Vec<_>>();
        let mut wires = self.wires.clone();

        while !waiting_gates.is_empty() {
            let mut next_waiting = Vec::new();

            for gate in &waiting_gates {
                if let Some(&a) = wires.get(&gate.a) {
                    if let Some(&b) = wires.get(&gate.b) {
                        let r = gate.op.eval(a, b);
                        *wires.entry(gate.r.to_string()).or_default() = r;
                        continue;
                    }
                }

                next_waiting.push(*gate);
            }

            waiting_gates = next_waiting;
        }

        wires
            .iter()
            .filter(|(r, &v)| r.starts_with('z') && v == 1)
            .fold(0_u64, |acc, (r, _)| {
                acc | (1 << r[1..].parse::<u64>().unwrap())
            })
    }

    /// Solve part two.
    fn part2(&self) -> String {
        let mut input_types: FxHashMap<&str, FxHashSet<Role>> = FxHashMap::default();
        let mut result_types: FxHashMap<&str, FxHashSet<Role>> = FxHashMap::default();

        // analyse the role of each gate

        for gate in &self.gates {
            if gate.a == "x00" && gate.b == "y00" || gate.b == "x00" && gate.a == "y00" {
                // ignore first half adder
                continue;
            }

            let mut add_result_role =
                |r: &Role| result_types.entry(&gate.r).or_default().insert(r.clone());

            // full adder
            if (gate.a.starts_with('x') && gate.b.starts_with('y'))
                || (gate.a.starts_with('y') && gate.b.starts_with('x'))
            {
                add_result_role(match gate.op {
                    Operation::Xor => &Role::IntXorXor, // xy connected to the 1st XOR gate: output is the wire between the both XOR
                    Operation::And => &Role::ABAndGate, // xy wired to a AND gate
                    Operation::Or => panic!("OR gate should be wired to x/y"),
                });
            } else {
                let role = match gate.op {
                    Operation::Xor => &Role::SumOut,       // actually the 2nd XOR gate
                    Operation::And => &Role::AndGateWires, // connections of AND gates
                    Operation::Or => &Role::CarryOut,      // the only one OR gate is wired to Cout
                };

                input_types.entry(&gate.a).or_default().insert(role.clone());
                input_types.entry(&gate.b).or_default().insert(role.clone());
                add_result_role(role);
            }
        }

        // branch all logical adders

        let last_z_wire = result_types
            .keys()
            .filter(|wire| wire.starts_with('z'))
            .max()
            .unwrap();

        let mut bad_wires: Vec<&str> = Vec::new();

        for wire in result_types.keys() {
            let inp = &input_types.entry(wire).or_default();
            let res = &result_types[wire];

            if wire == last_z_wire && is_role(res, &Role::CarryOut) {
                // ok, last wire/bit of the result register should be wired to CarryOut
                continue;
            }

            if inp.is_empty() && wire.starts_with('z') && is_role(res, &Role::SumOut) {
                // ok, other z wires are Sum outputs
                continue;
            }

            if is_role(inp, &Role::CarryOut)
                && (is_role(res, &Role::AndGateWires) || is_role(res, &Role::ABAndGate))
            {
                // ok: CarryOut should be wired to 2nd XOR or a AND gate
                continue;
            }

            if is_roles(inp, &Role::SumOut, &Role::AndGateWires)
                && (is_role(res, &Role::CarryOut) || is_role(res, &Role::IntXorXor))
            {
                // ok: Cin and Sum should be wired to Cout or
                continue;
            }

            #[cfg(debug_assertions)]
            eprintln!("❌ {wire} : {inp:?} → {res:?}");

            // ⚠️ swapped wire pairs are not determined,
            // ⚠️ just the eight incorrectly wired

            bad_wires.push(wire);
        }

        bad_wires.sort_unstable();
        bad_wires.join(",")
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
    fn test_part1() {
        let mut puzzle = Puzzle::new();
        let data = aoc::load_input_data("sample_1.txt");
        puzzle.configure(&data);
        assert_eq!(puzzle.part1(), 4);
    }

    #[test]
    fn test_part2() {
        let mut puzzle = Puzzle::new();
        let data = aoc::load_input_data("sample_2.txt");
        puzzle.configure(&data);
        assert_eq!(puzzle.part1(), 2024);
    }
}
