//! [Day 7: Recursive Circus](https://adventofcode.com/2017/day/7)

use regex::Regex;
use rustc_hash::{FxHashMap, FxHashSet};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct NodeId(u64);

trait NodeIdTrait {
    fn nodeid(self) -> NodeId;
}

impl NodeIdTrait for &str {
    fn nodeid(self) -> NodeId {
        NodeId::from(self)
    }
}

impl NodeId {
    fn as_string(self) -> String {
        let mut key = self.0;
        let mut node = String::new();
        loop {
            let digit = b'a' + (key % 26) as u8;
            node.push(digit as char);
            key /= 26;
            if key == 0 {
                break;
            }
        }
        node
    }
}

impl From<&str> for NodeId {
    fn from(value: &str) -> Self {
        Self(
            value
                .bytes()
                .rev()
                .fold(0, |acc, d| acc * 26 + u64::from(d - b'a')),
        )
    }
}

impl std::fmt::Display for NodeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

struct Puzzle {
    root: NodeId,
    children: FxHashMap<NodeId, Vec<NodeId>>,
    nodes: FxHashMap<NodeId, u32>,
}

impl Puzzle {
    /// Initialize from the puzzle input.
    fn new(data: &str) -> Self {
        let mut all_children = FxHashSet::default();
        let mut children = FxHashMap::default();
        let mut nodes = FxHashMap::default();

        let re = Regex::new(r"^(\w+) \((\d+)\)(?: \-> (.+))?$").unwrap();

        for line in data.lines() {
            let caps = re.captures(line).unwrap();

            let node = caps[1].nodeid();
            let v: u32 = caps[2].parse().unwrap();
            nodes.insert(node, v);

            if let Some(c) = caps.get(3) {
                let child: Vec<NodeId> = c.as_str().split(", ").map(NodeId::from).collect();
                all_children.extend(child.iter().copied());
                children.insert(node, child);
            }
        }

        let keys: FxHashSet<NodeId> = nodes.keys().copied().collect();
        let root = keys.difference(&all_children).next().unwrap();

        Self {
            root: *root,
            children,
            nodes,
        }
    }

    fn traverse(&self, root: NodeId) -> (bool, u32) {
        let mut weight = self.nodes[&root];

        let mut z: FxHashMap<u32, Vec<NodeId>> = FxHashMap::default();

        for &node in self.children.get(&root).unwrap_or(&Vec::new()) {
            let (found, child_weight) = self.traverse(node);

            if found {
                // solution found
                return (found, child_weight);
            }

            z.entry(child_weight).or_default().push(node);
            weight += child_weight; // compute total weight of node 'root'
        }

        if z.keys().len() >= 2 {
            let mut bad_weight = 0;
            let mut bad = 0;
            let mut good = 0;
            for (&cost, n) in &z {
                if n.len() == 1 {
                    bad_weight = self.nodes[&n[0]];
                    bad = cost;
                } else {
                    good = cost;
                }
            }

            return (true, bad_weight + good - bad);
        }

        (false, weight)
    }

    /// Solve part one.
    fn part1(&self) -> String {
        self.root.as_string()
    }

    /// Solve part two.
    fn part2(&self) -> u32 {
        self.traverse(self.root).1
    }
}

/// # Panics
#[must_use]
pub fn solve(data: &str) -> (String, u32) {
    let puzzle = Puzzle::new(data);
    (puzzle.part1(), puzzle.part2())
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
    fn nodeid() {
        let key = NodeId::from("lyvdkxe");
        let node = key.as_string();
        assert_eq!(node, "lyvdkxe");

        let key = "zzlocxd".nodeid();
        let node = key.as_string();
        assert_eq!(node, "zzlocxd");
    }

    #[test]
    fn part1() {
        let puzzle = Puzzle::new(TEST_INPUT);
        assert_eq!(puzzle.part1(), "tknk");
    }

    #[test]
    fn part2() {
        let puzzle = Puzzle::new(TEST_INPUT);
        assert_eq!(puzzle.part2(), 60);
    }
}
