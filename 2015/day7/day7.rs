//! [Day 7: Some Assembly Required](https://adventofcode.com/2015/day/7)

use regex::Regex;
use rustc_hash::FxHashMap;

/// main function
fn main() {
    let args = aoc::parse_args();
    let data = args
        .input
        .lines()
        .map(std::string::ToString::to_string)
        .collect::<Vec<String>>();

    let mut opcodes: FxHashMap<String, String> = FxHashMap::default();
    let re_opcode = Regex::new(r"^(.+) \-> (\w+)$").unwrap();
    for line in &data {
        if let Some(op) = re_opcode.captures(line) {
            opcodes.insert(op[2].to_string(), op[1].to_string());
        }
    }

    // part 1
    let wire_a = wires(&opcodes, "a");
    println!("{wire_a}");

    // part 2
    opcodes.insert("b".to_string(), wire_a.to_string());
    let wire_a_bis = wires(&opcodes, "a");
    println!("{wire_a_bis}");
}

fn run(
    opcodes: &FxHashMap<String, String>,
    cache: &mut FxHashMap<String, u16>,
    reg: &str,
    level: u32,
) -> u16 {
    assert!(level <= 70, "too deep");

    if cache.contains_key(reg) {
        return cache[reg];
    }

    if let Ok(value) = reg.parse::<u16>() {
        return value;
    }

    if opcodes.contains_key(reg) {
        let re_num = Regex::new(r"^(\d+)$").unwrap();
        let re_copy = Regex::new(r"^(\w+)$").unwrap();
        let re_binary = Regex::new(r"^(\w+) (AND|OR) (\w+)$").unwrap();
        let re_unary = Regex::new(r"^(NOT) (\w+)$").unwrap();
        let re_shift = Regex::new(r"^(\w+) (RSHIFT|LSHIFT) (\d+)$").unwrap();

        let op = opcodes.get(reg).unwrap();

        // let indent = (0..level).map(|_| " ").collect::<String>();
        // println!("{} reg: {} = {}", indent, reg, op);

        let value: u16;

        if let Some(op) = re_num.captures(op) {
            // 123 -> x
            value = op[1].parse::<u16>().unwrap();
        } else if let Some(op) = re_copy.captures(op) {
            // lx -> a
            let src = op[1].to_string();
            return run(opcodes, cache, &src, level + 1);
        } else if let Some(op) = re_binary.captures(op) {
            // a AND b -> d
            let src1 = op[1].to_string();
            let opx = op[2].to_string();
            let src2 = op[3].to_string();

            let v1 = run(opcodes, cache, &src1, level + 1);
            let v2 = run(opcodes, cache, &src2, level + 1);
            if opx == "AND" {
                value = v1 & v2;
            } else {
                value = v1 | v2;
            }
        } else if let Some(op) = re_unary.captures(op) {
            // NOT a -> b
            let src = op[2].to_string();
            value = !run(opcodes, cache, &src, level + 1);
        } else if let Some(op) = re_shift.captures(op) {
            // a RSHIFT 2 -> c
            let src = op[1].to_string();
            let opx = op[2].to_string();
            let shift = op[3].parse::<u8>().unwrap();

            let v = run(opcodes, cache, &src, level + 1);

            match opx.as_ref() {
                "RSHIFT" => {
                    value = v >> shift;
                }
                "LSHIFT" => {
                    value = v << shift;
                }
                _ => {
                    panic!("unknown shift operation");
                }
            }
        } else {
            panic!("unknown opcode <{op}>");
        }

        cache.insert(reg.to_string(), value);
        value
    } else {
        panic!("unknown register {reg} ");
    }
}

fn wires(opcodes: &FxHashMap<String, String>, wire: &str) -> u16 {
    let mut values: FxHashMap<String, u16> = FxHashMap::default();

    run(opcodes, &mut values, wire, 0)
}
