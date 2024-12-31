//! [Day 16: Flawed Frequency Transmission](https://adventofcode.com/2019/day/16)
//!
//! Solve the day16 puzzle of Advent of Code 2019

/// Get the puzzle input into a vector of integer.
fn parse_data(data: &str) -> Vec<u8> {
    data.chars()
        .filter_map(|x| {
            let digit = (x as u32).wrapping_sub('0' as u32);
            if digit < 10 {
                u8::try_from(digit).ok()
            } else {
                None
            }
        })
        .collect()
}

/// Compute a phase of the Flawed Frequency Transmission (FFT)
fn fft(signal: &mut [u8]) {
    let pattern = [0, 1, 0, -1];

    let phase = signal.to_owned();

    for n in 0..signal.len() {
        let mut s = 0;
        for i in 0..signal.len() {
            s += pattern[(1 + i) / (1 + n) % 4] * i32::from(phase[i]);
        }
        signal[n] = u8::try_from(s.abs() % 10).unwrap();
    }
}

/// Solve part one: compute the 100th phase of the signal.
fn part1(data: &[u8]) -> u32 {
    let mut p = data.to_owned();
    for _ in 0..100 {
        fft(&mut p);
    }
    p[0..8].iter().fold(0, |acc, d| acc * 10 + u32::from(*d))
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

        p.clone_from(&t);
    }

    p[0..8].iter().fold(0, |acc, d| acc * 10 + u32::from(*d))
}

fn main() {
    let args = aoc::parse_args();

    let data = parse_data(&args.input);

    println!("{}", part1(&data));
    println!("{}", part2(&data));
}

#[test]
fn test_parse_data() {
    assert_eq!(parse_data("123"), [1u8, 2u8, 3u8]);
}

#[test]
fn test_part1() {
    // values given as examples in the puzzle
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
    // values given as examples in the puzzle
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

#[test]
fn test_fft() {
    // values given as examples in the puzzle

    let mut signal = parse_data("12345678");

    let phases = [
        parse_data("48226158"),
        parse_data("34040438"),
        parse_data("03415518"),
        parse_data("01029498"),
    ];

    for i in 0..3 {
        fft(&mut signal);
        assert_eq!(signal, phases[i]);
    }
}
