//! [Day 3: Lobby](https://adventofcode.com/2025/day/3)

/// Find the High Voltage (🤘🎸 AC⚡️DC) of two batteries in a bank.
fn high_voltage(bank: &str) -> u64 {
    let batteries = bank.as_bytes();

    (0..batteries.len() - 1)
        .map(|i| {
            let first = u64::from(batteries[i] - b'0');
            let second = batteries[i + 1..]
                .iter()
                .map(|batt| u64::from(batt - b'0'))
                .max()
                .unwrap();

            first * 10 + second
        })
        .max()
        .unwrap()
}

/// Find the largest joltage of n batteries in a bank.
fn largest_joltage_n(bank: &str, n: usize) -> u64 {
    let mut batteries = Vec::new();
    let mut remove = bank.len().saturating_sub(n);

    for battery in bank.as_bytes() {
        while remove > 0 && !batteries.is_empty() && batteries.last().unwrap() < &battery {
            batteries.pop();
            remove -= 1;
        }
        batteries.push(battery);
    }

    batteries
        .into_iter()
        .take(n)
        .fold(0, |acc, batt| acc * 10 + u64::from(batt - b'0'))
}

/// # Panics
#[must_use]
pub fn solve(data: &str) -> (u64, u64) {
    let mut part1 = 0;
    let mut part2 = 0;

    for bank in data.lines() {
        part1 += high_voltage(bank);
        part2 += largest_joltage_n(bank, 12);
    }

    (part1, part2)
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
    fn parts() {
        let (p1, p2) = solve(&TEST_INPUT);
        assert_eq!(p1, 357);
        assert_eq!(p2, 3121910778619);
    }

    #[test]
    fn part1() {
        assert_eq!(high_voltage("987654321111111"), 98);
        assert_eq!(high_voltage("811111111111119"), 89);
        assert_eq!(high_voltage("234234234234278"), 78);
        assert_eq!(high_voltage("818181911112111"), 92);

        assert_eq!(largest_joltage_n("987654321111111", 2), 98);
        assert_eq!(largest_joltage_n("811111111111119", 2), 89);
        assert_eq!(largest_joltage_n("234234234234278", 2), 78);
        assert_eq!(largest_joltage_n("818181911112111", 2), 92);
    }

    #[test]
    fn part2() {
        assert_eq!(largest_joltage_n("987654321111111", 12), 987654321111);
        assert_eq!(largest_joltage_n("811111111111119", 12), 811111111119);
        assert_eq!(largest_joltage_n("234234234234278", 12), 434234234278);
        assert_eq!(largest_joltage_n("818181911112111", 12), 888911112111);
    }
}
