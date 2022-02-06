//! [Day 6: Signals and Noise](https://adventofcode.com/2016/day/6)

fn main() {
    let data = std::fs::read_to_string("input.txt").unwrap();

    let (part1, part2) = solve(&data);
    println!("{}", part1);
    println!("{}", part2);
}

/// solve both parts
fn solve(data: &str) -> (String, String) {
    let mut freq = [[0; 26]; 8];

    for line in data.split('\n') {
        for (i, c) in line.chars().enumerate() {
            assert!(i < 8, "too many chars");
            if ('a'..='z').contains(&c) {
                freq[i][c as usize - 'a' as usize] += 1;
            }
        }
    }

    let mut message_most = String::new();
    let mut message_least = String::new();

    for freq_c in &freq {
        let mut max = usize::MIN;
        let mut max_char = ' ';

        let mut min = usize::MAX;
        let mut min_char = ' ';

        for (j, f) in freq_c.iter().enumerate().take(26) {
            if *f > max {
                max = *f;
                max_char = (u8::try_from(j).unwrap() + b'a') as char;
            }
            if *f != 0 && *f < min {
                min = *f;
                min_char = (u8::try_from(j).unwrap() + b'a') as char;
            }
        }

        message_most.push(max_char);
        message_least.push(min_char);
    }

    (message_most, message_least)
}

#[cfg(test)]
#[test]
fn test_part1() {
    let data = "eedadn
drvtee
eandsr
raavrd
atevrs
tsrnev
sdttsa
rasrtv
nssdts
ntnada
svetve
tesnvt
vntsnd
vrdear
dvrsen
enarar";

    let (part1, part2) = solve(&data);

    assert_eq!(part1, "easter  ".to_string());
    assert_eq!(part2, "advent  ".to_string());
}
