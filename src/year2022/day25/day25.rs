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
    let mut digits = Vec::new();
    loop {
        //  let ch = ['=', '-', '0', '1', '2'][(n + 2).rem_euclid(5) as usize];
        let ch = match (n + 2).rem_euclid(5) {
            0 => '=',
            1 => '-',
            2 => '0',
            3 => '1',
            4 => '2',
            _ => unreachable!(),
        };
        digits.push(ch);
        n = (n + 2).div_euclid(5);
        if n == 0 {
            break;
        }
    }

    digits.iter().rev().collect::<String>()
}

#[must_use]
pub fn solve(data: &str) -> (String, aoc::Christmas) {
    let sum_of_numbers = data.lines().map(from_snafu).sum();

    (to_snafu(sum_of_numbers), aoc::CHRISTMAS)
}

pub fn main() {
    let args = aoc::parse_args();
    args.run(solve);
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = include_str!("test.txt");

    #[test]
    fn test01() {
        let puzzle = solve(TEST_INPUT);
        assert_eq!(puzzle.0, "2=-1=0");
    }

    #[test]
    fn test_from_snafu() {
        assert_eq!(from_snafu("2=-01"), 976);
    }

    #[test]
    fn test_to_snafu() {
        assert_eq!(to_snafu(976), "2=-01");
    }
}
