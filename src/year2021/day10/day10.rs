//! [Day 10: Syntax Scoring](https://adventofcode.com/2021/day/10)

pub fn main() {
    let args = aoc::parse_args();
    args.run(solve);
}

/// # Panics
#[must_use]
pub fn solve(data: &str) -> (u64, u64) {
    let mut part1 = 0;
    let mut part2 = vec![];

    for line in data.lines() {
        let (corrupted, completed) = check(line);

        part1 += corrupted;
        if completed != 0 {
            part2.push(completed);
        }
    }

    // part2.sort_by(|a, b| a.cmp(b));
    part2.sort_unstable();

    (part1, part2[part2.len() / 2])
}

fn check(line: &str) -> (u64, u64) {
    let mut stack = vec![];

    for c in line.chars() {
        match c {
            '(' => stack.push(')'),
            '[' => stack.push(']'),
            '{' => stack.push('}'),
            '<' => stack.push('>'),
            _ => {
                let d = stack.pop().unwrap();
                if c != d {
                    match c {
                        ')' => return (3, 0),
                        ']' => return (57, 0),
                        '}' => return (1197, 0),
                        '>' => return (25137, 0),
                        _ => return (0, 0),
                    }
                }
            }
        }
    }

    let mut score = 0u64;
    while let Some(d) = stack.pop() {
        match d {
            ')' => score = score * 5 + 1,
            ']' => score = score * 5 + 2,
            '}' => score = score * 5 + 3,
            '>' => score = score * 5 + 4,
            _ => score *= 5,
        }
    }

    (0, score)
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = include_str!("test.txt");

    #[test]
    fn test_solve() {
        assert_eq!(solve(TEST_INPUT), (26397, 288957));
    }
}
