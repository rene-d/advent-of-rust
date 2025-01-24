//! [Day 25: Snowverload](https://adventofcode.com/2023/day/25)

use rustc_hash::FxHashMap;
use rustworkx_core::connectivity::stoer_wagner_min_cut;
use rustworkx_core::petgraph::graph::Graph;
use rustworkx_core::petgraph::Undirected;

type G = Graph<(), (), Undirected>;

struct Puzzle {
    graph: G,
}

impl Puzzle {
    fn new(data: &str) -> Self {
        let mut graph = G::new_undirected();

        let mut nodes = FxHashMap::default();

        for line in data.lines() {
            let (node, cnx) = line.split_once(": ").unwrap();
            let connections = cnx.split_ascii_whitespace().collect::<Vec<_>>();

            let node_index = *nodes.entry(node).or_insert_with(|| graph.add_node(()));

            for connection in connections {
                let connection_index = *nodes
                    .entry(connection)
                    .or_insert_with(|| graph.add_node(()));

                graph.add_edge(node_index, connection_index, ());
            }
        }

        Self { graph }
    }

    /// Solve part one.
    fn part1(&self) -> usize {
        let min_cut = stoer_wagner_min_cut(&self.graph, |_| Ok::<u32, ()>(1));

        let (_, edges) = min_cut.unwrap().unwrap();

        (self.graph.node_count() - edges.len()) * edges.len()
    }
}

/// # Panics
#[must_use]
pub fn solve(data: &str) -> (usize, aoc::Christmas) {
    let puzzle = Puzzle::new(data);
    (puzzle.part1(), aoc::CHRISTMAS)
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
    fn test01() {
        let puzzle = Puzzle::new(TEST_INPUT);
        assert_eq!(puzzle.part1(), 54);
    }
}
