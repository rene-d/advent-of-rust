//! [Day 9: Stream Processing](https://adventofcode.com/2017/day/9)

struct Puzzle {
    score: u32,
    garbage: u32,
}

impl Puzzle {
    /// Initialize from the puzzle input.
    fn solve(data: &str) -> Self {
        let mut it = data.trim_ascii().bytes();

        let mut in_group = 0;
        let mut in_garbage = false;
        let mut score = 0;
        let mut garbage = 0;

        while let Some(c) = it.next() {
            if c == b'!' {
                it.next();
            } else if in_garbage {
                if c == b'>' {
                    in_garbage = false;
                } else {
                    garbage += 1;
                }
            } else if c == b'<' {
                in_garbage = true;
            } else if c == b'{' {
                in_group += 1;
            } else if c == b'}' {
                score += in_group;
                in_group -= 1;
            }
        }

        Self { score, garbage }
    }

    /// Solve part one.
    const fn part1(&self) -> u32 {
        self.score
    }

    /// Solve part two.
    const fn part2(&self) -> u32 {
        self.garbage
    }
}

fn main() {
    let args = aoc::parse_args();
    let puzzle = Puzzle::solve(&args.input);
    println!("{}", puzzle.part1());
    println!("{}", puzzle.part2());
}

/// Test from puzzle input
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(Puzzle::solve("{}").part1(), 1);
        assert_eq!(Puzzle::solve("{{{}}}").part1(), 6);
        assert_eq!(Puzzle::solve("{{},{}}").part1(), 5);
        assert_eq!(Puzzle::solve("{{{},{},{{}}}}").part1(), 16);
        assert_eq!(Puzzle::solve("{<a>,<a>,<a>,<a>}").part1(), 1);
        assert_eq!(Puzzle::solve("{{<ab>},{<ab>},{<ab>},{<ab>}}").part1(), 9);
        assert_eq!(Puzzle::solve("{{<!!>},{<!!>},{<!!>},{<!!>}}").part1(), 9);
        assert_eq!(Puzzle::solve("{{<a!>},{<a!>},{<a!>},{<ab>}}").part1(), 3);
    }

    #[test]
    fn part2() {
        assert_eq!(Puzzle::solve("<>").part2(), 0);
        assert_eq!(Puzzle::solve("<random characters>").part2(), 17);
        assert_eq!(Puzzle::solve("<<<<>").part2(), 3);
        assert_eq!(Puzzle::solve("<{!>}>").part2(), 2);
        assert_eq!(Puzzle::solve("<!!>").part2(), 0);
        assert_eq!(Puzzle::solve("<!!!>>").part2(), 0);
        assert_eq!(Puzzle::solve(r#"<{o"i!a,<{i<a>"#).part2(), 10);
    }
}
