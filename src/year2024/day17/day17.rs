//! [Day 17: Chronospatial Computer](https://adventofcode.com/2024/day/17)

struct Puzzle {
    reg_a: u32,
    reg_b: u32,
    reg_c: u32,
    program: Vec<u32>,
}

impl Puzzle {
    fn new(data: &str) -> Self {
        let mut program = Vec::new();
        let mut reg_a = 0;
        let mut reg_b = 0;
        let mut reg_c = 0;

        for line in data.lines() {
            if let Some(v) = line.strip_prefix("Register A: ") {
                reg_a = v.parse().unwrap();
            } else if let Some(v) = line.strip_prefix("Register B: ") {
                reg_b = v.parse().unwrap();
            } else if let Some(v) = line.strip_prefix("Register C: ") {
                reg_c = v.parse().unwrap();
            } else if let Some(v) = line.strip_prefix("Program: ") {
                program = v.split(',').filter_map(|i| i.parse().ok()).collect();
            }
        }

        Self {
            reg_a,
            reg_b,
            reg_c,
            program,
        }
    }

    fn run(&self, mut a: u32, mut b: u32, mut c: u32) -> Vec<u32> {
        let mut ip = 0;
        let mut output = Vec::new();

        while ip < self.program.len() - 1 {
            let opcode = self.program[ip];
            let literal = self.program[ip + 1];

            let combo = || match literal {
                0..=3 => literal,
                4 => a,
                5 => b,
                6 => c,
                _ => panic!(),
            };

            match opcode {
                0 => {
                    // adv
                    a >>= literal;
                }
                1 => {
                    //bxl
                    b ^= literal;
                }
                2 => {
                    // bst
                    b = combo() % 8;
                }
                3 => {
                    // jnz
                    if a != 0 {
                        ip = usize::try_from(literal).unwrap();
                        continue;
                    }
                }
                4 => {
                    // bxc
                    b ^= c;
                }
                5 => {
                    // out
                    output.push(combo() % 8);
                }
                6 => {
                    /* bdv */
                    b = a >> combo();
                }
                7 => {
                    // cdv
                    c = a >> combo();
                }
                _ => panic!(),
            };

            ip += 2;
        }

        output
    }

    fn dump(&self) {
        for ip in (0..self.program.len() - 1).step_by(2) {
            let opcode = self.program[ip];
            let literal = self.program[ip + 1];

            let combo = match literal {
                0 => '0',
                1 => '1',
                2 => '2',
                3 => '3',
                4 => 'a',
                5 => 'b',
                6 => 'c',
                _ => '?', // should never be printed
            };

            let pow2_literal = 1 << literal;

            let (o, c) = match opcode {
                0 => (format!("adv {literal}"), format!("a = a / {pow2_literal}")),
                1 => (format!("bxl {literal}"), format!("b ^= {literal}")),
                2 => (format!("bst {combo}"), format!("b = {combo} % 8")),
                3 => (format!("jnz {literal}"), format!("jump {literal} if a≠0")),
                4 => ("bxc".to_string(), "b ^= c".to_string()),
                5 => (format!("out {combo}"), format!("out {combo} % 8")),
                6 => (format!("bdv {combo}"), format!("b = a >> {combo}")),
                7 => (format!("cdv {combo}"), format!("c = a >> {combo}")),
                _ => panic!(),
            };

            println!("{ip:3}:  {opcode} {literal}   {o:<10} ; {c}");
        }
    }

    /// Solve part one.
    fn part1(&self) -> String {
        let output = self.run(self.reg_a, self.reg_b, self.reg_c);

        output
            .iter()
            .map(std::string::ToString::to_string)
            .collect::<Vec<String>>()
            .join(",")
    }

    // Programs are always like (some instructions can be inverted):
    //     b = a % 8
    //     b ^= 5              <== first variant xor
    //     c = a >> b
    //     b ^= 6              <== second variant xor
    //     b ^= c
    //     out b % 8
    //     a = a / 8
    //     jump 0 if a≠0

    fn quine(&self, a: u64, i: usize, xor1: u64, xor2: u64) -> u64 {
        let target = u64::from(self.program[i]);

        let start_octal = u64::from(i == self.program.len() - 1);

        for octal in start_octal..8 {
            let new_a = (a * 8) | octal;
            let b = octal ^ xor1;
            let c = new_a >> b;
            let b = b ^ xor2;
            let b = b ^ c;
            if b % 8 == target {
                if i == 0 {
                    return new_a;
                }
                let new_a = self.quine(new_a, i - 1, xor1, xor2);
                if new_a != u64::MAX {
                    return new_a;
                }
            }
        }
        u64::MAX
    }

    /// Solve part two.
    fn part2(&self) -> u64 {
        let xors = self
            .program
            .chunks(2)
            .filter(|instr| instr[0] == 1)
            .map(|instr| instr[1])
            .collect::<Vec<_>>();

        if xors.len() != 2 {
            return 0;
        }

        let xor_1 = u64::from(xors[0]);
        let xor_2 = u64::from(xors[1]);

        self.quine(0, self.program.len() - 1, xor_1, xor_2)
    }
}

/// # Panics
#[must_use]
pub fn solve(data: &str) -> (String, u64) {
    let puzzle = Puzzle::new(data);
    (puzzle.part1(), puzzle.part2())
}

pub fn main() {
    let args = aoc::parse_args();

    if args.verbose {
        let puzzle = Puzzle::new(&args.input);
        puzzle.dump();
        return;
    }

    args.run(solve);
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE_1: &str = include_str!("sample_1.txt");
    const SAMPLE_2: &str = include_str!("sample_2.txt");

    #[test]
    fn test01() {
        let puzzle = Puzzle::new(SAMPLE_1);
        assert_eq!(puzzle.part1(), "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    fn test02() {
        let mut puzzle = Puzzle::new(SAMPLE_2);
        puzzle.reg_a = 117440;
        assert_eq!(puzzle.part1(), "0,3,5,4,3,0");
    }
}
