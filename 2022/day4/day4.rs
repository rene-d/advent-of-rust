//! [Day 4: Camp Cleanup](https://adventofcode.com/2022/day/4)

struct Puzzle {
    assignment_pairs: Vec<(u32, u32, u32, u32)>,
}

impl Puzzle {
    fn new() -> Puzzle {
        Puzzle {
            assignment_pairs: Vec::new(),
        }
    }

    fn configure(&mut self, path: &str) {
        let data = std::fs::read_to_string(path).unwrap();
        let mut lines = data.split('\n').collect::<Vec<_>>();
        lines.pop();

        for assignment_pair in lines {
            let (assignment_left, assignment_right) = assignment_pair.split_once(',').unwrap();
            let (left_id_first, left_id_last) = assignment_left.split_once('-').unwrap();
            let (right_id_first, right_id_last) = assignment_right.split_once('-').unwrap();
            self.assignment_pairs.push((
                left_id_first.parse().unwrap(),
                left_id_last.parse().unwrap(),
                right_id_first.parse().unwrap(),
                right_id_last.parse().unwrap(),
            ));
        }
    }

    fn assignment_contained(a: &(u32, u32, u32, u32)) -> bool {
        a.0 >= a.2 && a.1 <= a.3 || a.0 <= a.2 && a.1 >= a.3
    }

    fn assignment_overlapped(a: &(u32, u32, u32, u32)) -> bool {
        Puzzle::assignment_contained(a) || a.0 >= a.2 && a.0 <= a.3 || a.1 >= a.2 && a.1 <= a.3
    }

    fn part1(&self) -> u32 {
        let mut result = 0;
        for assignment in &self.assignment_pairs {
            if Puzzle::assignment_contained(assignment) {
                result += 1;
            }
        }
        result
    }

    fn part2(&self) -> u32 {
        let mut result = 0;
        for assignment in &self.assignment_pairs {
            if Puzzle::assignment_overlapped(assignment) {
                result += 1;
            }
        }
        result
    }
}

/// Solve the puzzle with the user input
fn main() {
    let mut puzzle = Puzzle::new();
    puzzle.configure("test02.txt");
    println!("{}", puzzle.part1());
    println!("{}", puzzle.part2());
}

/// Test from puzzle input
#[test]
fn test01() {
    let mut puzzle = Puzzle::new();
    puzzle.configure("test01.txt");
    assert_eq!(puzzle.part1(), 2);
    assert_eq!(puzzle.part2(), 4);
}

/// Test from user input
#[test]
fn test02() {
    let mut puzzle = Puzzle::new();
    puzzle.configure("test02.txt");
    assert_eq!(puzzle.part1(), 556);
    assert_eq!(puzzle.part2(), 876);
}
