//! [Day 3: Binary Diagnostic](https://adventofcode.com/2021/day/3)

pub fn main() {
    let args = aoc::parse_args();
    args.run(solve);
}

/// # Panics
/// over malformed input
#[must_use]
pub fn solve(data: &str) -> (i32, isize) {
    let data: Vec<_> = data.lines().collect();
    (part1(&data), part2(&data))
}

/// step 2
fn part2(data: &[&str]) -> isize {
    let nb_bits = data[0].len();
    // println!("nb_bits: {}", nb_bits);

    // oxygen generator rating
    let mut dioxygen_rate = 0;
    let mut dioxygen_start = String::new();

    for bit in 0..nb_bits {
        let mut one = 0;
        let mut nb = 0;

        for value in data {
            if value.starts_with(&dioxygen_start) {
                let c = value.chars().nth(bit).unwrap();
                if c == '1' {
                    one += 1;
                    dioxygen_rate = isize::from_str_radix(value, 2).unwrap();
                }
                nb += 1;
            }
        }

        if one >= nb - one {
            dioxygen_start.push('1');
        } else {
            dioxygen_start.push('0');
        }
    }

    // CO2 scrubber rating
    let mut co2_rate = 0;
    let mut co2_start = String::new();

    for bit in 0..nb_bits {
        let mut one = 0;
        let mut nb = 0;

        for value in data {
            if value.starts_with(&co2_start) {
                let c = value.chars().nth(bit).unwrap();
                if c == '1' {
                    one += 1;
                } else {
                    co2_rate = isize::from_str_radix(value, 2).unwrap();
                }
                nb += 1;
            }
        }

        if one >= nb - one {
            co2_start.push('0');
        } else {
            co2_start.push('1');
        }
    }
    dioxygen_rate * co2_rate
}

/// step 1: compute `gamma_rate` * `espilon_rate`
fn part1(data: &[&str]) -> i32 {
    let mut gamma_rate = 0;
    let mut freq_list: [i32; 12] = [0; 12];
    let mut nb = 0;

    let width = data.first().unwrap().len();
    let mask = (1 << width) - 1;

    for bits in data {
        for (i, bit) in bits.chars().enumerate() {
            assert!(i < width);
            if bit == '1' {
                freq_list[i] += 1_i32;
            }
        }
        nb += 1;
    }

    for freq in freq_list.iter().take(width) {
        gamma_rate *= 2;
        if *freq >= nb / 2 {
            gamma_rate += 1;
        }
    }

    let espilon_rate = mask - gamma_rate;

    gamma_rate * espilon_rate
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = include_str!("test.txt");

    #[test]
    fn test_part1() {
        let data = load_data();
        assert_eq!(part1(&data), 198);
    }

    #[test]
    fn test_part2() {
        let data = load_data();
        assert_eq!(part2(&data), 230);
    }

    /// load data from file
    fn load_data<'a>() -> Vec<&'a str> {
        TEST_INPUT.lines().collect()
    }
}
