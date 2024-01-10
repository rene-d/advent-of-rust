//! [Day 8: Treetop Tree House](https://adventofcode.com/2022/day/8)

struct Puzzle {
    nx: usize,
    /// number of columns
    ny: usize,
    /// number of lines
    trees: Vec<Vec<u8>>,
}

impl Puzzle {
    fn new() -> Self {
        Self {
            nx: 0,
            ny: 0,
            trees: vec![],
        }
    }

    /// Loads data from input (one line)
    fn configure(&mut self, path: &str) {
        let data = std::fs::read_to_string(path).unwrap();
        let lines = data.split('\n').collect::<Vec<_>>();

        self.nx = lines.first().unwrap().len();
        for line in lines {
            if line.is_empty() {
                continue;
            }
            let mut row = vec![0; self.nx];
            for (x, tree) in line.chars().enumerate() {
                row[x] = tree.to_digit(10).unwrap() as u8;
            }
            self.trees.push(row);
        }
        self.ny = self.trees.len();

        // println!("{}x{}", self.nx, self.ny);
        // println!("{:?}", self.trees);
    }

    // Solves part one
    fn part1(&self) -> usize {
        let mut visible = 2 * self.nx + 2 * self.ny - 4;

        for y in 1..(self.ny - 1) {
            for x in 1..(self.nx - 1) {
                let tree = self.trees[y][x];

                let visible_d = (y + 1..self.ny).all(|i| self.trees[i][x] < tree);
                let visible_u = (0..y).all(|i| self.trees[i][x] < tree);
                let visible_r = (x + 1..self.nx).all(|i| self.trees[y][i] < tree);
                let visible_l = (0..x).all(|i| self.trees[y][i] < tree);

                if visible_d || visible_u || visible_r || visible_l {
                    visible += 1;
                }
            }
        }

        visible
    }

    // Solve part two
    fn part2(&self) -> usize {
        let mut max_scene = 0;

        for y in 1..(self.ny - 1) {
            for x in 1..(self.nx - 1) {
                let tree = self.trees[y][x];
                let mut scene = 1;
                let mut visibility;

                // max visibility from the tree to the right
                visibility = 0;
                for i in x + 1..self.nx {
                    visibility += 1;
                    if tree <= self.trees[y][i] {
                        break;
                    }
                }
                scene *= visibility;

                // to the left
                visibility = 0;
                for i in (0..x).rev() {
                    visibility += 1;
                    if tree <= self.trees[y][i] {
                        break;
                    }
                }
                scene *= visibility;

                // downwards
                visibility = 0;
                for i in y + 1..self.ny {
                    visibility += 1;
                    if tree <= self.trees[i][x] {
                        break;
                    }
                }
                scene *= visibility;

                // upwards
                visibility = 0;
                for i in (0..y).rev() {
                    visibility += 1;
                    if tree <= self.trees[i][x] {
                        break;
                    }
                }
                scene *= visibility;

                if scene > max_scene {
                    max_scene = scene;
                }
            }
        }

        max_scene
    }
}

/// main function
fn main() {
    let args = aoc::parse_args();
    let mut puzzle = Puzzle::new();
    puzzle.configure(&args.path);
    println!("{}", puzzle.part1());
    println!("{}", puzzle.part2());
}

#[test]
fn test01() {
    let mut puzzle = Puzzle::new();
    puzzle.configure("test.txt");
    assert_eq!(puzzle.part1(), 21);
    assert_eq!(puzzle.part2(), 8);
}
