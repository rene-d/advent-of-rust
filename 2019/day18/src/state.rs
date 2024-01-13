use crate::{mazecell::MazeCell, path::Path};
use std::collections::HashMap;

#[derive(Hash, Eq, PartialEq, Ord, PartialOrd, Clone)]
pub struct State {
    pub keys: u32,
    robots: Vec<u8>,
}

impl State {
    #[must_use]
    pub fn new(num_robots: u8) -> Self {
        Self {
            keys: 0,
            robots: (b'1'..=(b'0' + num_robots)).collect(),
        }
    }

    /// Find the next states
    /// # Panics
    /// may be...
    #[must_use]
    pub fn next(&self, paths: &HashMap<u8, Vec<Path>>) -> Vec<(State, usize)> {
        // impl Iterator<Item = Self> {

        let mut result = vec![];

        for (index, pos) in self.robots.iter().enumerate() {
            for path in paths.get(pos).unwrap() {
                // we already have the key
                if path.destination.is_key() && self.keys & (1 << (path.destination - b'a')) != 0 {
                    continue;
                }

                // not all doors of path are open
                if path.doors & self.keys != path.doors {
                    continue;
                }

                let mut robots = self.robots.clone();
                robots[index] = path.destination;

                result.push((
                    Self {
                        keys: self.keys | path.keys,
                        robots,
                    },
                    path.steps,
                ));
            }
        }
        result
    }
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut keys = String::new();
        let robots: String = self.robots.iter().map(|c| *c as char).collect();

        for i in 0..26 {
            if self.keys & (1 << i) != 0 {
                keys.push((i + b'a') as char);
            }
        }
        write!(f, "State[keys:{keys} robots:{robots}]")
    }
}
