//! [Day 1: Calorie Counting](https://adventofcode.com/2022/day/1)

struct Puzzle {
    calories: Vec<usize>,
}

impl Puzzle {
    const fn new() -> Self {
        Self {
            calories: Vec::new(),
        }
    }

    fn configure(&mut self, data: &str) {
        self.calories = data
            .trim_ascii()
            .split("\n\n")
            .map(|x| {
                x.split('\n')
                    .map(|y| y.parse::<usize>().unwrap())
                    .sum::<usize>()
            })
            .collect::<Vec<_>>();

        // Reverse sort to have to most significant values first
        self.calories.sort_by(|a, b| b.cmp(a));
    }

    fn part1(&self) -> usize {
        self.calories[0]
    }

    fn part2(&self) -> usize {
        self.calories[0..3].iter().sum::<usize>()
    }
}

fn main() {
    let args = aoc::parse_args();
    let mut puzzle = Puzzle::new();
    puzzle.configure(&args.input);
    println!("{}", puzzle.part1());
    println!("{}", puzzle.part2());
}

/// Test from puzzle input
#[test]
fn test01() {
    let mut puzzle = Puzzle::new();
    puzzle.configure(&aoc::load_input_data("test.txt"));
    assert_eq!(puzzle.part1(), 24000);
    assert_eq!(puzzle.part2(), 45000);
}
