//! [Day 9: Stream Processing](https://adventofcode.com/2017/day/9)

/// Initialize from the puzzle input.
#[must_use]
pub fn solve(data: &str) -> (u32, u32) {
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

    (score, garbage)
}

pub fn main() {
    let args = aoc::parse_args();
    args.run(solve);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(solve("{}").0, 1);
        assert_eq!(solve("{{{}}}").0, 6);
        assert_eq!(solve("{{},{}}").0, 5);
        assert_eq!(solve("{{{},{},{{}}}}").0, 16);
        assert_eq!(solve("{<a>,<a>,<a>,<a>}").0, 1);
        assert_eq!(solve("{{<ab>},{<ab>},{<ab>},{<ab>}}").0, 9);
        assert_eq!(solve("{{<!!>},{<!!>},{<!!>},{<!!>}}").0, 9);
        assert_eq!(solve("{{<a!>},{<a!>},{<a!>},{<ab>}}").0, 3);
    }

    #[test]
    fn part2() {
        assert_eq!(solve("<>").1, 0);
        assert_eq!(solve("<random characters>").1, 17);
        assert_eq!(solve("<<<<>").1, 3);
        assert_eq!(solve("<{!>}>").1, 2);
        assert_eq!(solve("<!!>").1, 0);
        assert_eq!(solve("<!!!>>").1, 0);
        assert_eq!(solve(r#"<{o"i!a,<{i<a>"#).1, 10);
    }
}
