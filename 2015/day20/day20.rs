//! [Day 20: Infinite Elves and Infinite Houses](https://adventofcode.com/2015/day/20)

struct Puzzle {
    house_present: usize,
}

impl Puzzle {
    const fn new() -> Self {
        Self { house_present: 0 }
    }

    fn configure(&mut self, data: &str) {
        self.house_present = data.trim_ascii().parse::<usize>().unwrap();
    }

    fn part1(&self) -> usize {
        let mut house_number: usize = 0;
        let mut present_count: usize = 0;
        while present_count < self.house_present {
            house_number += 1;

            present_count = 10 * divisors::get_divisors(house_number).iter().sum::<usize>();

            // The crate `divisors` forgot to include 1 as a divisor. It also forgot to include
            // the number itself, except for 2. For instance:
            // - `divisors::get_divisors(1)` returns { }
            // - `divisors::get_divisors(2)` returns { 2 }
            // - `divisors::get_divisors(3)` returns { }
            // - `divisors::get_divisors(10)` returns { 2, 5 }
            if house_number == 1 || house_number == 2 {
                present_count += 10;
            } else {
                present_count += 10 * (1 + house_number);
            }
        }

        house_number
    }

    fn part2(&self) -> usize {
        let mut house_number: usize = 0;
        let mut present_count: usize = 0;
        while present_count < self.house_present {
            house_number += 1;

            present_count = 11
                * divisors::get_divisors(house_number)
                    .iter()
                    .filter(|x| 50 * **x >= house_number)
                    .sum::<usize>();

            // The crate `divisors` forgot to include 1 as a divisor. It also forgot to include
            // the number itself, except for 2. For instance:
            // - `divisors::get_divisors(1)` returns { }
            // - `divisors::get_divisors(2)` returns { 2 }
            // - `divisors::get_divisors(3)` returns { }
            // - `divisors::get_divisors(10)` returns { 2, 5 }
            if house_number == 1 || house_number == 2 {
                present_count += 11;
            } else {
                if house_number <= 50 {
                    present_count += 11;
                }
                present_count += 11 * house_number;
            }
        }

        house_number
    }
}

fn main() {
    let args = aoc::parse_args();

    let mut puzzle = Puzzle::new();

    puzzle.configure(&args.input);

    let result = puzzle.part1();
    println!("{result}");

    let result = puzzle.part2();
    println!("{result}");
}

/// Test from puzzle input
#[test]
fn test01() {
    let mut puzzle = Puzzle::new();
    puzzle.house_present = 10;
    assert_eq!(puzzle.part1(), 1);
    assert_eq!(puzzle.part2(), 1);
}

/// Test from puzzle input
#[test]
fn test02() {
    let mut puzzle = Puzzle::new();
    puzzle.house_present = 30;
    assert_eq!(puzzle.part1(), 2);
    assert_eq!(puzzle.part2(), 2);
}

/// Test from puzzle input
#[test]
fn test03() {
    let mut puzzle = Puzzle::new();
    puzzle.house_present = 40;
    assert_eq!(puzzle.part1(), 3);
    assert_eq!(puzzle.part2(), 3);
}

/// Test from puzzle input
#[test]
fn test04() {
    let mut puzzle = Puzzle::new();
    puzzle.house_present = 60;
    assert_eq!(puzzle.part1(), 4);
    assert_eq!(puzzle.part2(), 4);
}

/// Test from puzzle input
#[test]
fn test05() {
    let mut puzzle = Puzzle::new();
    puzzle.house_present = 70;
    assert_eq!(puzzle.part1(), 4);
    assert_eq!(puzzle.part2(), 4);
}

/// Test from puzzle input
#[test]
fn test06() {
    let mut puzzle = Puzzle::new();
    puzzle.house_present = 80;
    assert_eq!(puzzle.part1(), 6);
    assert_eq!(puzzle.part2(), 6);
}

/// Test from puzzle input
#[test]
fn test07() {
    let mut puzzle = Puzzle::new();
    puzzle.house_present = 120;
    assert_eq!(puzzle.part1(), 6);
    assert_eq!(puzzle.part2(), 6);
}

/// Test from puzzle input
#[test]
fn test08() {
    let mut puzzle = Puzzle::new();
    puzzle.house_present = 130;
    assert_eq!(puzzle.part1(), 8);
    assert_eq!(puzzle.part2(), 6);
}

/// Test from puzzle input
#[test]
fn test09() {
    let mut puzzle = Puzzle::new();
    puzzle.house_present = 150;
    assert_eq!(puzzle.part1(), 8);
    assert_eq!(puzzle.part2(), 8);
}
