//! [Day 1: Not Quite Lisp](https://adventofcode.com/2015/day/1)

/// Solve the puzzle.
fn solve(data: &str) -> (i32, i32) {
    let mut floor = 0;
    let mut position = 0;
    let mut enter = 0;

    for c in data.chars() {
        position += 1;
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => {}
        }
        if floor == -1 && enter == 0 {
            enter = position;
        }
    }

    (floor, enter)
}

/// Main function.
fn main() {
    let mut args = aoc::parse_args();

    args.run(solve);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test01() {
        assert_eq!(solve("(())").0, 0);
        assert_eq!(solve("()()").0, 0);

        assert_eq!(solve("(((").0, 3);
        assert_eq!(solve("(()(()(").0, 3);

        assert_eq!(solve("))(((((").0, 3);

        assert_eq!(solve("())").0, -1);
        assert_eq!(solve("))(").0, -1);

        assert_eq!(solve(")))").0, -3);
        assert_eq!(solve(")())())").0, -3);
    }

    #[test]
    fn test02() {
        assert_eq!(solve(")").1, 1);
        assert_eq!(solve("()())").1, 5);
    }
}
