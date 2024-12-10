use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};

use crate::mazecell::MazeCell;
use crate::path::Path;
use crate::state::State;

use aoc::grid::Grid;

pub trait Dijkstra {
    fn search(&self) -> usize;
}

/// Calculate distances to keys and seen keys and doors from the given cell of the maze.
fn get_paths_from(maze: &Grid<u8>, start: (usize, usize)) -> Vec<Path> {
    let mut paths = vec![];

    let mut seen = HashSet::new();
    let mut q = VecDeque::new();

    q.push_back((start, 0, 0, 0));
    while let Some((pos, steps, mut keys_set, mut doors_set)) = q.pop_front() {
        //
        if pos != start && maze[pos].is_key() {
            paths.push(Path {
                destination: maze[pos],
                steps,
                keys: keys_set,
                doors: doors_set,
            });
        }

        for np in maze.iter_directions(pos) {
            let c = maze[np];

            if !c.is_wall() && !seen.contains(&np) {
                seen.insert(np);

                if c.is_key() {
                    keys_set |= 1 << (c - b'a');
                }
                if c.is_door() {
                    doors_set |= 1 << (c - b'A');
                }

                q.push_back((np, steps + 1, keys_set, doors_set));
            }
        }
    }

    paths
}

fn get_all_paths(maze: &Grid<u8>) -> (u8, HashMap<u8, Vec<Path>>) {
    let mut all_paths = HashMap::new();

    let mut num_robots = 0;

    for (xy, c) in maze.iter() {
        let mut c = *c;

        if c.is_entrance() || c.is_key() {
            if c.is_entrance() {
                num_robots += 1;
                c = b'0' + num_robots;
            }

            all_paths.insert(c, get_paths_from(maze, xy));
        }
    }

    (num_robots, all_paths)
}

impl Dijkstra for Grid<u8> {
    fn search(&self) -> usize {
        let all_keys = self
            .iter()
            .filter(|(_, c)| c.is_key())
            .fold(0, |all_keys, (_, c)| all_keys | 1 << (c - b'a'));

        let (num_robots, all_paths) = get_all_paths(self);

        let state = State::new(num_robots);

        let mut scores = HashMap::new();
        scores.insert(state.clone(), 0usize);

        let mut closed = HashSet::new();

        let mut open = HashSet::new();
        open.insert(state.clone());

        let mut q = BinaryHeap::new();
        q.push((Reverse(0), state));

        while let Some((_, current)) = q.pop() {
            if current.keys == all_keys {
                return *scores.get(&current).unwrap();
            }

            open.remove(&current);
            closed.insert(current.clone());

            for (neighbor, steps) in current.next(&all_paths) {
                if closed.contains(&neighbor) {
                    continue;
                }

                let score = scores[&current] + steps;

                if score < *scores.get(&neighbor).unwrap_or(&usize::MAX) {
                    scores.insert(neighbor.clone(), score);

                    if !open.contains(&neighbor) {
                        q.push((Reverse(score), neighbor));
                    }
                }
            }
        }

        0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        let maze = "\
###############
#d.ABC.#.....a#
######@#@######
###############
######@#@######
#b.....#.....c#
###############
";

        let maze = Grid::<u8>::parse(maze);

        assert_eq!(maze.search(), 24);
    }
}
