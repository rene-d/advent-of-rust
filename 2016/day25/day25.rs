/*!
[Day 25: Clock Signal](https://adventofcode.com/2016/day/25)
*/

use std::convert::TryFrom;

/// index of a register
type Register = usize;

/// `to_reg` returns the index of a register, or panics if the string is not a valid register name.
fn to_reg(name: &str) -> Register {
    let name = name.chars().next().unwrap();
    match name {
        'a' => 0,
        'b' => 1,
        'c' => 2,
        'd' => 3,
        _ => panic!("Invalid register name: {}", name),
    }
}

/// instruction set of the processor
enum Instruction {
    Cpy(Register, Register),
    CpyValue(i32, Register),
    Inc(Register),
    Dec(Register),
    Jnz(Register, i32),
    Jmp(i32),
    Out(Register),
    Nop,
}

/// a program, the registers, with loader and executor
struct Program {
    instructions: Vec<Instruction>,
    registers: [i32; 4],
    ip: usize,
    output: Option<i32>,
}

impl Program {
    /// `new` initializes a new program
    fn new() -> Program {
        Program {
            instructions: Vec::new(),
            registers: [0; 4],
            ip: 0,
            output: None,
        }
    }

    /// `load` loads the program from a sequence of instructions
    fn load(&mut self, input: &str) {
        self.instructions = input
            .lines()
            .map(|line| {
                let mut words = line.split_whitespace();
                let instruction = words.next().unwrap();
                let source = words.next().unwrap();
                match instruction {
                    "cpy" => {
                        let dest = to_reg(words.next().unwrap());
                        if let Ok(value) = source.parse::<i32>() {
                            Instruction::CpyValue(value, dest)
                        } else {
                            Instruction::Cpy(to_reg(source), dest)
                        }
                    }
                    "inc" => Instruction::Inc(to_reg(source)),
                    "dec" => Instruction::Dec(to_reg(source)),
                    "jnz" => {
                        let offset = words.next().unwrap().parse::<i32>().unwrap();
                        if let Ok(value) = source.parse::<i32>() {
                            if value == 0 {
                                Instruction::Nop
                            } else {
                                Instruction::Jmp(offset)
                            }
                        } else {
                            Instruction::Jnz(to_reg(source), offset)
                        }
                    }
                    "out" => Instruction::Out(to_reg(source)),
                    _ => panic!("Unknown instruction: {}", instruction),
                }
            })
            .collect();
    }

    /// run one instruction and advance the instruction pointer
    fn step(&mut self) {
        self.output = None;

        match &self.instructions[self.ip] {
            Instruction::Nop => {}
            Instruction::Cpy(src, dest) => {
                self.registers[*dest] = self.registers[*src];
            }
            Instruction::CpyValue(value, dest) => {
                self.registers[*dest] = *value;
            }
            Instruction::Inc(reg) => {
                self.registers[*reg] += 1;
            }
            Instruction::Dec(reg) => {
                self.registers[*reg] -= 1;
            }
            Instruction::Jnz(reg, offset) => {
                if self.registers[*reg] != 0 {
                    self.ip = Program::new_ip(self.ip, *offset);
                    return;
                }
            }
            Instruction::Jmp(offset) => {
                self.ip = Program::new_ip(self.ip, *offset);
                return;
            }
            Instruction::Out(reg) => {
                let value = self.registers[*reg];
                self.output = Some(value);
            }
        }
        self.ip += 1;
    }

    /// reset the program to the initial state
    fn reset(&mut self) {
        self.registers = [0, 0, 0, 0];
        self.ip = 0;
        self.output = None;
    }

    /// run the program and returns the value of register `a`
    fn run_clock_signal(&mut self, a: i32) -> bool {
        self.reset();

        self.registers[to_reg("a")] = a;

        let mut output = [0; 256];
        let mut output_index = 0;

        while self.ip < self.instructions.len() && output_index < output.len() {
            self.step();

            if let Some(value) = self.output {
                if value != i32::try_from(output_index).unwrap() % 2 {
                    return false;
                }
                output[output_index] = value;
                output_index += 1;
            }
        }

        true
    }

    /// `new_ip` returns the new instruction pointer after jumping `offset` instructions
    fn new_ip(ip: usize, offset: i32) -> usize {
        if offset >= 0 {
            ip.checked_add(usize::try_from(offset).unwrap()).unwrap()
        } else {
            ip.checked_sub(usize::try_from(-offset).unwrap()).unwrap()
        }
    }
}

fn main() {
    let data = std::fs::read_to_string("input.txt").unwrap();

    let mut program = Program::new();

    program.load(&data);

    for a in 0..10000 {
        if program.run_clock_signal(a) {
            println!("{}", a);
            break;
        }
    }
}
