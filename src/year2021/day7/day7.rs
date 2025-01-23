//! [Day 7: The Treachery of Whales](https://adventofcode.com/2021/day/7)

pub fn main() {
    let args = aoc::parse_args();
    args.run(solve);
}

/// # Panics
/// over malformed input
#[must_use]
pub fn solve(data: &str) -> (i32, i32) {
    let positions = data
        .trim_ascii()
        .split(',')
        .map_while(|s| s.parse::<i32>().ok())
        .collect::<Vec<i32>>();

    let mut min_sum1 = i32::MAX;
    let mut min_sum2 = i32::MAX;

    let mm = positions.iter().max().unwrap();
    for pos in 0..*mm {
        let mut sum1 = 0;
        let mut sum2 = 0;
        for q in &positions {
            let n = (q - pos).abs();

            sum1 += n;
            sum2 += n * (n + 1) / 2;
        }
        if sum1 < min_sum1 {
            min_sum1 = sum1;
        }

        if sum2 < min_sum2 {
            min_sum2 = sum2;
        }
    }

    (min_sum1, min_sum2)
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = include_str!("test.txt");

    #[test]
    fn test_solve() {
        assert_eq!(solve(TEST_INPUT), (37, 168));
    }
}
