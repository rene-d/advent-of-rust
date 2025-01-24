//! [Day 8: Matchsticks](https://adventofcode.com/2015/day/8)

pub fn main() {
    let args = aoc::parse_args();
    args.run(solve);
}

/// # Panics
#[must_use]
pub fn solve(data: &str) -> (usize, usize) {
    let mut total_raw = 0;
    let mut total_decoded = 0;
    let mut total_encoded = 0;

    for line in data.lines() {
        assert_eq!(line.chars().next().unwrap(), '"');
        assert_eq!(line.chars().last().unwrap(), '"');

        let mut len_decoded = line.len() - 2;
        let mut len_encoded = line.len() + 4;

        let mut escape_char = false;
        for c in line.chars().skip(1).take(line.len() - 2) {
            if escape_char {
                escape_char = false;
                if c == 'x' {
                    len_decoded -= 2; // remove the two digits
                }
            } else if c == '\\' {
                escape_char = true;
                len_decoded -= 1; // remove the escape char from the length
            }

            if c == '\\' {
                len_encoded += 1; // escape the backslash
            }
            if c == '"' {
                len_encoded += 1; // escape the double quote
            }
        }

        total_decoded += len_decoded;
        total_encoded += len_encoded;
        total_raw += line.len();
    }

    (total_raw - total_decoded, total_encoded - total_raw)
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = include_str!("test.txt");

    #[test]
    fn test_solve() {
        assert_eq!(solve(TEST_INPUT), (12, 19));
    }
}
