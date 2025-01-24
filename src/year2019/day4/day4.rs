//! [Day 4: Secure Container](https://adventofcode.com/2019/day/4)

struct Puzzle {
    a: u32,
    b: u32,
}

impl Puzzle {
    fn new(data: &str) -> Self {
        let (a, b) = data.trim_ascii().split_once('-').unwrap();

        Self {
            a: a.parse().unwrap(),
            b: b.parse().unwrap(),
        }
    }

    /// Solve part one.
    fn part1(&self) -> u32 {
        let mut result = 0;

        for n in self.a..=self.b {
            let s = n.to_string();
            let mut ok = true;
            let mut same_adj = false;
            for (i, j) in s.chars().zip(s.chars().skip(1)) {
                if j < i {
                    ok = false;
                    break;
                }
                if i == j {
                    same_adj = true;
                }
            }
            if same_adj && ok {
                result += 1;
            }
        }
        result
    }

    /// Solve part two.
    fn part2(&self) -> u32 {
        let mut result = 0;

        for n in self.a..=self.b {
            let s = n.to_string();
            let mut ok = true;
            let mut freq = [0; 10];
            for (i, j) in s.chars().zip(s.chars().skip(1)) {
                if j < i {
                    ok = false;
                    break;
                }
            }
            if ok {
                for c in s.chars() {
                    freq[c.to_digit(10).unwrap() as usize] += 1;
                }
                if freq.iter().any(|&x| x == 2) {
                    result += 1;
                }
            }
        }
        result
    }
}

/// # Panics
#[must_use]
pub fn solve(data: &str) -> (u32, u32) {
    let puzzle = Puzzle::new(data);
    (puzzle.part1(), puzzle.part2())
}

pub fn main() {
    let args = aoc::parse_args();
    args.run(solve);
}
