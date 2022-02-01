/*!

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

#[derive(Copy, Clone)]
enum RegOrValue {
    Register(Register),
    Value(i32),
}

impl RegOrValue {
    fn from_str(s: &str) -> RegOrValue {
        if let Ok(value) = s.parse::<i32>() {
            RegOrValue::Value(value)
        } else {
            RegOrValue::Register(to_reg(s))
        }
    }
}

/// instruction set of the processor
#[derive(Copy, Clone)]
enum Instruction {
    Cpy(RegOrValue, RegOrValue),
    Inc(Register),
    Dec(Register),
    Jnz(RegOrValue, RegOrValue),
    Out(Register),
    Tgl(Register),
}

/// a program, the registers, with loader and executor
struct Program {
    instructions: Vec<Instruction>,
    toggled: Vec<Instruction>,
    registers: [i32; 4],
    ip: usize,
    output: Option<i32>,
}

impl Program {
    /// `new` initializes a new program
    fn new() -> Program {
        Program {
            instructions: Vec::new(),
            toggled: Vec::new(),
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
                        let dest = words.next().unwrap();
                        Instruction::Cpy(RegOrValue::from_str(source), RegOrValue::from_str(dest))
                    }
                    "inc" => Instruction::Inc(to_reg(source)),
                    "dec" => Instruction::Dec(to_reg(source)),
                    "jnz" => {
                        let offset = words.next().unwrap();
                        Instruction::Jnz(RegOrValue::from_str(source), RegOrValue::from_str(offset))
                    }
                    "out" => Instruction::Out(to_reg(source)),
                    "tgl" => Instruction::Tgl(to_reg(source)),
                    _ => panic!("Unknown instruction: {}", instruction),
                }
            })
            .collect();

        self.toggled = self
            .instructions
            .iter()
            .map(|instruction| match instruction {
                // For one-argument instructions, inc becomes dec, and all other one-argument instructions become inc.
                Instruction::Inc(reg) => Instruction::Dec(*reg),
                Instruction::Dec(reg) => Instruction::Inc(*reg),
                Instruction::Out(reg) => Instruction::Inc(*reg),
                Instruction::Tgl(reg) => Instruction::Inc(*reg),

                // For two-argument instructions, jnz becomes cpy, and all other two-instructions become jnz.
                Instruction::Jnz(a, b) => Instruction::Cpy(*a, *b),
                Instruction::Cpy(a, b) => Instruction::Jnz(*a, *b),
            })
            .collect();

        self.reset();
    }

    /// run one instruction and advance the instruction pointer
    fn step(&mut self) {
        self.output = None;

        match &self.instructions[self.ip] {
            Instruction::Cpy(src, dest) => {
                if let RegOrValue::Register(reg) = dest {
                    self.registers[*reg] = match src {
                        RegOrValue::Register(reg) => self.registers[*reg],
                        RegOrValue::Value(value) => *value,
                    }
                }
            }
            Instruction::Inc(reg) => {
                self.registers[*reg] += 1;
            }
            Instruction::Dec(reg) => {
                self.registers[*reg] -= 1;
            }
            Instruction::Jnz(cond, offset) => {
                let cond = match cond {
                    RegOrValue::Register(reg) => self.registers[*reg],
                    RegOrValue::Value(value) => *value,
                };
                if cond != 0 {
                    let offset = match offset {
                        RegOrValue::Register(reg) => self.registers[*reg],
                        RegOrValue::Value(value) => *value,
                    };
                    self.ip = Program::new_ip(self.ip, offset);
                    return;
                }
            }
            Instruction::Out(reg) => {
                let value = self.registers[*reg];
                self.output = Some(value);
            }
            Instruction::Tgl(reg) => {
                let ip = Program::new_ip(self.ip, self.registers[*reg]);

                if ip < self.instructions.len() {
                    let x = self.instructions[ip];
                    self.instructions[ip] = self.toggled[ip];
                    self.toggled[ip] = x;
                }
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

    /// `new_ip` returns the new instruction pointer after jumping `offset` instructions
    fn new_ip(ip: usize, offset: i32) -> usize {
        if offset >= 0 {
            ip.checked_add(usize::try_from(offset).unwrap()).unwrap()
        } else {
            ip.checked_sub(usize::try_from(-offset).unwrap()).unwrap()
        }
    }

    /// run the program and returns the value of register `a`
    fn run(&mut self) -> bool {
        let mut max_iterations = 1000;

        while self.ip < self.instructions.len() && max_iterations > 0 {
            self.step();
            max_iterations -= 1;
        }

        self.ip >= self.instructions.len()
    }
}

fn main() {
    let data = std::fs::read_to_string("input.txt").unwrap();

    let mut program = Program::new();
    program.load(&data);

    for a in 0..1000000 {
        program.reset();
        program.registers[0] = a;
        let success = program.run();

        if success {
            println!("{} {:?}", a, success);
            break;
        }
    }
}

#[test]
fn test_program() {
    let demo = "cpy 41 a
inc a
inc a
dec a
jnz a 2
dec a";

    let mut program = Program::new();
    program.load(demo);

    assert!(program.run());
    assert_eq!(program.registers[0], 42);
}

#[test]
fn test_jnz() {
    assert_eq!(Program::new_ip(10, 2), 12);
    assert_eq!(Program::new_ip(10, -2), 8);
}

#[test]
fn test_inc_a() {
    let mut program = Program::new();
    program.load("inc a");
    program.registers[0] = 42;
    program.step();
    assert_eq!(program.registers[0], 43);
}

#[test]
fn test_dec_b() {
    let mut program = Program::new();
    program.load("dec b");
    program.registers[1] = 42;
    program.step();
    assert_eq!(program.registers[1], 41);
}

#[test]
fn test_toggle() {
    let demo = "cpy 2 a
tgl a
tgl a
tgl a
cpy 1 a
dec a
dec a";

    let mut program = Program::new();
    program.load(demo);

    let ok = program.run();
    assert!(ok);

    assert_eq!(program.registers[0], 3);
}
