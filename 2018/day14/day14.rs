//! [Day 14: Chocolate Charts](https://adventofcode.com/2018/day/14)

fn score(iterations: usize) -> String {
    let mut elf_one: usize = 0;
    let mut elf_two = 1;

    let mut scores = [3u8, 7u8].to_vec();

    for _ in 0.. {
        let n = scores[elf_one] + scores[elf_two];

        if n >= 10 {
            scores.push(1);
            scores.push(n % 10);
        } else {
            scores.push(n);
        }

        elf_one = (elf_one + 1 + (scores[elf_one] as usize)) % scores.len();
        elf_two = (elf_two + 1 + (scores[elf_two] as usize)) % scores.len();

        // for (i, score) in scores.iter().enumerate() {
        //     if i == elf_one {
        //         print!("({score})",);
        //     } else if i == elf_two {
        //         print!("[{score}]");
        //     } else {
        //         print!(" {score} ");
        //     }
        // }
        // println!();

        if scores.len() >= iterations + 10 {
            let r: String = scores[iterations..(iterations + 10)]
                .iter()
                .map(u8::to_string)
                .collect();
            return r;
        }
    }

    unreachable!();
}

fn appear(recipes: &str) -> usize {
    let mut elf_one: usize = 0;
    let mut elf_two = 1;

    let score: Vec<u8> = recipes
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect();

    let score = score.as_slice();
    let width = score.len();

    let mut scores = [3u8, 7u8].to_vec();

    for _ in 0.. {
        let n = scores[elf_one] + scores[elf_two];

        if n >= 10 {
            scores.push(1);
            scores.push(n % 10);
        } else {
            scores.push(n);
        }

        elf_one = (elf_one + 1 + (scores[elf_one] as usize)) % scores.len();
        elf_two = (elf_two + 1 + (scores[elf_two] as usize)) % scores.len();

        let length = scores.len();
        if length >= width && scores[(length - width)..] == score[..] {
            return length - width;
        }
        if length > width && scores[(length - width - 1)..(length - 1)] == score[..] {
            return length - width - 1;
        }
    }
    0
}

struct Puzzle {
    recipes: String,
}

impl Puzzle {
    fn new() -> Puzzle {
        Puzzle {
            recipes: String::new(),
        }
    }

    /// Get the puzzle input.
    fn configure(&mut self, path: &str) {
        let data = std::fs::read_to_string(path).unwrap();

        self.recipes = data.trim().to_owned();
    }

    /// Solve part one.
    fn part1(&self) -> String {
        score(self.recipes.parse().unwrap())
    }

    /// Solve part two.
    fn part2(&self) -> usize {
        appear(&self.recipes)
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
        assert_eq!(score(9), "5158916779");
    }

    #[test]
    fn test02() {
        assert_eq!(score(5), "0124515891");
    }

    #[test]
    fn test03() {
        assert_eq!(score(18), "9251071085");
    }

    #[test]
    fn test04() {
        assert_eq!(score(2018), "5941429882");
    }

    #[test]
    fn test05() {
        assert_eq!(appear("51589"), 9);
    }

    #[test]
    fn test06() {
        assert_eq!(appear("01245"), 5);
    }

    #[test]
    fn test07() {
        assert_eq!(appear("92510"), 18);
    }

    #[test]
    fn test08() {
        assert_eq!(appear("59414"), 2018);
    }
}
