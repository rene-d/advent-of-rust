//! [Day 20: A Regular Map](https://adventofcode.com/2018/day/20)

use rustc_hash::{FxHashMap, FxHashSet};
use std::collections::VecDeque;

/// # Panics
/// over malformed input
#[must_use]
pub fn solve(data: &str) -> (u32, u32) {
    let mut edges: FxHashMap<(i32, i32), FxHashSet<(i32, i32)>> = FxHashMap::default();

    let mut branchs = vec![];

    let mut x = 0;
    let mut y = 0;

    for c in data.trim_ascii().chars() {
        match c {
            '^' | '$' => (),
            '(' => branchs.push((x, y)),
            ')' => {
                (x, y) = branchs.pop().unwrap();
            }
            '|' => {
                (x, y) = *branchs.last().unwrap();
            }
            _ => {
                let (dx, dy) = match c {
                    'N' => (0, -1),
                    'E' => (1, 0),
                    'S' => (0, 1),
                    'W' => (-1, 0),
                    _ => panic!("unknown char '{c}"),
                };
                (*edges.entry((x, y)).or_default()).insert((dx, dy));

                x += dx;
                y += dy;
            }
        }
    }

    // solve

    let mut q = VecDeque::new();
    let mut seen = FxHashSet::default();

    let mut max_steps = 0;
    let mut thousand_doors = 0;

    q.push_back((0, 0, 0));
    seen.insert((0, 0));

    while let Some((steps, x, y)) = q.pop_front() {
        max_steps = max_steps.max(steps);

        if steps >= 1000 {
            thousand_doors += 1;
        }

        if let Some(neighbors) = edges.get(&(x, y)) {
            for (dx, dy) in neighbors {
                let nx = x + dx;
                let ny = y + dy;

                if seen.insert((nx, ny)) {
                    q.push_back((steps + 1, nx, ny));
                }
            }
        }
    }

    (max_steps, thousand_doors)
}

pub fn main() {
    let args = aoc::parse_args();
    args.run(solve);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test01() {
        let answers = solve("^ENWWW(NEEE|SSE(EE|N))$");
        assert_eq!(answers.0, 10);
    }

    #[test]
    fn test02() {
        let answers = solve("^ENNWSWW(NEWS|)SSSEEN(WNSE|)EE(SWEN|)NNN$");
        assert_eq!(answers.0, 18);
    }

    #[test]
    fn test03() {
        let answers = solve("^ESSWWN(E|NNENN(EESS(WNSE|)SSS|WWWSSSSE(SW|NNNE)))$");
        assert_eq!(answers.0, 23);
    }

    #[test]
    fn test04() {
        let answers = solve("^WSSEESWWWNW(S|NENNEEEENN(ESSSSW(NWSW|SSEN)|WSWWN(E|WWS(E|SS))))$");
        assert_eq!(answers.0, 31);
    }
}
