//! [Day 15: Lens Library](https://adventofcode.com/2023/day/15)

fn hash_algo(s: &str) -> u32 {
    s.chars()
        .fold(0, |value, c| ((value + u32::from(c)) * 17) % 256)
}

struct Puzzle<'a> {
    data: &'a str,
}

impl<'a> Puzzle<'a> {
    const fn new(data: &'a str) -> Self {
        Self {
            data: data.trim_ascii(),
        }
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

/// # Panics
#[must_use]
pub fn solve(data: &str) -> (u32, u32) {
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
        assert_eq!(puzzle.part1(), 1320);
    }

    #[test]
    fn test02() {
        let puzzle = Puzzle::new(TEST_INPUT);
        assert_eq!(puzzle.part2(), 145);
    }
}
