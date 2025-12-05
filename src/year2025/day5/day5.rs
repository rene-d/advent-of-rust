//! [Day 5: Cafeteria](https://adventofcode.com/2025/day/5)

fn merge_ranges(mut ranges: Vec<(u64, u64)>) -> Vec<(u64, u64)> {
    if ranges.is_empty() {
        return Vec::new();
    }

    ranges.sort_by_key(|k| k.0);

    let mut merged = Vec::new();
    let (mut cur_start, mut cur_end) = ranges[0];

    for &(s, e) in ranges.iter().skip(1) {
        if s <= cur_end + 1 {
            cur_end = std::cmp::max(cur_end, e);
        } else {
            merged.push((cur_start, cur_end));
            cur_start = s;
            cur_end = e;
        }
    }

    merged.push((cur_start, cur_end));
    merged
}

/// # Panics
#[must_use]
pub fn solve(data: &str) -> (usize, u64) {
    let (ranges_str, fresh_ids_str) = data.split_once("\n\n").unwrap();

    let ranges: Vec<(u64, u64)> = ranges_str
        .lines()
        .map(|line| {
            let (a, b) = line.split_once('-').unwrap();
            (a.parse().unwrap(), b.parse().unwrap())
        })
        .collect();

    let fresh_ids: Vec<u64> = fresh_ids_str
        .lines()
        .map(|id| id.parse().unwrap())
        .collect();

    let merged = merge_ranges(ranges);

    // Part 1
    let mut part1 = 0;
    for &id in &fresh_ids {
        let i = merged
            .binary_search_by(|(a, _)| a.cmp(&id))
            .unwrap_or_else(|i| i);

        if i > 0 {
            let (_, end) = merged[i - 1];
            if id <= end {
                part1 += 1;
            }
        }
    }

    // Part 2
    let part2: u64 = merged.iter().map(|&(a, b)| b - a + 1).sum();

    (part1, part2)
}

pub fn main() {
    let args = aoc::parse_args();
    args.run(solve);
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = include_str!("test.txt");

    #[test]
    fn parts() {
        let (p1, p2) = solve(TEST_INPUT);
        assert_eq!(p1, 3);
        assert_eq!(p2, 14);
    }
}
