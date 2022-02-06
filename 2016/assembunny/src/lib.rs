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
        'a' => REG_A,
        'b' => REG_B,
        'c' => REG_C,
        'd' => REG_D,
        _ => panic!("Invalid register name: {}", name),
    }
}

fn reg_name(r: Register) -> char {
    match r {
        0 => 'a',
        1 => 'b',
        REG_C => 'c',
        REG_D => 'd',
        _ => panic!("Invalid register index: {}", r),
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

impl std::fmt::Display for RegOrValue {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                RegOrValue::Register(reg) => format!("{}", reg_name(*reg)),
                RegOrValue::Value(value) => format!("{}", value),
            }
        )
    }
}

/// the instruction set of the processor
#[derive(Copy, Clone)]
enum Instruction {
    Cpy(RegOrValue, RegOrValue),
    Inc(Register),
    Dec(Register),
    Jnz(RegOrValue, RegOrValue),
    Out(RegOrValue),
    Tgl(Register),
    Nop,
}

impl std::fmt::Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match &self {
                Instruction::Cpy(a, b) => format!("cpy {} {}", a, b),
                Instruction::Inc(reg) => format!("inc {}", reg_name(*reg)),
                Instruction::Dec(reg) => format!("dec {}", reg_name(*reg)),
                Instruction::Jnz(a, b) => format!("jnz {} {}", a, b),
                Instruction::Out(a) => format!("out {}", a),
                Instruction::Tgl(reg) => format!("tgl {}", reg_name(*reg)),
                Instruction::Nop => "nop".to_string(),
            }
        )
    }
}

/// a program, the registers, with loader and executor
pub struct BunnyVM {
    instructions: Vec<Instruction>,
    toggled: Vec<Instruction>,
    /// the registers `REG_A` to `REG_D`
    pub registers: [i32; 4],
    /// the instruction pointer
    pub ip: usize,
    /// the output of `out` instruction if applicable, otherwise `None`
    pub output: Option<i32>,
}

impl BunnyVM {
    /// `new` initializes a new program
    #[must_use]
    pub fn new(program: &str) -> BunnyVM {
        let mut p = BunnyVM {
            instructions: Vec::new(),
            toggled: Vec::new(),
            registers: [0; 4],
            ip: 0,
            output: None,
        };

        p.load(program);
        p
    }

    /// print the program
    pub fn print(&self) {
        for instruction in &self.instructions {
            println!("{}", instruction);
        }
    }

    /// print the program with registers state
    pub fn print_state(&self) {
        for (i, instruction) in self.instructions.iter().enumerate() {
            let current = if self.ip == i { "=>" } else { "  " };
            let instr = format!("{}", instruction);
            let reg = if i < 4 {
                format!("{}: {}", reg_name(i), self.registers[i])
            } else {
                "".to_owned()
            };
            println!("{:3}  {}:   {:20}   {}", current, i, instr, reg);
        }
    }

