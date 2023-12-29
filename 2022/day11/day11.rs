//! [Day 11: Monkey in the Middle](https://adventofcode.com/2022/day/11)

use clap::Parser;

#[derive(Parser)]
struct Args {
    /// Puzzle input
    #[arg(default_value = "input.txt")]
    path: String,
}

#[derive(Copy, Clone, Debug)]
enum Operation {
    Square,
    Addition(u64),
    Product(u64),
}

impl Operation {
    fn new(s: &str) -> Self {
        if s == "old * old" {
            return Operation::Square;
        } else if let Some(m) = s.strip_prefix("old + ") {
            return Operation::Addition(m.parse().unwrap());
        } else if let Some(m) = s.strip_prefix("old * ") {
            return Operation::Product(m.parse().unwrap());
        }
        panic!("bad operation {s}")
    }

    fn calc(&self, arg: u64) -> u64 {
        match self {
            Operation::Square => arg * arg,
            Operation::Addition(n) => arg + n,
            Operation::Product(n) => arg * n,
        }
    }
}

#[derive(Clone, Debug)]
struct Monkey {
    inspections: usize,
    items: Vec<u64>,
    operation: Operation,
    divisible_by: u64,
    if_true: usize,
    if_false: usize,
}

struct Puzzle {
    monkeys: Vec<Monkey>,
}

impl Puzzle {
    fn new() -> Self {
        Self { monkeys: vec![] }
    }

    /// Loads data from input (one line)
    fn configure(&mut self, path: &str) {
        let data = std::fs::read_to_string(path).unwrap();
        let mut lines = data.split('\n');

        // Nota: monkey definitions always start at id 0

        while lines.next().is_some() {
            let mut monkey = Monkey {
                inspections: 0,
                items: vec![],
                operation: Operation::Square,
                divisible_by: 0,
                if_true: 0,
                if_false: 0,
            };

            monkey.items = lines
                .next()
                .unwrap()
                .strip_prefix("  Starting items: ")
                .unwrap()
                .split(',')
                .map(|x| x.trim().parse::<u64>().unwrap())
                .collect::<Vec<u64>>();

            monkey.operation = Operation::new(
                lines
                    .next()
                    .unwrap()
                    .strip_prefix("  Operation: new = ")
                    .unwrap(),
            );

            monkey.divisible_by = lines
                .next()
                .unwrap()
                .strip_prefix("  Test: divisible by ")
                .unwrap()
                .parse::<_>()
                .unwrap();

            monkey.if_true = lines
                .next()
                .unwrap()
                .strip_prefix("    If true: throw to monkey ")
                .unwrap()
                .parse::<_>()
                .unwrap();

            monkey.if_false = lines
                .next()
                .unwrap()
                .strip_prefix("    If false: throw to monkey ")
                .unwrap()
                .parse::<_>()
                .unwrap();

            lines.next().unwrap(); // skip the empty line after monkey definition

            self.monkeys.push(monkey);
        }
    }

    /// Solves part one
    fn part1(&self) -> usize {
        self.solve(20)
    }

    /// Solve part two
    fn part2(&self) -> usize {
        self.solve(10000)
    }

    /// Solve the puzzle
    fn solve(&self, rounds: usize) -> usize {
        // we need to clone the monkeys array since we modify the starting item lists
        let mut monkeys = self.monkeys.clone();

        let monkey_count = monkeys.len();
        let modulus: u64 = monkeys.iter().map(|x| x.divisible_by).product();

        for _ in 0..rounds {
            for id in 0..monkey_count {
                // I don't know how to safely borrow monkeys[id]
                let monkey = monkeys[id].clone();

                monkeys[id].items.clear();
                monkeys[id].inspections += monkey.items.len();

                for item in monkey.items {
                    let mut worry_level = monkey.operation.calc(item);

                    if rounds == 20 {
                        worry_level /= 3; // part 1
                    } else {
                        worry_level %= modulus; // part 2
                    }

                    let thrown_to = match worry_level % monkey.divisible_by {
                        0 => monkey.if_true,
                        _ => monkey.if_false,
                    };
                    monkeys[thrown_to].items.push(worry_level);
                }
            }
        }

        // compute the level of monkey business
        let mut monkey_business = monkeys
            .iter()
            .map(|x| x.inspections)
            .collect::<Vec<usize>>();
        monkey_business.sort_by(|a, b| b.cmp(a));
        monkey_business[0] * monkey_business[1]
    }
}

/// main function
fn main() {
    let args = Args::parse();
    let mut puzzle = Puzzle::new();
    puzzle.configure(&args.path);
    println!("{}", puzzle.part1());
    println!("{}", puzzle.part2());
}

#[test]
fn test01() {
    let mut puzzle = Puzzle::new();
    puzzle.configure("test.txt");
    assert_eq!(puzzle.part1(), 10605);
    assert_eq!(puzzle.part2(), 2_713_310_158);
}
