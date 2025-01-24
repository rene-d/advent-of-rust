//! [Day 7: No Space Left On Device](https://adventofcode.com/2022/day/7)

use rustc_hash::FxHashMap;
use std::path::PathBuf;

struct Puzzle {
    total_dir_size: FxHashMap<String, usize>,
}

impl Puzzle {
    fn new(data: &str) -> Self {
        let mut total_dir_size = FxHashMap::default();

        let lines = data.lines().collect::<Vec<_>>();

        let mut dir_size = FxHashMap::default();
        let mut current_path = PathBuf::from("/");

        dir_size.entry("/".to_string()).or_insert(0);

        for line in lines {
            if line == "$ cd /" {
                current_path = PathBuf::from("/");
            } else if line == "$ cd .." {
                assert_ne!(current_path.as_path().as_os_str().len(), 1);
                current_path.pop();
            } else if let Some(dir) = line.strip_prefix("$ cd ") {
                current_path.push(dir);

                let key = current_path.as_path().to_str().unwrap().to_string();
                dir_size.entry(key).or_insert(0);
            } else if line == "$ ls" || line.starts_with("dir ") {
                //  nothing to do
            } else if !line.is_empty() {
                // <size> <filename>
                let mut info = line.split(' ');
                let size = info.next().unwrap().parse::<usize>().unwrap();
                // let filename = info.next().unwrap();

                let key = current_path.as_path().to_str().unwrap().to_string();
                *dir_size.get_mut(&key).unwrap() += size;
            }
        }

        for dir in dir_size.keys() {
            let total = dir_size
                .iter()
                .map(|(x, size)| {
                    if dir.len() == 1
                        || x.starts_with(dir)
                            && (x.len() == dir.len() || {
                                x.chars().nth(dir.len()).unwrap() == std::path::MAIN_SEPARATOR
                            })
                    {
                        size
                    } else {
                        &0
                    }
                })
                .sum();

            total_dir_size.insert(dir.clone(), total);
        }

        Self { total_dir_size }
    }

    // Solves part one
    fn part1(&self) -> usize {
        let mut result = 0;
        for dir in &self.total_dir_size {
            if dir.1 <= &100_000 {
                result += dir.1;
            }
        }
        result
    }

    // Solve part two
    fn part2(&self) -> usize {
        let total = self.total_dir_size.get("/").unwrap();

        let mut sizes = self.total_dir_size.values().collect::<Vec<_>>();
        sizes.sort_unstable();

        for size in sizes {
            if total - size + 30_000_000 <= 70_000_000 {
                return *size;
            }
        }

        0
    }
}

#[must_use]
pub fn solve(data: &str) -> (usize, usize) {
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
    fn test01() {
        let puzzle = Puzzle::new(TEST_INPUT);
        assert_eq!(puzzle.part1(), 95437);
        assert_eq!(puzzle.part2(), 24_933_642);
    }
}
