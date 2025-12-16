//! [Day 3: Binary Diagnostic](https://adventofcode.com/2021/day/3)

pub fn main() {
    let args = aoc::parse_args();
    args.run(solve);
}

/// # Panics
#[must_use]
pub fn solve(data: &str) -> (u32, u32) {
    let (bit_length, numbers) = parse_data(data);
    (part1(bit_length, &numbers), part2(bit_length, &numbers))
}

enum RatingType {
    Dioxygen,
    CarbonDioxide,
}

fn filter_rating(bit_length: usize, numbers: &[u32], rating_type: &RatingType) -> u32 {
    let mut remaining = numbers.to_vec();

    for i in (0..bit_length).rev() {
        if remaining.len() == 1 {
            break;
        }

        let bit: u32 = 1 << i;

        let zeros = remaining.iter().filter(|&&num| (num & bit) == 0).count();
        let ones = remaining.len() - zeros;

        let keep_bit = match rating_type {
            RatingType::Dioxygen => {
                // keep most common bit, tie: 1
                if ones >= zeros { bit } else { 0 }
            }
            RatingType::CarbonDioxide => {
                // keep least common bit, tie: 0
                if zeros <= ones { 0 } else { bit }
            }
        };

        remaining.retain(|&num| (num & bit) == keep_bit);
    }

    remaining[0]
}

/// step 2
fn part2(bit_length: usize, numbers: &[u32]) -> u32 {
    filter_rating(bit_length, numbers, &RatingType::Dioxygen)
        * filter_rating(bit_length, numbers, &RatingType::CarbonDioxide)
}

/// step 1: compute `gamma_rate` * `espilon_rate`
fn part1(bit_length: usize, data: &[u32]) -> u32 {
    let mut gamma_rate = 0;
    let mut freq_list: [u32; 12] = [0; 12];
    let mut nb = 0;

    let mask = (1 << bit_length) - 1;

    for num in data {
        for i in 0..bit_length {
            if num & (1 << i) != 0 {
                freq_list[bit_length - i - 1] += 1;
            }
        }
        nb += 1;
    }

    for freq in freq_list.iter().take(bit_length) {
        gamma_rate *= 2;
        if *freq >= nb / 2 {
            gamma_rate += 1;
        }
    }

    let espilon_rate = mask - gamma_rate;

    gamma_rate * espilon_rate
}

fn parse_data(data: &str) -> (usize, Vec<u32>) {
    let bit_length = data.find('\n').unwrap();
    let numbers = data
        .lines()
        .filter_map(|line| u32::from_str_radix(line, 2).ok())
        .collect();

    (bit_length, numbers)
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = include_str!("test.txt");

    #[test]
    fn test_part1() {
        let (bit_length, numbers) = parse_data(TEST_INPUT);
        assert_eq!(part1(bit_length, &numbers), 198);
    }

    #[test]
    fn test_part2() {
        let (bit_length, numbers) = parse_data(TEST_INPUT);
        assert_eq!(part2(bit_length, &numbers), 230);
    }
}
