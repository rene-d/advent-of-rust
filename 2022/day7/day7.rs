//! [Day 7: No Space Left On Device](https://adventofcode.com/2022/day/7)

use clap::Parser;
use std::collections::{HashSet, HashMap};
use std::path::PathBuf;

#[derive(Parser)]
struct Args {
    /// Puzzle input
    path: String,
}

struct Puzzle {
    // Puzzle input
    dirs_size : HashMap<String, usize>,
}

impl Puzzle {
    fn new() -> Self {
        Self {
            // data: String::new(),
            dirs_size : HashMap::new(),
        }
    }

    /// Loads data from input (one line)
    fn configure(&mut self, path: &str) {
        let data = std::fs::read_to_string(path).unwrap();
        let lines = data.split('\n').collect::<Vec<_>>();

        let mut path = PathBuf::from("/");

        let mut dirs = HashSet::new();
        let mut files = HashMap::new();

        for line in lines {
            if line == "$ cd /" {
                path = PathBuf::from("/");
            } else if line == "$ cd .." {
                path.pop();
            } else if line.starts_with("$ cd ") {
                path.push(&line[5..]);
            } else if line == "$ ls" {
                // ignore
            } else if line.starts_with("dir ") {
                // ignore
            } else if !line.is_empty() {
                let mut info = line.split(' ');
                let size = info.next().unwrap().parse::<usize>().unwrap();
                let name = info.next().unwrap();

                path.push(name);
                files.insert(path.as_path().to_str().unwrap().to_string() ,size);
                path.pop();
            }

            dirs.insert(path.as_path().to_str().unwrap().to_string());
        }


        for dir in dirs {
            let mut dir_size = 0;
            for file in &files {
                if file.0.starts_with(&dir) {
                    dir_size += file.1;
                }
            }
            self.dirs_size.insert(dir, dir_size);
        }

    }

    // Solves part one
    fn part1(&self) -> usize {
        let mut result = 0;
        for dir in &self.dirs_size {
            if dir.1 <= &100000 {
                result += dir.1;
            }
        }
        result
    }

    // Solve part two
    fn part2(&self) -> usize {
        let total = self.dirs_size.get("/").unwrap();


        let mut sizes = vec![];
        for dir in &self.dirs_size {
            sizes.push(dir.1);
        }
        sizes.sort();

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
