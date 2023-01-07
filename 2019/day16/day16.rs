//! [Day 16: Flawed Frequency Transmission](https://adventofcode.com/2019/day/16)

/// Get the puzzle input into a vector of integer.
fn parse_data(data: &str) -> Vec<u8> {
    data.chars()
        .filter_map(|x| {
            let digit = (x as u32).wrapping_sub('0' as u32);
            if digit < 10 {
                Some(digit as u8)
            } else {
                None
            }
        })
        .collect()
}

/// Solve part one: compute phase after 100 phase rounds.
fn part1(data: &[u8]) -> u32 {
    // #[allow(unused_assignments)]
    let mut p = vec![];
    let mut t = vec![];
    let pattern = [0, 1, 0, -1];

    p = data.to_owned();
    t.resize(data.len(), 0u8);

    for _ in 0..100 {
        for n in 0..data.len() {
            let mut s = 0;
            for i in 0..data.len() {
                s += pattern[(1 + i) / (1 + n) % 4] * p[i] as i32;
            }
            t[n] = (s.abs() % 10) as u8;
        }

        p = t.to_owned();
    }

    p[0..8].iter().fold(0, |acc, d| acc * 10 + (*d as u32))
}

/// Solve part two: find the eight-digit message embedded in the final output list.
fn part2(data: &[u8]) -> u32 {
    let offset = data[0..7].iter().fold(0, |acc, d| acc * 10 + (*d as usize));

    let n = data.len() * 10_000 - offset;
    let mut p = vec![0u8; n];
    let mut t = vec![0u8; n];

    for i in 0..n {
        p[i] = data[(i + offset) % data.len()];
    }

    for _ in 0..100 {
        // value is the sum of all values at its right modulo 10
        let mut s = 0;
        for i in (0..n).rev() {
            s = (s + p[i]) % 10;
            t[i] = s;
        }

        p = t.to_owned();
    }

    p[0..8].iter().fold(0, |acc, d| acc * 10 + (*d as u32))
}

fn main() {
    let puzzle_input = std::fs::read_to_string("input.txt").unwrap();

    let data = parse_data(&puzzle_input);

    println!("{}", part1(&data));
    println!("{}", part2(&data));
}

#[test]
fn test_parse1() {
    assert_eq!(parse_data("123"), [1u8, 2u8, 3u8]);
}

#[test]
fn test_part1() {
    assert_eq!(
        part1(&parse_data("80871224585914546619083218645595")),
        24176176
    );
    assert_eq!(
        part1(&parse_data("19617804207202209144916044189917")),
        73745418
    );
    assert_eq!(
        part1(&parse_data("69317163492948606335995924319873")),
        52432133
    );
}

#[test]
fn test_part2() {
    assert_eq!(
        part2(&parse_data("03036732577212944063491565474664")),
        84462026
    );
    assert_eq!(
        part2(&parse_data("02935109699940807407585447034323")),
        78725270
    );
    assert_eq!(
        part2(&parse_data("03081770884921959731165446850517")),
        53553731
    );
}
