//! [Day 14: Chocolate Charts](https://adventofcode.com/2018/day/14)

fn score(recipes: &str) -> String {
    let iterations: usize = recipes.trim_ascii().parse().unwrap();

    let mut elf_one = 0;
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
        .trim_ascii()
        .chars()
        .map(|c| u8::try_from(c.to_digit(10).unwrap()).unwrap())
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

/// # Panics
/// over malformed input
#[must_use]
pub fn solve(data: &str) -> (String, usize) {
    (score(data), appear(data))
}

pub fn main() {
    let args = aoc::parse_args();
    args.run(solve);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test01() {
        assert_eq!(score("9"), "5158916779");
    }

    #[test]
    fn test02() {
        assert_eq!(score("5"), "0124515891");
    }

    #[test]
    fn test03() {
        assert_eq!(score("18"), "9251071085");
    }

    #[test]
    fn test04() {
        assert_eq!(score("2018"), "5941429882");
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
