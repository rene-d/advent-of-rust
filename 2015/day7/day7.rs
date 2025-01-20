//! [Day 7: Some Assembly Required](https://adventofcode.com/2015/day/7)

use regex::Regex;
use rustc_hash::FxHashMap;

struct Parser {
    num: Regex,
    copy: Regex,
    binary: Regex,
    unary: Regex,
    shift: Regex,
}

impl Parser {
    fn new() -> Self {
        Self {
            num: Regex::new(r"^(\d+)$").unwrap(),
            copy: Regex::new(r"^(\w+)$").unwrap(),
            binary: Regex::new(r"^(\w+) (AND|OR) (\w+)$").unwrap(),
            unary: Regex::new(r"^(NOT) (\w+)$").unwrap(),
            shift: Regex::new(r"^(\w+) (RSHIFT|LSHIFT) (\d+)$").unwrap(),
        }
    }
}

fn run(
    parser: &Parser,
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
        let op = opcodes.get(reg).unwrap();

        // let indent = (0..level).map(|_| " ").collect::<String>();
        // println!("{} reg: {} = {}", indent, reg, op);

        let value: u16;

        if let Some(op) = parser.num.captures(op) {
            // 123 -> x
            value = op[1].parse::<u16>().unwrap();
        } else if let Some(op) = parser.copy.captures(op) {
            // lx -> a
            let src = op[1].to_string();
            return run(parser, opcodes, cache, &src, level + 1);
        } else if let Some(op) = parser.binary.captures(op) {
            // a AND b -> d
            let src1 = op[1].to_string();
            let opx = op[2].to_string();
            let src2 = op[3].to_string();

            let v1 = run(parser, opcodes, cache, &src1, level + 1);
            let v2 = run(parser, opcodes, cache, &src2, level + 1);
            if opx == "AND" {
                value = v1 & v2;
            } else {
                value = v1 | v2;
            }
        } else if let Some(op) = parser.unary.captures(op) {
            // NOT a -> b
            let src = op[2].to_string();
            value = !run(parser, opcodes, cache, &src, level + 1);
        } else if let Some(op) = parser.shift.captures(op) {
            // a RSHIFT 2 -> c
            let src = op[1].to_string();
            let opx = op[2].to_string();
            let shift = op[3].parse::<u8>().unwrap();

            let v = run(parser, opcodes, cache, &src, level + 1);

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

fn wires(parser: &Parser, opcodes: &FxHashMap<String, String>, wire: &str) -> u16 {
    let mut values: FxHashMap<String, u16> = FxHashMap::default();

    run(parser, opcodes, &mut values, wire, 0)
}

fn solve(data: &str) -> (u16, u16) {
    let parser = Parser::new();

    let mut opcodes: FxHashMap<String, String> = FxHashMap::default();
    let re_opcode = Regex::new(r"^(.+) \-> (\w+)$").unwrap();
    for line in data.lines() {
        if let Some(op) = re_opcode.captures(line) {
            opcodes.insert(op[2].to_string(), op[1].to_string());
        }
    }

    // part 1
    let wire_a = wires(&parser, &opcodes, "a");

    // part 2
    opcodes.insert("b".to_string(), wire_a.to_string());
    let wire_a_bis = wires(&parser, &opcodes, "a");

    (wire_a, wire_a_bis)
}

/// main function
fn main() {
    let mut args = aoc::parse_args();

    args.run(solve);
}
