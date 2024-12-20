//! [Day 2: 1202 Program Alarm](https://adventofcode.com/2019/day/2)

struct Puzzle {
    program: Vec<u32>,
}

const ADD: u32 = 1;
const MUL: u32 = 2;

impl Puzzle {
    fn new() -> Puzzle {
        Puzzle {
            program: Vec::new(),
        }
    }

    /// Get the puzzle input.
    fn configure(&mut self, path: &str) {
        let data = std::fs::read_to_string(path).unwrap();

        self.load(&data);
    }

    fn load(&mut self, data: &str) {
        self.program = data
            .trim_ascii()
            .split(',')
            .map(|x| x.parse::<_>().unwrap())
            .collect();
    }

    #[cfg(test)]
    fn dump(&self) -> String {
        self.program
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(",")
    }

    fn run(memory: &mut [u32]) {
        let mut ip = 0;
        while memory[ip] != 99 {
            assert!(ip + 3 < memory.len());

            let opcode = memory[ip];
            let noun = usize::try_from(memory[ip + 1]).unwrap();
            let verb = usize::try_from(memory[ip + 2]).unwrap();
            let result = usize::try_from(memory[ip + 3]).unwrap();

            memory[result] = match opcode {
                ADD => memory[noun] + memory[verb],
                MUL => memory[noun] * memory[verb],
                _ => panic!("Invalid opcode"),
            };
            ip += 4;
        }
    }

    /// Solve part one.
    fn part1(&self) -> u32 {
        let mut memory = self.program.clone();
        memory[1] = 12;
        memory[2] = 2;

        Self::run(&mut memory);

        memory[0]
    }

    /// Solve part two.
    fn part2(&self) -> u32 {
        for noun in 0..100 {
            for verb in 0..100 {
                let mut memory = self.program.clone();
                memory[1] = noun;
                memory[2] = verb;

                Self::run(&mut memory);

                // clippy...
                if memory[0] == 19_690_720 {
                    return 100 * noun + verb;
                }
            }
        }
        0
    }
}

fn main() {
    let args = aoc::parse_args();
    let mut puzzle = Puzzle::new();
    puzzle.configure(args.path.as_str());
    println!("{}", puzzle.part1());
    println!("{}", puzzle.part2());
}

/// Test from puzzle input
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test01() {
        let mut puzzle = Puzzle::new();

        puzzle.load("1,0,0,0,99");
        Puzzle::run(&mut puzzle.program);
        assert_eq!(puzzle.dump(), "2,0,0,0,99");

        puzzle.load("2,3,0,3,99");
        Puzzle::run(&mut puzzle.program);
        assert_eq!(puzzle.dump(), "2,3,0,6,99");
        puzzle.load("2,4,4,5,99,0");
        Puzzle::run(&mut puzzle.program);
        assert_eq!(puzzle.dump(), "2,4,4,5,99,9801");

        puzzle.load("1,1,1,4,99,5,6,0,99");
        Puzzle::run(&mut puzzle.program);
        assert_eq!(puzzle.dump(), "30,1,1,4,2,5,6,0,99");
    }
}
