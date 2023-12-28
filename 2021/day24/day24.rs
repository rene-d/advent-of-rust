//! [Day 24: Arithmetic Logic Unit](https://adventofcode.com/2021/day/24)

use rand::Rng;
use regex::Regex;
use std::fmt;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use structopt::StructOpt;

#[derive(Debug, Clone, Copy)]
enum OpCode {
    Inp, // input
    Add, // add
    Mul,
    Div,
    Mod,
    Eql,
}

#[derive(Debug, Clone, Copy)]
enum Register {
    W,
    X,
    Y,
    Z,
    Immediate,
    Input,
}

// #[derive(Debug)]
struct Instruction {
    opcode: OpCode,
    dest: Register, // target register
    src: Register,  // operand source
    immediate: i64, // if src==Immediate
}

/// implement `fmt::Debug` for Point
impl fmt::Debug for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let src = if let Register::Immediate = self.src {
            format!("{}", self.immediate)
        } else {
            format!("{:?}", self.src)
        };

        f.write_fmt(format_args!("{:?} {:?} {:6}", self.opcode, self.dest, src))
    }
}
fn load_program(data: &[String]) -> Vec<Instruction> {
    let re_inp = Regex::new(r"^inp ([wxyz])$").unwrap();
    let re_mul_imm = Regex::new(r"^mul ([wxyz]) (-?\d+)$").unwrap();
    let re_mul_reg = Regex::new(r"^mul ([wxyz]) ([wxyz])$").unwrap();
    let re_add_imm = Regex::new(r"^add ([wxyz]) (-?\d+)$").unwrap();
    let re_add_reg = Regex::new(r"^add ([wxyz]) ([wxyz])$").unwrap();
    let re_mod = Regex::new(r"^mod ([wxyz]) (\d+)$").unwrap();
    let re_div = Regex::new(r"^div ([wxyz]) (\d+)$").unwrap();
    let re_eq_imm = Regex::new(r"^eql ([wxyz]) (-?\d+)$").unwrap();
    let re_eq_reg = Regex::new(r"^eql ([wxyz]) ([wxyz])$").unwrap();

    let re_stmt: &[(OpCode, Register, Regex)] = &[
        (OpCode::Inp, Register::Input, re_inp),
        (OpCode::Mul, Register::Immediate, re_mul_imm),
        (OpCode::Mul, Register::W, re_mul_reg),
        (OpCode::Add, Register::Immediate, re_add_imm),
        (OpCode::Add, Register::W, re_add_reg),
        (OpCode::Mod, Register::Immediate, re_mod),
        (OpCode::Div, Register::Immediate, re_div),
        (OpCode::Eql, Register::Immediate, re_eq_imm),
        (OpCode::Eql, Register::W, re_eq_reg),
    ];

    let mut program: Vec<Instruction> = Vec::new();

    for line in data {
        for (opcode, target, re) in re_stmt {
            if let Some(a) = re.captures(line) {
                let dest = match a[1].as_ref() {
                    "w" => Register::W,
                    "x" => Register::X,
                    "y" => Register::Y,
                    "z" => Register::Z,
                    _ => unreachable!(),
                };

                match target {
                    Register::Input => {
                        program.push(Instruction {
                            opcode: *opcode,
                            dest,
                            src: Register::Input,
                            immediate: 0,
                        });
                    }
                    Register::Immediate => {
                        program.push(Instruction {
                            opcode: *opcode,
                            dest,
                            src: Register::Immediate,
                            immediate: a[2].parse().unwrap(),
                        });
                    }
                    _ => {
                        program.push(Instruction {
                            opcode: *opcode,
                            dest,
                            src: match a[2].as_ref() {
                                "w" => Register::W,
                                "x" => Register::X,
                                "y" => Register::Y,
                                "z" => Register::Z,
                                _ => unreachable!(),
                            },
                            immediate: 0,
                        });
                    }
                }

                // println!("{} -> {:?}", line, program.last().unwrap());
            }
        }
    }

    program
}

