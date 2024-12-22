//! [Day 5: Supply Stacks](https://adventofcode.com/2022/day/5)
use regex::Regex;

#[derive(PartialEq)]
enum State {
    Stacks,
    Moves,
}

#[derive(Debug)]
struct Move {
    crate_count: usize,
    stack_from: usize,
    stack_to: usize,
}

#[derive(Debug)]
struct Puzzle {
    stacks: Vec<String>,
    moves: Vec<Move>,
}

impl Puzzle {
    fn new() -> Self {
        Self {
            stacks: vec![String::new(); 9],
            moves: Vec::new(),
        }
    }

    /// load data from input
    fn configure(&mut self, path: &str) {
        let data = std::fs::read_to_string(path).unwrap();

        let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();

        let mut state = State::Stacks;

        for line in data.split('\n') {
            if line.is_empty() {
                state = State::Moves;
            } else if state == State::Stacks {
                let mut p: usize = 0;

                while p * 4 + 1 < line.len() {
                    let crate_id = line.chars().nth(p * 4 + 1).unwrap();
                    if crate_id.is_ascii_uppercase() {
                        self.stacks[p].push(crate_id);
                    }
                    p += 1;
                }
            } else if let Some(m) = re.captures(line) {
                let mov = Move {
                    crate_count: m[1].parse::<usize>().unwrap(),
                    stack_from: m[2].parse::<usize>().unwrap(),
                    stack_to: m[3].parse::<usize>().unwrap(),
                };
                self.moves.push(mov);
            }
        }
    }

    /// solves part1
    fn part1(&self) -> String {
        let mut stacks = self.stacks.clone();

        for m in &self.moves {
            for _ in 0..m.crate_count {
                let crate_id = stacks[m.stack_from - 1].remove(0);
                stacks[m.stack_to - 1].insert(0, crate_id);
            }
        }

        Self::top(&stacks)
    }

    /// solves part2
    fn part2(&self) -> String {
        let mut stacks = self.stacks.clone();

        for m in &self.moves {
            for i in 0..m.crate_count {
                let crate_id = stacks[m.stack_from - 1].remove(m.crate_count - i - 1);
                stacks[m.stack_to - 1].insert(0, crate_id);
            }
        }

        Self::top(&stacks)
    }

    /// returns the top of the stacks
    fn top(stacks: &[String]) -> String {
        let mut top = String::new();
        for stack in stacks {
            if let Some(crate_id) = stack.chars().next() {
                top.push(crate_id);
            }
        }

        top
    }
}

fn main() {
    let args = aoc::parse_args();
    let mut puzzle = Puzzle::new();
    puzzle.configure(args.path.as_str());
    println!("{}", puzzle.part1());
    println!("{}", puzzle.part2());
}

#[test]
fn test_puzzle() {
    let mut puzzle = Puzzle::new();

    puzzle.configure("test.txt");

    assert_eq!(puzzle.part1(), "CMZ");
    assert_eq!(puzzle.part2(), "MCD");
}
