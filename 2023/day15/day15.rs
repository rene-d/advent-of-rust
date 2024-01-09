//! [Day 15: Lens Library](https://adventofcode.com/2023/day/15)

fn hash_algo(s: &str) -> u32 {
    s.chars()
        .fold(0, |value, c| ((value + u32::from(c)) * 17) % 256)
}

struct Puzzle {
    data: String,
}

impl Puzzle {
    fn new() -> Puzzle {
        Puzzle {
            data: String::new(),
        }
    }

    /// Get the puzzle input.
    fn configure(&mut self, path: &str) {
        let data = std::fs::read_to_string(path).unwrap();

        self.data = data.trim().to_string();
    }

    /// Solve part one.
    fn part1(&self) -> u32 {
        self.data.split(',').map(hash_algo).sum()
    }

    /// Solve part two.
    fn part2(&self) -> u32 {
        let mut boxes: Vec<Vec<(String, u32)>> = vec![];

        boxes.resize(256, Vec::new());

        for s in self.data.split(',') {
            if s.contains('=') {
                let mut s = s.split('=');
                let lens = s.next().unwrap();
                let focal: u32 = s.next().unwrap().parse().unwrap();

                let h = hash_algo(lens) as usize;
                let mut found = false;
                for b in &mut boxes[h] {
                    if b.0 == lens {
                        *b = (lens.to_string(), focal);
                        found = true;
                        break;
                    }
                }
                if !found {
                    boxes[h].push((lens.to_owned(), focal));
                }
            } else if let Some(lens) = s.strip_suffix('-') {
                let h = hash_algo(lens) as usize;

                boxes[h].retain(|b| b.0 != lens);
            } else {
                panic!("bad entry {s}");
            }
        }

        let mut result = 0;
        for (i, b) in boxes.iter().enumerate() {
            for (j, &(_, focal)) in b.iter().enumerate() {
                result += (u32::try_from(i).unwrap() + 1) * (u32::try_from(j).unwrap() + 1) * focal;
            }
        }
        result
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
    fn test01() {
        let mut puzzle = Puzzle::new();
        puzzle.configure("test.txt");
        assert_eq!(puzzle.part1(), 1320);
    }

    #[test]
    fn test02() {
        let mut puzzle = Puzzle::new();
        puzzle.configure("test.txt");
        assert_eq!(puzzle.part2(), 145);
    }
}
