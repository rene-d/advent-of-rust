/*!
[Day 9: Explosives in Cyberspace](https://adventofcode.com/2016/day/9)

Wandering around a secure area, you come across a datalink port to a new
part of the network. After briefly scanning it for interesting files, you
find one file in particular that catches your attention. It's compressed
with an experimental format, but fortunately, the documentation for the
format is nearby.

The format compresses a sequence of characters. Whitespace is ignored. To
indicate that some sequence should be repeated, a marker is added to the
file, like (`10x2`). To decompress this marker, take the subsequent `10`
characters and repeat them `2` times. Then, continue reading the file **after**
the repeated data. The marker itself is not included in the decompressed
output.

If parentheses or other characters appear within the data referenced by a
marker, that's okay - treat it like normal data, not a marker, and then
resume looking for markers after the decompressed section.

What is the **decompressed length** of the file (your puzzle input)? Don't
count whitespace.

--- Part Two ---

Apparently, the file actually uses **version two** of the format.

In version two, the only difference is that markers within decompressed
data are decompressed. This, the documentation explains, provides much
more substantial compression capabilities, allowing many-gigabyte files
to be stored in only a few kilobytes.

Unfortunately, the computer you brought probably doesn't have enough
memory to actually decompress the file; you'll have to **come up with
another way** to get its decompressed length.

What is the **decompressed length** of the file using this improved format?
*/

/// Solve the puzzle with the given input.
fn main() {
    let data = std::fs::read_to_string("input.txt").unwrap();

    println!("{}", part1(&data));
    println!("{}", part2(&data));
}

/// Do part 1 of the puzzle
fn part1(data: &str) -> usize {
    data.split('\n')
        .fold(0, |acc, line| acc + expand_v1(line).len())
}

/// Do part 2 of the puzzle
fn part2(data: &str) -> usize {
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
fn expand_v2(s: &str) -> usize {
    expand(s, 2)
}

/// ``expand`` returns the length of the expanded string (v1 or v2).
fn expand(s: &str, version: u8) -> usize {
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
                .parse::<usize>()
                .unwrap();

            let taken = chars.by_ref().take(take);

            let count = if version == 1 {
                taken.count()
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

#[cfg(test)]
#[test]
fn test_expand_v1() {
    assert_eq!(expand_v1("ADVENT"), "ADVENT");
    assert_eq!(expand_v1("A(1x5)BC"), "ABBBBBC");
    assert_eq!(expand_v1("(3x3)XYZ"), "XYZXYZXYZ");
    assert_eq!(expand_v1("A(2x2)BCD(2x2)EFG"), "ABCBCDEFEFG");
    assert_eq!(expand_v1("(6x1)(1x3)A"), "(1x3)A");
    assert_eq!(expand_v1("X(8x2)(3x3)ABCY"), "X(3x3)ABC(3x3)ABCY");
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
    assert_eq!(expand_v2("(27x12)(20x12)(13x14)(7x10)(1x12)A"), 241920);
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

    assert_eq!(part2(data), 9 + 20 + 241920 + 445);
}
