use std::collections::VecDeque;

// addressint mode
const POSITION_MODE: i64 = 0; // https://adventofcode.com/2019/day/5
const IMMEDIATE_MODE: i64 = 1; // https://adventofcode.com/2019/day/5
const RELATIVE_MODE: i64 = 2; // https://adventofcode.com/2019/day/9

#[derive(Copy, Clone)]
struct Address(i64);

#[derive(Debug, PartialEq, Eq)]
pub enum State {
    Halted,
    Input,
    Output(i64),
}

#[derive(Clone)]
pub struct Computer {
    program: Vec<i64>,
    mem: Vec<i64>,
    ip: i64,
    relbase: i64,
    input: VecDeque<i64>,
}

impl Computer {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            program: Vec::new(),
            mem: Vec::new(),
            ip: 0,
            relbase: 0,
            input: VecDeque::new(),
        }
    }
}

impl Default for Computer {
    fn default() -> Self {
        Self::new()
    }
}

impl Computer {
    #[must_use]
    pub fn load(data: &str) -> Self {
        let mut computer = Self::new();

        for line in data.lines() {
            let mut line = line.trim_ascii();

            // remove "[nnn]  " at the beginning
            if line.starts_with('[') {
                if let Some(p) = line.find(']') {
                    //
                    line = line[(p + 1)..].trim_ascii_start();
                } else {
                    continue;
                }
            }

            for num in line.split(',') {
                if let Ok(num) = num.trim().parse::<i64>() {
                    computer.program.push(num);
                } else {
                    break;
                }
            }
        }

        computer.mem = computer.program.clone();

        computer
    }
}

impl std::fmt::Display for Computer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for num in &self.program {
            write!(f, "{num},")?;
        }
        Ok(())
    }
}

impl Computer {
    pub fn reset(&mut self) {
        self.mem = self.program.clone();
        self.ip = 0;
        self.relbase = 0;
        self.input.clear();
    }

    pub fn push(&mut self, value: i64) {
        self.input.push_back(value);
    }

    pub fn push_byte(&mut self, byte: u8) {
        self.input.push_back(i64::from(byte));
    }

    pub fn push_ascii(&mut self, string: &str) {
        string.bytes().for_each(|b| self.push_byte(b));
    }

    /// # Panics
    pub fn run(&mut self) -> State {
        loop {
            let opcode = self.addr_peek(Address(self.ip));

            match opcode % 100 {
                0 => {
                    // nop (extension)
                    self.ip += 1;
                }
                99 => break State::Halted,

                1 => {
                    // addition
                    let a = self.op_address(opcode, 1);
                    let b = self.op_address(opcode, 2);
                    let c = self.op_address(opcode, 3);
                    self.addr_poke(c, self.addr_peek(a) + self.addr_peek(b));
                    self.ip += 4;
                }

                2 => {
                    // multiplication
                    let a = self.op_address(opcode, 1);
                    let b = self.op_address(opcode, 2);
                    let c = self.op_address(opcode, 3);
                    self.addr_poke(c, self.addr_peek(a) * self.addr_peek(b));
                    self.ip += 4;
                }

                3 => {
                    // input
                    if let Some(value) = self.input.pop_front() {
                        // println!("got input {value}");

                        let a = self.op_address(opcode, 1);
                        self.addr_poke(a, value);
                        self.ip += 2;
                    } else {
                        // println!("waiting for input");
                        break State::Input;
                    }
                }

                4 => {
                    // output
                    let a = self.op_address(opcode, 1);
                    let value = self.addr_peek(a);
                    self.ip += 2;

                    // println!("output {value}");
                    break State::Output(value);
                }

                5 => {
                    // jump-if-true

                    let a = self.op_address(opcode, 1);

                    self.ip = if self.addr_peek(a) == 0 {
                        self.ip + 3
                    } else {
                        let b = self.op_address(opcode, 2);
                        self.addr_peek(b)
                    }
                }

                6 => {
                    // jump-if-false

                    let a = self.op_address(opcode, 1);

                    self.ip = if self.addr_peek(a) != 0 {
                        self.ip + 3
                    } else {
                        let b = self.op_address(opcode, 2);
                        self.addr_peek(b)
                    }
                }

                7 => {
                    let a = self.op_address(opcode, 1);
                    let b = self.op_address(opcode, 2);
                    let c = self.op_address(opcode, 3);

                    self.addr_poke(c, i64::from(self.addr_peek(a) < self.addr_peek(b)));

                    self.ip += 4;
                }

                8 => {
                    let a = self.op_address(opcode, 1);
                    let b = self.op_address(opcode, 2);
                    let c = self.op_address(opcode, 3);

                    self.addr_poke(c, i64::from(self.addr_peek(a) == self.addr_peek(b)));

                    self.ip += 4;
                }

                9 => {
                    let a = self.op_address(opcode, 1);
                    self.relbase += self.addr_peek(a);
                    self.ip += 2;
                }

                _ => panic!("opcode {opcode} not implemented"),
            }
        }
    }

    /// Calculate the operand address.
    fn op_address(&self, opcode: i64, offset: i64) -> Address {
        let mode = match offset {
            1 => (opcode / 100) % 10,
            2 => (opcode / 1000) % 10,
            3 => (opcode / 10000) % 10,
            _ => panic!(),
        };
        match mode {
            POSITION_MODE => Address(self.addr_peek(Address(self.ip + offset))),
            IMMEDIATE_MODE => Address(self.ip + offset),
            RELATIVE_MODE => Address(self.relbase + self.addr_peek(Address(self.ip + offset))),
            _ => panic!("invalid mode {mode}"),
        }
    }

    fn addr_peek(&self, address: Address) -> i64 {
        usize::try_from(address.0).map_or_else(
            |_| {
                panic!("segmentation fault at {}", address.0);
            },
            |a| *self.mem.get(a).unwrap_or(&0),
        )
    }

    fn addr_poke(&mut self, address: Address, num: i64) {
        if let Ok(a) = usize::try_from(address.0) {
            if a >= self.mem.len() {
                // allocate a new 16-int page
                self.mem.resize(a + 16, 0);
            }

            self.mem[a] = num;
        } else {
            panic!("segmentation fault at {}", address.0);
        }
    }

    /// To cheat.
    pub fn poke(&mut self, address: i64, value: i64) {
        self.addr_poke(Address(address), value);
    }
}

/// Test from puzzle input
#[cfg(test)]
mod test {
    use super::*;

    /// Tests from day5
    #[test]
    fn test_intcode() {
        let mut program = Computer::load(include_str!("day5_compare.intcode"));

        program.push(7);
        assert_eq!(program.run(), State::Output(999));

        program.reset();
        program.push(8);
        assert_eq!(program.run(), State::Output(1000));

        program.reset();
        program.push(9);
        assert_eq!(program.run(), State::Output(1001));
    }
}
