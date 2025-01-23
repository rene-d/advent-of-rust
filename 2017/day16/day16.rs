//! [Day 16: Permutation Promenade](https://adventofcode.com/2017/day/16)

struct Puzzle {
    program: Vec<String>,
}

impl Puzzle {
    fn new(data: &str) -> Self {
        Self {
            program: data
                .trim()
                .split(',')
                .map(std::string::ToString::to_string)
                .collect(),
        }
    }

    fn dance(&self, text: &mut [u8]) {
        for dance in &self.program {
            if let Some(s) = dance.strip_prefix('s') {
                let s = s.parse().unwrap();
                text.rotate_right(s);
            } else if let Some(x) = dance.strip_prefix('x') {
                let (a, b) = x.split_once('/').unwrap();
                let a = a.parse().unwrap();
                let b = b.parse().unwrap();

                text.swap(a, b);
            } else if let Some(p) = dance.strip_prefix('p') {
                let (a, b) = p.split_once('/').unwrap();

                let pos = |ch: u8| text.iter().position(|c| c == &ch).unwrap();

                let a = pos(a.as_bytes()[0]);
                let b = pos(b.as_bytes()[0]);

                text.swap(a, b);
            }
        }
    }

    /// Solve part one.
    fn part1(&self) -> String {
        let mut text: Vec<u8> = "abcdefghijklmnop".bytes().collect();

        self.dance(&mut text);

        text.iter().map(|b| *b as char).collect::<String>()
    }

    /// Solve part two.
    fn part2(&self) -> String {
        let mut text: Vec<u8> = "abcdefghijklmnop".bytes().collect();
        let initial = text.clone();

        let mut seen = vec![];
        seen.push(text.clone());

        for i in 1.. {
            self.dance(&mut text);

            let ok = (0..16).all(|i| initial[i] == text[i]);

            if ok {
                return seen[1_000_000_000 % i]
                    .iter()
                    .map(|b| *b as char)
                    .collect::<String>();
            }

            seen.push(text.clone());
        }
        unreachable!();
    }
}

/// # Panics
/// over malformed input
#[must_use]
pub fn solve(data: &str) -> (String, String) {
    let puzzle = Puzzle::new(data);
    (puzzle.part1(), puzzle.part2())
}

pub fn main() {
    let args = aoc::parse_args();
    args.run(solve);
}
