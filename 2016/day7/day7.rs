//! [Day 7: Internet Protocol Version 7](https://adventofcode.com/2016/day/7)

use std::collections::HashSet;

/// ``main`` reads the puzzle input then solves part 1 and part 2
fn main() {
    let filename = if let Some(x) = std::env::args().collect::<Vec<String>>().get(1) {
        x.clone()
    } else {
        "input.txt".to_string()
    };

    let data = std::fs::read_to_string(filename).unwrap();

    println!("{}", part1(&data));
    println!("{}", part2(&data));
}

/// ``part1`` counts the IPs that support TLS
fn part1(data: &str) -> usize {
    data.lines().filter(|s| support_tls(s)).count()
}

/// ``part2`` counts the IPs that support SSL
fn part2(data: &str) -> usize {
    data.lines().filter(|s| support_ssl(s)).count()
}

/// ``support_tls`` looks for an ABBA pattern outside brackets, and not within
fn support_tls(address: &str) -> bool {
    let mut has_abba = false;
    let mut hypernet = false;

    for i in 0..=address.len() - 4 {
        let a = address.chars().nth(i).unwrap();

        if a == '[' {
            hypernet = true;
            continue;
        }
        if a == ']' {
            hypernet = false;
            continue;
        }
        let b = address.chars().nth(i + 1).unwrap();
        let c = address.chars().nth(i + 2).unwrap();
        let d = address.chars().nth(i + 3).unwrap();

        if a == d && b == c && a != b {
            if hypernet {
                return false;
            }
            has_abba = true;
        }
    }
    has_abba
}

/// ``support_ssl`` tests for ABA patterns outside brackets and BAB within brackets
fn support_ssl(address: &str) -> bool {
    let mut hypernet = false;

    let mut supernet_aba = HashSet::new();
    let mut hypernet_bab = HashSet::new();

    for i in 0..=address.len() - 3 {
        let a = address.chars().nth(i).unwrap();

        if a == '[' {
            hypernet = true;
            continue;
        }
        if a == ']' {
            hypernet = false;
            continue;
        }
        let b = address.chars().nth(i + 1).unwrap();
        let c = address.chars().nth(i + 2).unwrap();

        if a == c && a != b {
            if hypernet {
                hypernet_bab.insert((b, a, b));
            } else {
                supernet_aba.insert((a, b, a));
            }
        }
    }

    for aba in supernet_aba {
        if hypernet_bab.contains(&aba) {
            return true;
        }
    }
    false
}

#[cfg(test)]
#[test]
fn test_tls() {
    assert!(support_tls("abba[mnop]qrst"));
    assert!(!support_tls("abcd[bddb]xyyx"));
    assert!(!support_tls("aaaa[qwer]tyui"));
    assert!(support_tls("ioxxoj[asdfgh]zxcvbn"));
}

#[test]
fn test_ssl() {
    assert!(support_ssl("aba[bab]xyz"));
    assert!(!support_ssl("xyx[xyx]xyx"));
    assert!(support_ssl("aaa[kek]eke"));
    assert!(support_ssl("zazbz[bzb]cdb"));
}
