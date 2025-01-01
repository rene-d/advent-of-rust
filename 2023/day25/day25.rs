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
    fn new() -> Self {
        Self {
            graph: G::new_undirected(),
        }
    }

    /// Get the puzzle input.
    fn configure(&mut self, data: &str) {
        let mut nodes = FxHashMap::default();

        for line in data.lines() {
            let (node, cnx) = line.split_once(": ").unwrap();
            let connections = cnx.split_ascii_whitespace().collect::<Vec<_>>();

            let node_index = *nodes.entry(node).or_insert_with(|| self.graph.add_node(()));

            for connection in connections {
                let connection_index = *nodes
                    .entry(connection)
                    .or_insert_with(|| self.graph.add_node(()));

                self.graph.add_edge(node_index, connection_index, ());
            }
        }
    }

    /// Solve part one.
    fn part1(&self) -> usize {
        let min_cut = stoer_wagner_min_cut(&self.graph, |_| Ok::<u32, ()>(1));

        let (_, edges) = min_cut.unwrap().unwrap();

        (self.graph.node_count() - edges.len()) * edges.len()
    }
}

fn main() {
    let args = aoc::parse_args();
    let mut puzzle = Puzzle::new();
    puzzle.configure(&args.input);
    println!("{}", puzzle.part1());
}

/// Test from puzzle input
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test01() {
        let mut puzzle = Puzzle::new();
        puzzle.configure(&aoc::load_input_data("test.txt"));
        assert_eq!(puzzle.part1(), 54);
    }
}
