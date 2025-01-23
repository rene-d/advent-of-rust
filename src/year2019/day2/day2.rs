//! [Day 2: 1202 Program Alarm](https://adventofcode.com/2019/day/2)

struct Puzzle {
    program: Vec<u32>,
}

const ADD: u32 = 1;
const MUL: u32 = 2;

impl Puzzle {
    fn new(data: &str) -> Self {
        Self {
            program: data
                .trim_ascii()
                .split(',')
                .map(|x| x.parse::<_>().unwrap())
                .collect(),
        }
    }

    #[cfg(test)]
    fn dump(&self) -> String {
        self.program
            .iter()
            .map(std::string::ToString::to_string)
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

/// # Panics
/// over malformed input
#[must_use]
pub fn solve(data: &str) -> (u32, u32) {
    let puzzle = Puzzle::new(data);
    (puzzle.part1(), puzzle.part2())
}

pub fn main() {
    let args = aoc::parse_args();
    args.run(solve);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test01() {
        let mut puzzle = Puzzle::new("1,0,0,0,99");
        Puzzle::run(&mut puzzle.program);
        assert_eq!(puzzle.dump(), "2,0,0,0,99");

        let mut puzzle = Puzzle::new("2,3,0,3,99");
        Puzzle::run(&mut puzzle.program);
        assert_eq!(puzzle.dump(), "2,3,0,6,99");
        let mut puzzle = Puzzle::new("2,4,4,5,99,0");
        Puzzle::run(&mut puzzle.program);
        assert_eq!(puzzle.dump(), "2,4,4,5,99,9801");

        let mut puzzle = Puzzle::new("1,1,1,4,99,5,6,0,99");
        Puzzle::run(&mut puzzle.program);
        assert_eq!(puzzle.dump(), "30,1,1,4,2,5,6,0,99");
    }
}
