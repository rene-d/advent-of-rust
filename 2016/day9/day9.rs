//! [Day 9: Explosives in Cyberspace](https://adventofcode.com/2016/day/9)

/// Solve the puzzle with the given input.
fn main() {
    let args = aoc::parse_args();

    println!("{}", part1(&args.input));
    println!("{}", part2(&args.input));
}

/// Do part 1 of the puzzle
fn part1(data: &str) -> u64 {
    data.split('\n').fold(0, |acc, line| acc + expand_v1(line).len() as u64)
}

/// Do part 2 of the puzzle
fn part2(data: &str) -> u64 {
    data.split('\n').fold(0, |acc, line| acc + expand_v2(line))
}

/// ``expand_v1`` returns the expanded string according to the format v1.
fn expand_v1(s: &str) -> String {
    let mut new_s = String::new();
    let mut chars = s.chars();

    while let Some(c) = chars.next() {
        if c == '(' {
            // do expansion
            let take = chars
                .by_ref()
                .take_while(|c| *c != 'x')
                .collect::<String>()
                .parse::<usize>()
                .unwrap();

            let repeat = chars
                .by_ref()
                .take_while(|c| *c != ')')
                .collect::<String>()
                .parse::<usize>()
                .unwrap();

            let expanded = chars.by_ref().take(take).collect::<String>().repeat(repeat);

            new_s.push_str(expanded.as_str());
        } else {
            // just add the character
            new_s.push(c);
        }
    }

    new_s
}

/// ``expand_v2`` returns the length of expanded string according to the format v2.
fn expand_v2(s: &str) -> u64 {
    expand(s, 2)
}

/// ``expand`` returns the length of the expanded string (v1 or v2).
fn expand(s: &str, version: u8) -> u64 {
    let mut new_len = 0;
    let mut chars = s.chars();

    while let Some(c) = chars.next() {
        if c == '(' {
            // do expansion
            let take = chars
                .by_ref()
                .take_while(|c| *c != 'x')
                .collect::<String>()
                .parse::<usize>()
                .unwrap();

            let repeat = chars
                .by_ref()
                .take_while(|c| *c != ')')
                .collect::<String>()
                .parse::<u64>()
                .unwrap();

            let taken = chars.by_ref().take(take);

            let count = if version == 1 {
                taken.count() as u64
            } else {
                expand(taken.collect::<String>().as_str(), version)
            };

            new_len += repeat * count;
        } else {
            // just add the character
            new_len += 1;
        }
    }

    new_len
}

#[test]
fn test_expand_v1() {
    fn test_v1(s: &str, expected: &str) {
        assert_eq!(expand_v1(s), expected);
        assert_eq!(expand(s, 1), expected.len() as u64);
    }
    test_v1("ADVENT", "ADVENT");
    test_v1("A(1x5)BC", "ABBBBBC");
    test_v1("(3x3)XYZ", "XYZXYZXYZ");
    test_v1("A(2x2)BCD(2x2)EFG", "ABCBCDEFEFG");
    test_v1("(6x1)(1x3)A", "(1x3)A");
    test_v1("X(8x2)(3x3)ABCY", "X(3x3)ABC(3x3)ABCY");
}

#[test]
fn test_part1() {
    let data = "ADVENT
A(1x5)BC
(3x3)XYZ
A(2x2)BCD(2x2)EFG
(6x1)(1x3)A
X(8x2)(3x3)ABCY";
    assert_eq!(part1(data), 6 + 7 + 9 + 11 + 6 + 18);
}

#[test]
fn test_expand() {
    assert_eq!(expand("X(8x2)(3x3)ABCY", 1), 18);
    assert_eq!(expand("X(8x2)(3x3)ABCY", 2), 20);
}

#[test]
fn test_expand_v2() {
    assert_eq!(expand_v2("(3x3)XYZ"), 9); // XYZXYZXYZ
    assert_eq!(expand_v2("X(8x2)(3x3)ABCY"), 20); // XABCABCABCABCABCABCY
    assert_eq!(expand_v2("(27x12)(20x12)(13x14)(7x10)(1x12)A"), 241_920);
    assert_eq!(
        expand_v2("(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN"),
        445
    );
}

#[test]
fn test_part2() {
    let data = "(3x3)XYZ
X(8x2)(3x3)ABCY
(27x12)(20x12)(13x14)(7x10)(1x12)A
(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN";

    assert_eq!(part2(data), 9 + 20 + 241_920 + 445);
}
