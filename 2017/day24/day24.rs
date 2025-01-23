//! [Day 24: Electromagnetic Moat](https://adventofcode.com/2017/day/24)

use std::collections::VecDeque;

/// # Panics
/// over malformed input
#[must_use]
pub fn solve(data: &str) -> (u32, u32) {
    let ports: Vec<_> = data
        .lines()
        .filter_map(|line| line.split_once('/'))
        .map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap()))
        .collect();

    let mut max_length = 0;
    let mut queue = VecDeque::new();

    let mut max_strength = 0; // part 1 answer
    let mut max_length_strength = 0; // part 2 answer

    queue.push_front((0u32, 0, 1, ports));
    while let Some((pin, strength, length, ports)) = queue.pop_back() {
        //
        max_strength = max_strength.max(strength);

        // match expression is equivalent to the following, clippy is sometimes really weird
        // if length > max_length {
        //     max_length = length;
        //     max_length_strength = strength;
        // } else if length == max_length {
        //     max_length_strength = max_length_strength.max(strength);
        // }

        match length {
            _ if length > max_length => {
                max_length = length;
                max_length_strength = strength;
            }
            _ if length == max_length => {
                max_length_strength = max_length_strength.max(strength);
            }
            _ => {}
        };

        for (i, &(a, b)) in ports.iter().enumerate() {
            let c = if a == pin {
                b
            } else if b == pin {
                a
            } else {
                continue;
            };

            let mut np = ports.clone();
            np.remove(i);

            queue.push_front((c, strength + a + b, length + 1, np));
        }
    }

    (max_strength, max_length_strength)
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
    fn test_solve() {
        let answer = solve(TEST_INPUT);
        assert_eq!(answer.0, 31);
        assert_eq!(answer.1, 19);
    }
}
