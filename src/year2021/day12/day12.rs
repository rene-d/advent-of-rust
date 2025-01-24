//! [Day 12: Passage Pathing](https://adventofcode.com/2021/day/12)

use std::collections::VecDeque;

use rustc_hash::{FxHashMap, FxHashSet};

fn compute_paths(data: &str, visite_small_twice: bool) -> u32 {
    // Map containing each cave and its neighbors as a list
    let mut nodes: FxHashMap<&str, Vec<&str>> = FxHashMap::default();

    // Fill the map with caves
    for entry in data.lines() {
        // Split line between "left" cave and "right" cave
        let (left, right) = entry.split_once('-').unwrap();

        // Update map with eventual new caves and neighbors
        nodes.entry(left).or_default().push(right);
        nodes.entry(right).or_default().push(left);
    }

    // Initialize paths list
    let mut heap = VecDeque::new();
    let mut path_count = 0;

    let mut start = FxHashSet::default();
    start.insert("start");

    heap.push_back(("start", start, None));

    while let Some((node, once, twice)) = heap.pop_front() {
        if node == "end" {
            // Count this path and let it be removed from the paths list
            path_count += 1;
            continue;
        }

        // Get neighbors from last node
        let neighbor_list = nodes.get(&node).unwrap();

        for neighbor in neighbor_list {
            // Check wether this small cave was visited
            if !once.contains(neighbor) {
                if neighbor.to_lowercase() == *neighbor {
                    // We add this new path
                    let mut new_once = once.clone();
                    new_once.insert(neighbor);
                    heap.push_back((neighbor, new_once, twice));
                } else {
                    heap.push_back((neighbor, once.clone(), twice));
                }
            }
            // Check wether we already visited twice a small cave
            else if visite_small_twice
                && twice.is_none()
                && once.contains(neighbor)
                && *neighbor != "start"
                && *neighbor != "end"
            {
                // We add this new path
                heap.push_back((neighbor, once.clone(), Some(neighbor)));
            }
        }
    }

    path_count
}

pub fn main() {
    let args = aoc::parse_args();
    args.run(solve);
}

/// # Panics
#[must_use]
pub fn solve(data: &str) -> (u32, u32) {
    let small_once = compute_paths(data, false);
    let small_twice = compute_paths(data, true);

    (small_once, small_twice)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_slightly_larger() {
        let data = "\
dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc
";

        assert_eq!(compute_paths(data, false), 19);
        assert_eq!(compute_paths(data, true), 103);
    }

    #[test]
    fn test_even_larger() {
        let data = "\
fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW
";
        assert_eq!(compute_paths(data, false), 226);
        assert_eq!(compute_paths(data, true), 3509);
    }
}
