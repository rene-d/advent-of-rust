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

    /// # Panics
    pub fn run(&mut self) -> State {
        loop {
            let opcode = self.peek(Address(self.ip));

            match opcode % 100 {
                0 => {
                    // nop (extension)
                    self.ip += 1;
                }
                99 => break State::Halted,

                1 => {
                    // addition
                    let a = self.address((opcode / 100) % 10, 1);
                    let b = self.address((opcode / 1000) % 10, 2);
                    let c = self.address((opcode / 10000) % 10, 3);
                    self.poke(c, self.peek(a) + self.peek(b));
                    self.ip += 4;
                }

                2 => {
                    // multiplication
                    let a = self.address((opcode / 100) % 10, 1);
                    let b = self.address((opcode / 1000) % 10, 2);
                    let c = self.address((opcode / 10000) % 10, 3);
                    self.poke(c, self.peek(a) * self.peek(b));
                    self.ip += 4;
                }

                3 => {
                    // input
                    if let Some(value) = self.input.pop_front() {
                        // println!("got input {value}");

                        let a = self.address((opcode / 100) % 10, 1);
                        self.poke(a, value);
                        self.ip += 2;
                    } else {
                        // println!("waiting for input");
                        break State::Input;
                    }
                }

                4 => {
                    // output
                    let a = self.address((opcode / 100) % 10, 1);
                    let value = self.peek(a);
                    self.ip += 2;

                    // println!("output {value}");
                    break State::Output(value);
                }

                5 => {
                    // jump-if-true

                    let a = self.address((opcode / 100) % 10, 1);

                    self.ip = if self.peek(a) == 0 {
                        self.ip + 3
                    } else {
                        let b = self.address((opcode / 1000) % 10, 2);
                        self.peek(b)
                    }
                }

                6 => {
                    // jump-if-false

                    let a = self.address((opcode / 100) % 10, 1);

                    self.ip = if self.peek(a) != 0 {
                        self.ip + 3
                    } else {
                        let b = self.address((opcode / 1000) % 10, 2);
                        self.peek(b)
                    }
                }

                7 => {
                    let a = self.address((opcode / 100) % 10, 1);
                    let b = self.address((opcode / 1000) % 10, 2);
                    let c = self.address((opcode / 10000) % 10, 3);

                    self.poke(c, i64::from(self.peek(a) < self.peek(b)));

                    self.ip += 4;
                }

                8 => {
                    let a = self.address((opcode / 100) % 10, 1);
                    let b = self.address((opcode / 1000) % 10, 2);
                    let c = self.address((opcode / 10000) % 10, 3);

                    self.poke(c, i64::from(self.peek(a) == self.peek(b)));

                    self.ip += 4;
                }

                _ => panic!("opcode {opcode} not implemented"),
            }
        }
    }

    fn peek(&self, address: Address) -> i64 {
        usize::try_from(address.0).map_or_else(
            |_| {
                panic!("segmentation fault at {}", address.0);
            },
            |a| self.mem[a],
        )
    }

    fn poke(&mut self, address: Address, num: i64) {
        if let Ok(a) = usize::try_from(address.0) {
            self.mem[a] = num;
        } else {
            panic!("segmentation fault at {}", address.0);
        }
    }

    fn address(&self, mode: i64, offset: i64) -> Address {
        match mode {
            POSITION_MODE => Address(self.peek(Address(self.ip + offset))),
            IMMEDIATE_MODE => Address(self.ip + offset),
            RELATIVE_MODE => Address(self.relbase + self.peek(Address(self.ip + offset))),
            _ => panic!("invalid mode {mode}"),
        }
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
