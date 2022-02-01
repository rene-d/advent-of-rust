/*!
[Day 12: Leonardo's Monorail](https://adventofcode.com/2016/day/12)


You finally reach the top floor of this building: a garden with a slanted glass ceiling. Looks like there are no more stars to be had.

While sitting on a nearby bench amidst some [tiger lilies](https://www.google.com/search?q=tiger+lilies&tbm=isch), you manage to decrypt some of the files you extracted from the servers downstairs.

According to these documents, Easter Bunny HQ isn't just this building - it's a collection of buildings in the nearby area. They're all connected by a local monorail, and there's another building not far from here! Unfortunately, being night, the monorail is currently not operating.

You remotely connect to the monorail control systems and discover that the boot sequence expects a password. The password-checking logic (your puzzle input) is easy to extract, but the code it uses is strange: it's assembunny code designed for the [new computer](https://adventofcode.com/2016/day/11) you just assembled. You'll have to execute the code and get the password.

The assembunny code you've extracted operates on four [registers](https://en.wikipedia.org/wiki/Processor_register) (`a`, `b`, `c`, and `d`) that start at `0` and can hold any [integer](https://en.wikipedia.org/wiki/Integer). However, it seems to make use of only a few [instructions](https://en.wikipedia.org/wiki/Instruction_set):

- `cpy x y` **copies** `x` (either an integer or the **value** of a register) into register `y`.
- `inc x` **increases** the value of register `x` by one.
- `dec x` **decreases** the value of register `x` by one.
- `jnz x y` **jumps** to an instruction `y` away (positive means forward; negative means backward), but only if `x` is **not zero**.

The `jnz` instruction moves relative to itself: an offset of `-1` would continue at the previous instruction, while an offset of `2` would **skip over** the next instruction.

After executing the assembunny code in your puzzle input, **what value is left in register `a`**?

--- Part Two ---

As you head down the fire escape to the monorail, you notice it didn't start; register `c` needs to be initialized to the position of the ignition key.

If you instead **initialize register `c` to be `1`**, what value is now left in register `a`?
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
}

/// a program, the registers, with loader and executor
struct Program {
    instructions: Vec<Instruction>,
    registers: [i32; 4],
}

impl Program {
    /// `new` initializes a new program
    fn new() -> Program {
        Program {
            instructions: Vec::new(),
            registers: [0; 4],
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
                        if source == "1" {
                            Instruction::Jmp(offset)
                        } else {
                            Instruction::Jnz(to_reg(source), offset)
                        }
                    }
                    _ => panic!("Unknown instruction: {}", instruction),
                }
            })
            .collect();
    }

    /// run the program and returns the value of register `a`
    fn run(&mut self, c: i32) -> i32 {
        self.registers = [0, 0, 0, 0];

        self.registers[to_reg("c")] = c;

        let mut ip = 0;

        while ip < self.instructions.len() {

            match &self.instructions[ip] {
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
                        ip = Program::jump(ip, *offset);
                        continue;
                    }
                }
                Instruction::Jmp(offset) => {
                    ip = Program::jump(ip, *offset);
                    continue;
                }
            }
            ip += 1;
        }

        self.registers[to_reg("a")]
    }

    /// `jump` returns the new instruction pointer after jumping `offset` instructions
    fn jump(ip: usize, offset: i32) -> usize {
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

    println!("{}", program.run(0));
    println!("{}", program.run(1));
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
    assert_eq!(program.run(0), 42);
}

#[test]
fn test_jnz() {
    assert_eq!(Program::jump(10, 2), 12);
    assert_eq!(Program::jump(10, -2), 8);
}