fn run_program(program: &[Instruction], input: &[i64], z: i64, verbose: bool) -> i64 {
    let mut registers: [i64; 4] = [0, 0, 0, z];
    let mut input_ptr = 0;

    for instruction in program {
        let src = match instruction.src {
            Register::Input => {
                assert!(input_ptr < input.len(), "not enough input");
                input_ptr += 1;
                input[input_ptr - 1]
            }
            Register::W => registers[0],
            Register::X => registers[1],
            Register::Y => registers[2],
            Register::Z => registers[3],
            Register::Immediate => instruction.immediate,
        };

        let mut dest_value: i64 = registers[instruction.dest as usize];

        dest_value = match instruction.opcode {
            OpCode::Inp => src,
            OpCode::Add => dest_value + src,
            OpCode::Mul => dest_value * src,
            OpCode::Div => {
                assert!(src != 0, "Division by zero!");
                dest_value / src
            }
            OpCode::Mod => {
                assert!(src != 0, "Division by zero!");
                dest_value % src
            }
            OpCode::Eql => i64::from(src == dest_value),
        };

        registers[instruction.dest as usize] = dest_value;

        if verbose {
            println!("{instruction:?} -> {registers:?}");
        }
    }

    if verbose {
        println!("{input:?} -> {registers:?}");
    }
    registers[Register::Z as usize]
}

fn run_box(w: i64, z: i64, div: i64, n1: i64, n2: i64) -> i64 {
    if w == z % 26 + n1 {
        z / div
    } else {
        w + (z / div) * 26 + n2
    }
}

fn solve(data: &[String]) {
    let boxes = &mut [(0i64, 0i64, 0i64); 14];

    for nbox in 0..14 {
        let box_program = data[(nbox * 18)..(nbox + 1) * 18].to_vec();
        let program = load_program(&box_program);

        let div = program[4].immediate;
        let n1 = program[5].immediate;
        let n2 = program[15].immediate;

        boxes[nbox] = (div, n1, n2);

        // verify that the box is correct
        for w in 1..10 {
            for z in 0..260 {
                assert_eq!(
                    run_box(w, z, div, n1, n2),
                    run_program(&program, &[w], z, false)
                );
            }
        }
    }

    // verify the boxes vs. the assembly-like program
    for _ in 0..10 {
        let z_init: i64 = rand::thread_rng().gen_range(0..1_000_000_000);
        let program = load_program(data);
        let digits = [1, 2, 3, 4, 5, 6, 7, 8, 9, 1, 2, 3, 4, 5];
        let z_program = run_program(&program, &digits, z_init, false);

        let mut z_boxes = z_init;
        for nbox in 0..14 {
            z_boxes = run_box(
                digits[nbox],
                z_boxes,
                boxes[nbox].0,
                boxes[nbox].1,
                boxes[nbox].2,
            );
        }
        assert_eq!(z_program, z_boxes);
    }

    // brute force a 14-digit input takes too much time
    // we can reduce to a 7-digit input since w is determined by z in boxes with div==26

    let mut valid_monads: Vec<i64> = Vec::new();

    for seed in 0..(9i64.pow(7)) {
        let mut digits = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        let mut z = 0;

        let mut w_seed = seed;
        for (nbox, params) in boxes.iter().enumerate() {
            let w: i64 = match params.0 {
                1 => {
                    let w = w_seed % 9 + 1;
                    w_seed /= 9;
                    w
                }
                26 => z % 26 + params.1,
                _ => panic!("bad divisor"),
            };

            if !(1..=9).contains(&w) {
                z = 1;
                break;
            }

            z = run_box(w, z, params.0, params.1, params.2);
            digits[nbox] = w;
        }
        if z == 0 {
            let monad = digits.iter().fold(0, |acc, &x| acc * 10 + x);
            valid_monads.push(monad);
        }
    }

    // println!("(valid MONAD count: {})", valid_monads.len());

    println!("{}", valid_monads.iter().max().unwrap());
    println!("{}", valid_monads.iter().min().unwrap());
}

/// parse command line arguments
#[derive(StructOpt)]
struct Cli {
    #[structopt(default_value = "input.txt", parse(from_os_str))]
    path: std::path::PathBuf,

    #[structopt(short = "-i", default_value = "")]
    input: String,

    //#[structopt(default_value = "")]
    #[structopt(short = "w", default_value = "0")]
    w: i64,

    //#[structopt(default_value = "")]
    #[structopt(short = "z", default_value = "0")]
    z: i64,
}

/// main function
fn main() {
    let args = Cli::from_args();
    // println!("reading data from: {}", args.path.display());
    let data = load_data(args.path);

    let program = load_program(&data);

    if data.len() == 252 && args.input.is_empty() {
        solve(&data);
    } else {
        let mut monad = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

        monad[0] = args.w;

        for (i, c) in args.input.chars().enumerate() {
            monad[i] = (c as i64) - 48;
        }

        run_program(&program, &monad, args.z, true);
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

/// load data from file
fn load_data(path: std::path::PathBuf) -> Vec<String> {
    let mut data = vec![];
    if let Ok(lines) = read_lines(path) {
        for line in lines.flatten() {
            data.push(line);
        }
    }
    data
}
