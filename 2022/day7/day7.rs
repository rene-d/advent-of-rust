//! [Day 7: No Space Left On Device](https://adventofcode.com/2022/day/7)

#![allow(clippy::if_same_then_else)]

use clap::Parser;
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Parser)]
struct Args {
    /// Puzzle input
    #[arg(default_value = "input.txt")]
    path: String,
}

struct Puzzle {
    // Puzzle input
    total_dir_size: HashMap<String, usize>,
}

impl Puzzle {
    fn new() -> Self {
        Self {
            // data: String::new(),
            total_dir_size: HashMap::new(),
        }
    }

    /// Loads data from input (one line)
    fn configure(&mut self, path: &str) {
        let data = std::fs::read_to_string(path).unwrap();
        let lines = data.lines().collect::<Vec<_>>();

        let mut dir_size = HashMap::new();
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
            } else if line == "$ ls" {
                //  nothing to do
            } else if line.starts_with("dir ") {
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

            self.total_dir_size.insert(dir.clone(), total);
        }
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
            if total - size + 30000000 <= 70000000 {
                return *size;
            }
        }

        0
    }
}

/// main function
fn main() {
    let args = Args::parse();
    let mut puzzle = Puzzle::new();
    puzzle.configure(&args.path);
    println!("{}", puzzle.part1());
    println!("{}", puzzle.part2());
}

#[test]
fn test01() {
    let mut puzzle = Puzzle::new();
    puzzle.configure("test.txt");
    assert_eq!(puzzle.part1(), 95437);
    assert_eq!(puzzle.part2(), 24933642);
}
