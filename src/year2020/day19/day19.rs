//! [Day 19: Monster Messages](https://adventofcode.com/2020/day/19)

use rustc_hash::FxHashMap;

enum Rule {
    Ch(char),
    Id(Vec<Vec<u32>>),
}

struct Rules {
    r: FxHashMap<u32, Rule>,
}

impl Rules {
    fn new(data: &str) -> Self {
        let mut r = FxHashMap::default();

        for line in data.lines() {
            let mut line = line.split(": ");

            let id: u32 = line.next().unwrap().parse().unwrap();

            let arg = line.next().unwrap();

            if let Some(arg) = arg.strip_prefix('"') {
                if let Some(arg) = arg.strip_suffix('"') {
                    let rule = Rule::Ch(arg.chars().nth(0).unwrap());

                    r.insert(id, rule);
                }
            } else {
                let mut subids = vec![];

                for subset in arg.split(" | ") {
                    let ids = subset
                        .split(' ')
                        .map(|x| x.parse::<u32>().unwrap())
                        .collect();
                    subids.push(ids);
                }

                r.insert(id, Rule::Id(subids));
            }
        }

        Self { r }
    }

    fn matches(&self, rule_id: u32, message: &str) -> bool {
        let mut seen: FxHashMap<(usize, usize, u32), bool> = FxHashMap::default();

        self.match_rule(rule_id, message, 0, message.len(), &mut seen)
    }

    fn match_rule(
        &self,
        rule_id: u32,
        message: &str,
        start: usize,
        end: usize,
        seen: &mut FxHashMap<(usize, usize, u32), bool>,
    ) -> bool {
        let key = (start, end, rule_id);
        if seen.contains_key(&key) {
            return seen[&key];
        }

        let rule = &self.r[&rule_id];

        let ret = match rule {
            Rule::Ch(ch) => (end == start + 1) && message.chars().nth(start).unwrap() == *ch,

            Rule::Id(rule_sets) => rule_sets
                .iter()
                .any(|rule_set| self.match_list(rule_set, message, start, end, seen)),
        };

        seen.insert(key, ret);
        ret
    }

    fn match_list(
        &self,
        rule_set: &[u32],
        message: &str,
        start: usize,
        end: usize,
        seen: &mut FxHashMap<(usize, usize, u32), bool>,
    ) -> bool {
        if start == end && rule_set.is_empty() {
            return true;
        }

        if start == end || rule_set.is_empty() {
            return false;
        }

        for i in (start + 1)..=end {
            if i == end && rule_set.len() > 1 {
                continue;
            }

            if self.match_rule(rule_set[0], message, start, i, seen)
                && self.match_list(&rule_set[1..], message, i, end, seen)
            {
                return true;
            }
        }

        false
    }

    fn solve(&self, messages: &[String]) -> usize {
        messages
            .iter()
            .filter(|message| self.matches(0, message))
            .count()
    }
}

struct Puzzle {
    rules: Rules,
    messages: Vec<String>,
}

impl Puzzle {
    fn new(data: &str) -> Self {
        let (rules, messages) = data.split_once("\n\n").unwrap();

        Self {
            rules: Rules::new(rules),
            messages: messages.lines().map(str::to_string).collect(),
        }
    }

    /// Solve part one.
    fn part1(&self) -> usize {
        self.rules.solve(&self.messages)
    }

    /// Solve part two.
    fn part2(&mut self) -> usize {
        self.rules
            .r
            .insert(8, Rule::Id(vec![vec![42], vec![42, 8]]));

        self.rules
            .r
            .insert(11, Rule::Id(vec![vec![42, 31], vec![42, 11, 31]]));

        self.rules.solve(&self.messages)
    }
}

/// # Panics
/// over malformed input
#[must_use]
pub fn solve(data: &str) -> (usize, usize) {
    let mut puzzle = Puzzle::new(data);
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
    const TEST_INPUT_2: &str = include_str!("test2.txt");

    #[test]
    fn test01() {
        let puzzle = Puzzle::new(TEST_INPUT);
        assert_eq!(puzzle.part1(), 2);
    }

    #[test]
    fn test02() {
        let mut puzzle = Puzzle::new(TEST_INPUT_2);
        assert_eq!(puzzle.part2(), 12);
    }
}
