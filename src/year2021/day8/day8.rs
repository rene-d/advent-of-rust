//! [Day 8: Seven Segment Search](https://adventofcode.com/2021/day/8)

use rustc_hash::FxHashMap;
use rustc_hash::FxHashSet;

pub fn main() {
    let args = aoc::parse_args();
    args.run(solve);
}

/// # Panics
#[must_use]
pub fn solve(data: &str) -> (i32, u32) {
    (part1(data), part2(data))
}

/// Return true if `a` contains all chars of string `b`
fn contains(a: &str, b: &str) -> bool {
    for c in b.chars() {
        if !a.contains(c) {
            return false;
        }
    }
    true
}

fn part2(data: &str) -> u32 {
    // Nota: the Python version is much more readable...
    let mut total = 0;
    for line in data.lines() {
        let (notes1, code1) = line.split_once('|').unwrap();

        let notes = notes1.split_whitespace().collect::<Vec<&str>>();
        let code = code1.split_whitespace().collect::<Vec<&str>>();

        let mut d: FxHashMap<usize, FxHashSet<String>> = FxHashMap::default();

        for c in notes {
            let mut s: Vec<char> = c.chars().collect();
            s.sort_unstable();
            let s_sorted = String::from_iter(s);
            d.entry(c.len()).or_default().insert(s_sorted);
        }

        let mut map: FxHashMap<String, u8> = FxHashMap::default();

        let mut zero: &str = "";
        let one = d.get(&2).unwrap().iter().next().unwrap(); // the only digit with the 2 segments
        let mut two: &str = "";
        let mut three: &str = "";
        let four = d.get(&4).unwrap().iter().next().unwrap(); // the only digit with the 4 segments
        let mut five: &str = "";
        let mut six: &str = "";
        let seven = d.get(&3).unwrap().iter().next().unwrap(); // the only digit with the 3 segments
        let eight = d.get(&7).unwrap().iter().next().unwrap(); // the only digit with 7 segments
        let mut nine: &str = "";

        for x in d.get(&6).unwrap() {
            // could be 0 6 9
            if contains(x, one) {
                // si on n'a pas les deux segments du 1
                if contains(x, four) {
                    // si 4 est inclus, c'est forcément un 9
                    nine = x;
                } else {
                    // sinon, c'est forcément un 0
                    zero = x;
                }
            } else {
                // sinon, c'est forcément un 6
                six = x;
            }
        }

        for x in d.get(&5).unwrap() {
            // 2 3 5
            if contains(x, seven) {
                // si 7 est inclus, c est forcément un 3
                three = x;
            } else if contains(nine, x) {
                // si c'est inclus dans 9, c'est forcément un 5
                five = x;
            } else {
                // sinon, c'est forcément un 2
                two = x;
            }
        }

        map.insert(zero.to_string(), 0);
        map.insert(one.clone(), 1);
        map.insert(two.to_string(), 2);
        map.insert(three.to_string(), 3);
        map.insert(four.clone(), 4);
        map.insert(five.to_string(), 5);
        map.insert(six.to_string(), 6);
        map.insert(seven.clone(), 7);
        map.insert(eight.clone(), 8);
        map.insert(nine.to_string(), 9);

        let mut r = 0;
        for d in code {
            let mut s: Vec<char> = d.chars().collect();
            s.sort_unstable();
            let s_sorted = String::from_iter(s);

            let v = u32::from(*map.entry(s_sorted).or_insert(0));
            r = r * 10 + v;
        }
        total += r;
    }

    total
}

fn part1(data: &str) -> i32 {
    let mut digit_one = 0;
    let mut digit_four = 0;
    let mut digit_seven = 0;
    let mut digit_eight = 0;

    for line in data.lines() {
        let x = line.split_once('|').unwrap().1.trim();
        for c in x.split_whitespace() {
            match c.len() {
                2 => digit_one += 1,
                3 => digit_seven += 1,
                4 => digit_four += 1,
                7 => digit_eight += 1,
                _ => (),
            }
        }
    }

    digit_one + digit_four + digit_seven + digit_eight
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = include_str!("test.txt");

    #[test]
    fn test_contains() {
        assert!(contains("abcde", "ab"));
        assert!(contains("abcde", "ba"));
        assert!(contains("abcde", "abcde"));
        assert!(!contains("abcde", "az"));
        assert!(!contains("abcde", "ef"));
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 26);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 61229);
    }
}