    /// `load` loads the program from a sequence of instructions
    fn load(&mut self, input: &str) {
        self.instructions = input
            .lines()
            .map(|line| line.split(';').next().unwrap().trim()) // remove comments
            .filter(|line| !line.is_empty()) // ignore empty lines
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
                    "out" => Instruction::Out(RegOrValue::from_str(source)),
                    "tgl" => Instruction::Tgl(to_reg(source)),
                    _ => panic!("Unknown instruction: {}", instruction),
                }
            })
            .collect();

        self.toggled = self
            .instructions
            .iter()
            .map(|instruction| match instruction {
                Instruction::Nop => Instruction::Nop,

                // For one-argument instructions, inc becomes dec, and all other one-argument instructions become inc.
                Instruction::Inc(reg) => Instruction::Dec(*reg),
                Instruction::Dec(reg) | Instruction::Tgl(reg) => Instruction::Inc(*reg),

                Instruction::Out(reg) => match reg {
                    RegOrValue::Register(reg) => Instruction::Out(RegOrValue::Register(*reg)),
                    RegOrValue::Value(_) => Instruction::Nop,
                },

                // For two-argument instructions, jnz becomes cpy, and all other two-instructions become jnz.
                Instruction::Jnz(a, b) => Instruction::Cpy(*a, *b),
                Instruction::Cpy(a, b) => Instruction::Jnz(*a, *b),
            })
            .collect();

        assert_eq!(self.instructions.len(), self.toggled.len());

        self.reset();
    }

    /// returns true if the program is finished (instruction pointer is out of bounds)
    #[must_use]
    pub fn is_terminated(&self) -> bool {
        self.ip >= self.instructions.len()
    }

    /// run one instruction and advance the instruction pointer
    /// set `output` if applicable
    /// return false if the program is finished
    pub fn step(&mut self) -> bool {
        self.output = None;

        if self.is_terminated() {
            return false;
        }

        match &self.instructions[self.ip] {
            Instruction::Nop => {}
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
                    self.ip = BunnyVM::new_ip(self.ip, offset);
                    return !self.is_terminated();
                }
            }
            Instruction::Out(RegOrValue::Register(reg)) => {
                let value = self.registers[*reg];
                self.output = Some(value);
            }
            Instruction::Out(RegOrValue::Value(value)) => {
                self.output = Some(*value);
            }
            Instruction::Tgl(reg) => {
                let ip = BunnyVM::new_ip(self.ip, self.registers[*reg]);

                if ip < self.instructions.len() {
                    std::mem::swap(&mut self.instructions[ip], &mut self.toggled[ip]);
                }
            }
        }

        self.ip += 1;

        !self.is_terminated()
    }

    /// reset the program to the initial state
    pub fn reset(&mut self) {
        self.registers = [0, 0, 0, 0];
        self.ip = 0;
        self.output = None;
    }

    /// `new_ip` returns the new instruction pointer after jumping `offset` instructions
    /// cannot underflow, can overflow (terminate the program or do not toggle instruction)
    fn new_ip(ip: usize, offset: i32) -> usize {
        if offset >= 0 {
            ip.checked_add(usize::try_from(offset).unwrap()).unwrap()
        } else {
            ip.checked_sub(usize::try_from(-offset).unwrap()).unwrap()
        }
    }

    /// run the program
    /// ignore the outputs (`out` instructions)
    /// return `true` if the program is finished or `false` if it takes too long
    pub fn run(&mut self, max_iterations: usize) -> bool {
        let mut iterations = max_iterations;

        while self.step() && iterations > 0 {
            iterations -= 1;
        }

        self.is_terminated()
    }

    /// run the program and return the output
    /// # Panics
    /// if output is not a u8
    pub fn run_output(&mut self, max_iterations: usize) -> String {
        let mut iterations = max_iterations;
        let mut output = String::new();

        while self.step() && iterations > 0 {
            if let Some(c) = self.output {
                if (0..256).contains(&c) {
                    output.push(char::from(u8::try_from(c).unwrap()));
                }
            }
            iterations -= 1;
        }

        output
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

    let mut program = BunnyVM::new(demo);

    assert!(program.run(100));
    assert_eq!(program.registers[REG_A], 42);
}

#[test]
fn test_jnz() {
    assert_eq!(BunnyVM::new_ip(10, 2), 12);
    assert_eq!(BunnyVM::new_ip(10, -2), 8);
}

#[test]
fn test_inc_a() {
    let mut program = BunnyVM::new("inc a");
    program.registers[REG_A] = 42;
    program.step();
    assert_eq!(program.registers[REG_A], 43);
}

#[test]
fn test_dec_b() {
    let mut program = BunnyVM::new("dec b");
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

    let mut program = BunnyVM::new(demo);

    let ok = program.run(100);
    assert!(ok);

    assert_eq!(program.registers[REG_A], 3);
}
