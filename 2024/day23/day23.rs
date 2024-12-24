//! [Day 23: LAN Party](https://adventofcode.com/2024/day/23)

use petgraph::graph::{NodeIndex, UnGraph};
use std::collections::{HashMap, HashSet};

// Bron-Kerbosch recursive algorithm to find cliques
fn bron_kerbosch(
    graph: &UnGraph<(), ()>,
    r: &HashSet<NodeIndex>,
    p: &mut HashSet<NodeIndex>,
    x: &mut HashSet<NodeIndex>,
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

        let neighbors: HashSet<_> = graph.neighbors(v).collect();
        let mut p_new: HashSet<NodeIndex> = p.intersection(&neighbors).copied().collect();
        let mut x_new: HashSet<NodeIndex> = x.intersection(&neighbors).copied().collect();

        bron_kerbosch(graph, &r_new, &mut p_new, &mut x_new, cliques);

        p.remove(&v);
        x.insert(v);
    }
}

struct Puzzle {
    connections: Vec<(String, String)>,
}

impl Puzzle {
    const fn new() -> Self {
        Self {
            connections: Vec::new(),
        }
    }

    /// Get the puzzle input.
    fn configure(&mut self, path: &str) {
        let data = std::fs::read_to_string(path).unwrap_or_else(|_| {
            eprintln!("cannot read input file {path}");
            std::process::exit(1);
        });

        for line in data.lines() {
            if let Some((from, to)) = line.split_once('-') {
                self.connections.push((from.to_string(), to.to_string()));
            }
        }
    }

    /// Solve part one.
    fn part1(&self) -> usize {
        let mut graph: HashMap<String, HashSet<String>> = HashMap::new();
        let mut triangles: HashSet<[&String; 3]> = HashSet::new();

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

        let mut nodes = HashMap::new();

        for (n1, n2) in &self.connections {
            let i1 = *nodes
                .entry(n1.clone())
                .or_insert_with(|| graph.add_node(()));
            let i2 = *nodes
                .entry(n2.clone())
                .or_insert_with(|| graph.add_node(()));
            graph.add_edge(i1, i2, ());
        }

        let mut node_names: HashMap<&NodeIndex, &str> = HashMap::new();
        for (k, v) in &nodes {
            node_names.insert(v, k);
        }

        // find the largest clique
        let mut cliques = Vec::new();
        let r = HashSet::new();
        let mut p: HashSet<NodeIndex> = graph.node_indices().collect();
        let mut x = HashSet::new();

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

fn main() {
    let args = aoc::parse_args();
    let mut puzzle = Puzzle::new();
    puzzle.configure(args.path.as_str());
    println!("{}", puzzle.part1());
    println!("{}", puzzle.part2());
}

/// Test from puzzle input
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let mut puzzle = Puzzle::new();
        puzzle.configure("test.txt");
        assert_eq!(puzzle.part1(), 7);
    }

    #[test]
    fn test_part2() {
        let mut puzzle = Puzzle::new();
        puzzle.configure("test.txt");
        assert_eq!(puzzle.part2(), "co,de,ka,ta");
    }
}
