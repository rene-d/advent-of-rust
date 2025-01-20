//! [Day 23: LAN Party](https://adventofcode.com/2024/day/23)

use petgraph::graph::{NodeIndex, UnGraph};
use rustc_hash::{FxHashMap, FxHashSet};

// Bron-Kerbosch recursive algorithm to find cliques
fn bron_kerbosch(
    graph: &UnGraph<(), ()>,
    r: &FxHashSet<NodeIndex>,
    p: &mut FxHashSet<NodeIndex>,
    x: &mut FxHashSet<NodeIndex>,
    cliques: &mut Vec<Vec<NodeIndex>>,
) {
    if p.is_empty() && x.is_empty() {
        cliques.push(r.iter().copied().collect());
        return;
    }

    let p_clone = p.clone();
    for &v in &p_clone {
        let mut r_new = r.clone();
        r_new.insert(v);

        let neighbors: FxHashSet<_> = graph.neighbors(v).collect();
        let mut p_new: FxHashSet<NodeIndex> = p.intersection(&neighbors).copied().collect();
        let mut x_new: FxHashSet<NodeIndex> = x.intersection(&neighbors).copied().collect();

        bron_kerbosch(graph, &r_new, &mut p_new, &mut x_new, cliques);

        p.remove(&v);
        x.insert(v);
    }
}

struct Puzzle {
    connections: Vec<(String, String)>,
}

impl Puzzle {
    fn new(data: &str) -> Self {
        let mut connections = Vec::new();
        for line in data.lines() {
            if let Some((from, to)) = line.split_once('-') {
                connections.push((from.to_string(), to.to_string()));
            }
        }
        Self { connections }
    }

    /// Solve part one.
    fn part1(&self) -> usize {
        let mut graph: FxHashMap<String, FxHashSet<String>> = FxHashMap::default();
        let mut triangles: FxHashSet<[&String; 3]> = FxHashSet::default();

        for (n1, n2) in &self.connections {
            graph
                .entry(n1.to_string())
                .or_default()
                .insert(n2.to_string());

            graph
                .entry(n2.to_string())
                .or_default()
                .insert(n1.to_string());
        }

        for (node, neighbors) in &graph {
            for n1 in neighbors {
                for n2 in neighbors {
                    if n1 != n2 && graph[n1].contains(n2) {
                        let mut triangle = [node, n1, n2];
                        triangle.sort_unstable();

                        triangles.insert(triangle);
                    }
                }
            }
        }

        triangles
            .iter()
            .filter(|triangle| triangle.iter().any(|node| node.starts_with('t')))
            .count()
    }

    /// Solve part two.
    fn part2(&self) -> String {
        let mut graph = UnGraph::<(), ()>::new_undirected();

        let mut nodes = FxHashMap::default();

        for (n1, n2) in &self.connections {
            let i1 = *nodes
                .entry(n1.clone())
                .or_insert_with(|| graph.add_node(()));
            let i2 = *nodes
                .entry(n2.clone())
                .or_insert_with(|| graph.add_node(()));
            graph.add_edge(i1, i2, ());
        }

        let mut node_names: FxHashMap<&NodeIndex, &str> = FxHashMap::default();
        for (k, v) in &nodes {
            node_names.insert(v, k);
        }

        // find the largest clique
        let mut cliques = Vec::new();
        let r = FxHashSet::default();
        let mut p: FxHashSet<NodeIndex> = graph.node_indices().collect();
        let mut x = FxHashSet::default();

        bron_kerbosch(&graph, &r, &mut p, &mut x, &mut cliques);

        let largest_clique = cliques.into_iter().max_by_key(std::vec::Vec::len);

        if let Some(largest_clique) = largest_clique {
            let mut clique_names = largest_clique
                .iter()
                .map(|idx| node_names[idx])
                .collect::<Vec<_>>();
            clique_names.sort_unstable();

            return clique_names.join(",");
        }

        String::new()
    }
}

fn solve(data: &str) -> (usize, String) {
    let puzzle = Puzzle::new(data);
    (puzzle.part1(), puzzle.part2())
}

fn main() {
    let mut args = aoc::parse_args();
    args.run(solve);
}

/// Test from puzzle input
#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = include_str!("test.txt");

    #[test]
    fn test_part1() {
        let puzzle = Puzzle::new(TEST_INPUT);
        assert_eq!(puzzle.part1(), 7);
    }

    #[test]
    fn test_part2() {
        let puzzle = Puzzle::new(TEST_INPUT);
        assert_eq!(puzzle.part2(), "co,de,ka,ta");
    }
}
