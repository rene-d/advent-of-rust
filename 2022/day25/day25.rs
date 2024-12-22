//! [Day 25: Full of Hot Air](https://adventofcode.com/2022/day/25)

fn from_snafu(s: &str) -> i64 {
    let mut n = 0;
    let mut pow = 1;
    for ch in s.chars().rev() {
        n += pow
            * match ch {
                '2' => 2,
                '1' => 1,
                '0' => 0,
                '-' => -1,
                '=' => -2,
                _ => panic!("unknown digit '{ch}'"),
            };
        pow *= 5;
    }
    n
}

fn to_snafu(mut n: i64) -> String {
    let mut digits = vec![];
    loop {
        let ch = ['=', '-', '0', '1', '2'][(n + 2).rem_euclid(5) as usize];
        digits.push(ch);
        n = (n + 2).div_euclid(5);
        if n == 0 {
            break;
        }
    }

    digits.iter().rev().collect::<String>()
}

struct Puzzle {
    sum_of_numbers: i64,
}

impl Puzzle {
    const fn new() -> Self {
        Self { sum_of_numbers: 0 }
    }

    /// Loads data from input (one line)
    fn configure(&mut self, path: &str) {
        let data = std::fs::read_to_string(path).unwrap();

        self.sum_of_numbers = data.lines().map(from_snafu).sum();
    }

    // Solves part one
    fn part1(&self) -> String {
        to_snafu(self.sum_of_numbers)
    }
}

/// main function
fn main() {
    let args = aoc::parse_args();
    let mut puzzle = Puzzle::new();
    puzzle.configure(&args.path);
    println!("{}", puzzle.part1());
}

#[test]
fn test01() {
    let mut puzzle = Puzzle::new();
    puzzle.configure("test.txt");
    assert_eq!(puzzle.sum_of_numbers, 4890);
    assert_eq!(puzzle.part1(), "2=-1=0");
}

#[test]
fn test_from_snafu() {
    assert_eq!(from_snafu("2=-01"), 976);
}

#[test]
fn test_to_snafu() {
    assert_eq!(to_snafu(976), "2=-01");
}
