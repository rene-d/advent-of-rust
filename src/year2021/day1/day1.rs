//! [Day 1: Sonar Sweep](https://adventofcode.com/2021/day/1)

pub fn main() {
    let args = aoc::parse_args();
    args.run(solve);
}

/// # Panics
/// over malformed input
#[must_use]
pub fn solve(data: &str) -> (i32, i32) {
    let mut prev_num = 999_999_999_u32;
    let mut part1 = 0;

    let mut data2 = vec![];

    // step 1
    for line in data.lines() {
        // convertit string -> u32
        let num: u32 = line.parse().unwrap();

        // est-ce que on est en "increase" ?
        if prev_num < num {
            part1 += 1;
        }
        prev_num = num;

        // pour le step 2
        data2.push(num);
    }

    // step 2
    prev_num = 999_999_999_u32;
    let mut part2 = 0;

    for i in 0..data2.len() - 2 {
        let num = data2[i] + data2[i + 1] + data2[i + 2];

        if prev_num < num {
            part2 += 1;
        }
        prev_num = num;
    }

    (part1, part2)
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = include_str!("test.txt");

    #[test]
    fn test_solve() {
        assert_eq!(solve(TEST_INPUT), (7, 5));
    }
}
