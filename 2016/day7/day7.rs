/*!
[Day 7: Internet Protocol Version 7](https://adventofcode.com/2016/day/7)

While snooping around the local network of EBHQ, you compile a list of
[IP addresses](https://en.wikipedia.org/wiki/IP_address) (they're ``IPv7``,
of course; [IPv6](https://en.wikipedia.org/wiki/IPv6) is much too limited).
You'd like to figure out which IPs support TLS (transport-layer snooping).

An IP supports TLS if it has an Autonomous Bridge Bypass Annotation, or
**ABBA**. An ABBA is any four-character sequence which consists of a pair
of two different characters followed by the reverse of that pair, such as
`xyyx` or `abba`. However, the IP also must not have an ABBA within any
hypernet sequences, which are contained by **square brackets**.

For example:

- `abba[mnop]qrst` supports TLS (`abba` outside square brackets).
- `abcd[bddb]xyyx` does not support TLS (`bddb` is within square brackets,
  even though `xyyx` is outside square brackets).
- `aaaa[qwer]tyui` does not support TLS (`aaaa` is invalid; the interior
  characters must be different).
- `ioxxoj[asdfgh]zxcvbn` supports TLS (`oxxo` is outside square brackets,
  even though it's within a larger string).

**How many IPs** in your puzzle input support TLS?

--- Part Two ---

You would also like to know which IPs support **SSL** (super-secret
    listening).

An IP supports SSL if it has an Area-Broadcast Accessor, or **ABA**, anywhere
in the supernet sequences (outside any square bracketed sections), and a
corresponding Byte Allocation Block, or **BAB**, anywhere in the hypernet
sequences. An ABA is any three-character sequence which consists of the same
character twice with a different character between them, such as `xyx` or
`aba`. A corresponding BAB is the same characters but in reversed positions:
`yxy` and `bab`, respectively.

For example:

- `aba[bab]xyz` supports SSL (`aba` outside square brackets with corresponding
  bab within square brackets).
- `xyx[xyx]xyx` does not support SSL (`xyx`, but no corresponding `yxy`).
- `aaa[kek]eke` supports SSL (`eke` in supernet with corresponding `kek` in
  hypernet; the `aaa` sequence is not related, because the interior character
  must be different).
- `zazbz[bzb]cdb` supports SSL (`zaz` has no corresponding `aza`, but `zbz` has
  a corresponding `bzb`, even though `zaz` and `zbz` overlap).

How many IPs in your puzzle input support SSL?
*/

use std::collections::HashSet;

/// ``main`` reads the puzzle input then solves part 1 and part 2
fn main() {
    let data = std::fs::read_to_string("input.txt").unwrap();

    println!("{}", part1(&data));
    println!("{}", part2(&data));
}

/// ``part1`` counts the IPs that support TLS
fn part1(data: &str) -> usize {
    data.split('\n').filter(|s| support_tls(s)).count()
}

/// ``part2`` counts the IPs that support SSL
fn part2(data: &str) -> usize {
    data.split('\n').filter(|s| support_ssl(s)).count()
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
    assert!(support_tls("ioxxoj[asdfgh]zxcvbn"))
}

#[test]
fn test_ssl() {
    assert!(support_ssl("aba[bab]xyz"));
    assert!(!support_ssl("xyx[xyx]xyx"));
    assert!(support_ssl("aaa[kek]eke"));
    assert!(support_ssl("zazbz[bzb]cdb"))
}
