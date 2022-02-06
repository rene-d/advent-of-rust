//! [Day 16: Dragon Checksum](https://adventofcode.com/2016/day/16)

fn main() {
    let data = aoc::load_input_data(16).trim().to_owned();

    println!("{} ", checksum(&fill(&data, 272)));
    println!("{} ", checksum(&fill(&data, 35_651_584)));
}

/// compute the checksum recursively
/// Nota: this function could be *really* optimized, the whole program too.
fn checksum(data: &str) -> String {
    if data.len() % 2 == 1 {
        data.to_owned()
    } else {
        let mut result = String::new();

        for i in 0..data.len() / 2 {
            let pos = 2 * i;
            let pair = &data[pos..(pos + 2)];

            result.push(if pair == "00" || pair == "11" { '1' } else { '0' });
        }

        checksum(&result)
    }
}

/// fill the disk
fn fill(seed: &str, length: usize) -> String {
    let mut result = seed.to_owned();
    while result.len() < length {
        result = step(&result);
    }
    result.truncate(length);
    result
}

/// one step of the fill
/// return ``a+"0"+flip_bits(reverse(a))``
fn step(a: &str) -> String {
    let mut result = String::new();

    result.push_str(a);
    result.push('0');
    for c in a.chars().rev() {
        result.push(if c == '1' { '0' } else { '1' });
    }

    result
}

#[test]
fn test_step() {
    assert_eq!(step("1"), "100");
    assert_eq!(step("0"), "001");
    assert_eq!(step("11111"), "11111000000");
    assert_eq!(step("111100001010"), "1111000010100101011110000");
}
#[test]
fn test_fill() {
    assert_eq!(step("10000"), "10000011110");
    assert_eq!(step("10000011110"), "10000011110010000111110");
    assert_eq!(fill("10000", 20), "10000011110010000111");
}

#[test]
fn test_checksum() {
    assert_eq!(checksum("110010110100"), "100");
}
