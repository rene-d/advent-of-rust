//! [Day 5: Doesn't He Have Intern-Elves For This?](https://adventofcode.com/2015/day/5)

pub fn main() {
    let args = aoc::parse_args();
    args.run(solve);
}

/// # Panics
#[must_use]
pub fn solve(data: &str) -> (u32, u32) {
    (part1(data), part2(data))
}

fn part2(data: &str) -> u32 {
    let mut nice_words = 0;
    for word in data.lines() {
        // pair of any two letters that appears at least twice in the string without overlapping
        let mut twice = false;
        for i in 0..word.len() - 3 {
            let c = word.chars().nth(i).unwrap();
            let d = word.chars().nth(i + 1).unwrap();

            for j in i + 2..word.len() - 1 {
                if c == word.chars().nth(j).unwrap() && d == word.chars().nth(j + 1).unwrap() {
                    twice = true;
                    break;
                }
            }

            if twice {
                break;
            }
        }

        if !twice {
            continue;
        }

        // letter which repeats with exactly one letter between them
        for i in 0..word.len() - 2 {
            if word.chars().nth(i).unwrap() == word.chars().nth(i + 2).unwrap() {
                nice_words += 1;
                break;
            }
        }
    }

    nice_words
}

fn part1(data: &str) -> u32 {
    let mut nice_words = 0;
    for word in data.lines() {
        if word.contains("ab") || word.contains("cd") || word.contains("pq") || word.contains("xy")
        {
            continue;
        }

        let mut vowel = 0;
        for c in word.chars() {
            if "aeiou".contains(c) {
                vowel += 1;
            }
        }
        if vowel < 3 {
            continue;
        }

        // finally, check the repeating letters
        for i in 0..word.len() - 1 {
            if word.chars().nth(i).unwrap() == word.chars().nth(i + 1).unwrap() {
                nice_words += 1;
                break;
            }
        }
    }

    nice_words
}
