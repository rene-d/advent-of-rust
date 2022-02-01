/*!

*/

use std::convert::TryFrom;

pub const REG_A: usize = 0;
pub const REG_B: usize = 1;
pub const REG_C: usize = 2;
pub const REG_D: usize = 3;

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
pub struct Program {
    instructions: Vec<Instruction>,
    toggled: Vec<Instruction>,
    pub registers: [i32; 4],
    pub ip: usize,
    pub output: Option<i32>,
}

impl Program {
    /// `new` initializes a new program
    #[must_use]
    pub fn new(program: &str) -> Program {
        let mut p = Program {
            instructions: Vec::new(),
            toggled: Vec::new(),
            registers: [0; 4],
            ip: 0,
            output: None,
        };

        p.load(program);
        p
    }

    /// returns the length of the loaded program
    #[must_use]
    pub fn len(&self) -> usize {
        self.instructions.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.instructions.is_empty()
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
                Instruction::Dec(reg) | Instruction::Out(reg) | Instruction::Tgl(reg) => Instruction::Inc(*reg),

                // For two-argument instructions, jnz becomes cpy, and all other two-instructions become jnz.
                Instruction::Jnz(a, b) => Instruction::Cpy(*a, *b),
                Instruction::Cpy(a, b) => Instruction::Jnz(*a, *b),
            })
            .collect();

        assert_eq!(self.instructions.len(), self.toggled.len());

        self.reset();
    }

    /// run one instruction and advance the instruction pointer
    pub fn step(&mut self) {
        self.output = None;

        match &self.instructions[self.ip] {
            Instruction::Cpy(src, dest) => {
                if let RegOrValue::Register(reg_dest) = dest {
                    self.registers[*reg_dest] = match src {
                        RegOrValue::Register(reg_src) => self.registers[*reg_src],
                        RegOrValue::Value(value_src) => *value_src,
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

                if ip < self.len() {
                    std::mem::swap(&mut self.instructions[ip], &mut self.toggled[ip]);
                }
            }
        }

        self.ip += 1;
    }

    /// reset the program to the initial state
    pub fn reset(&mut self) {
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
    pub fn run(&mut self, max_iterations: usize) -> bool {
        let mut iterations = max_iterations;

        while self.ip < self.len() && iterations > 0 {
            self.step();
            iterations -= 1;
        }

        self.ip >= self.len() || iterations != 0
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

    let mut program = Program::new(demo);

    assert!(program.run(100));
    assert_eq!(program.registers[REG_A], 42);
}

#[test]
fn test_jnz() {
    assert_eq!(Program::new_ip(10, 2), 12);
    assert_eq!(Program::new_ip(10, -2), 8);
}

#[test]
fn test_inc_a() {
    let mut program = Program::new("inc a");
    program.registers[REG_A] = 42;
    program.step();
    assert_eq!(program.registers[REG_A], 43);
}

#[test]
fn test_dec_b() {
    let mut program = Program::new("dec b");
    program.registers[REG_B] = 42;
    program.step();
    assert_eq!(program.registers[REG_B], 41);
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

    let mut program = Program::new(demo);

    let ok = program.run(100);
    assert!(ok);

    assert_eq!(program.registers[REG_A], 3);
}
