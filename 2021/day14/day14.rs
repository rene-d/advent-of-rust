//! [Day 14: Extended Polymerization](https://adventofcode.com/2021/day/14)

use rustc_hash::FxHashMap;
use rustc_hash::FxHashSet;

#[derive(Debug)]
struct Rule {
    generated: (String, String),
    output: char,
}

impl Rule {
    const fn new() -> Self {
        Self {
            generated: (String::new(), String::new()),
            output: '\0',
        }
    }
}

struct Puzzle {
    template: String,
    generator: FxHashMap<String, Rule>,
    elements: FxHashSet<char>,
    steps: u64,
}

impl Puzzle {
    fn new() -> Self {
        Self {
            template: String::new(),
            generator: FxHashMap::default(),
            elements: FxHashSet::default(),
            steps: 0,
        }
    }

    fn configure(&mut self, data: &str) {
        let mut data: Vec<String> = data.lines().rev().map(ToString::to_string).collect();

        self.template = data.pop().unwrap();
        data.pop();

        for c in self.template.chars() {
            self.elements.insert(c);
        }

        for insertion_rule in data {
            let mut halves = insertion_rule.split(" -> ");
            let first = halves.next().unwrap().to_string();
            let first_elem = (first.chars().next().unwrap(), first.chars().nth(1).unwrap());
            let second = halves.next().unwrap().parse::<char>().unwrap();
            self.elements.insert(second);
            self.elements.insert(first_elem.0);
            self.elements.insert(first_elem.1);

            let mut generated = Rule::new();
            generated.generated.0.push(first_elem.0);
            generated.generated.0.push(second);
            generated.generated.1.push(second);
            generated.generated.1.push(first_elem.1);
            generated.output = second;
            self.generator.insert(first, generated);
        }
    }

    fn part1(&self) -> u64 {
        let mut polymer = self.template.clone();
        for _ in 0..self.steps {
            let mut polymer_new = String::new();
            for index in 0..polymer.len() - 1 {
                let slice = polymer.get(index..=index + 1).unwrap();
                polymer_new.push(slice.chars().next().unwrap());
                if let Some(rule) = self.generator.get(slice) {
                    polymer_new.push(rule.output);
                }
            }
            polymer_new.push(polymer.chars().last().unwrap());
            polymer.clone_from(&polymer_new);
        }

        let mut min = u64::MAX;
        let mut max = u64::MIN;
        for c in &self.elements {
            let count = polymer.chars().filter(|x| *x == *c).count() as u64;
            if count > max {
                max = count;
            }
            if count < min {
                min = count;
            }
        }

        max - min
    }

    fn part2(&self) -> u64 {
        let mut elements_count = FxHashMap::default();
        let mut generators_count = FxHashMap::default();

        for c in &self.elements {
            elements_count.insert(*c, 0_u64);
        }

        for g in &self.generator {
            generators_count.insert(g.0.clone(), 0_u64);
        }

        for index in 0..self.template.len() - 1 {
            let slice = self.template.get(index..=index + 1).unwrap();
            if let Some(count) = generators_count.get_mut(slice) {
                *count += 1;
            }
        }

        for c in self.template.chars() {
            let count = elements_count.get_mut(&c).unwrap();
            *count += 1;
        }

        for _ in 0..self.steps {
            let mut generators_new = FxHashMap::default();
            for rule in &self.generator {
                generators_new.insert(rule.0.clone(), 0_u64);
            }

            for rule_count in &generators_count {
                let rule = self.generator.get(rule_count.0).unwrap();
                if let Some(count_new) = generators_new.get_mut(&rule.generated.0) {
                    *count_new += *rule_count.1;
                }
                if let Some(count_new) = generators_new.get_mut(&rule.generated.1) {
                    *count_new += *rule_count.1;
                }
                let count = elements_count.get_mut(&rule.output).unwrap();
                *count += *rule_count.1;
            }

            for g in &mut generators_count {
                let count_new = generators_new.get(g.0).unwrap();
                *g.1 = *count_new;
            }
        }

        let mut min = u64::MAX;
        let mut max = u64::MIN;
        for elem in &elements_count {
            if *elem.1 != 0 && *elem.1 > max {
                max = *elem.1;
            }
            if *elem.1 != 0 && *elem.1 < min {
                min = *elem.1;
            }
        }

        max - min
    }
}

fn main() {
    let args = aoc::parse_args();

    let mut puzzle = Puzzle::new();

    puzzle.configure(&args.input);

    puzzle.steps = 10;
    let result = puzzle.part1();
    println!("{result}");

    puzzle.steps = 40;
    let result = puzzle.part2();
    println!("{result}");
}

/// Test from puzzle input
#[test]
fn test01() {
    let mut puzzle = Puzzle::new();
    puzzle.configure(&aoc::load_input_data("test01.txt"));

    puzzle.steps = 10;
    assert_eq!(puzzle.part1(), 1588);
    assert_eq!(puzzle.part2(), 1588);

    puzzle.steps = 40;
    assert_eq!(puzzle.part2(), 2_188_189_693_529);
}

/// Test from a user's input
#[test]
fn test02() {
    let mut puzzle = Puzzle::new();
    puzzle.configure(&aoc::load_input_data("test02.txt"));

    puzzle.steps = 10;
    assert_eq!(puzzle.part1(), 3058);
    assert_eq!(puzzle.part2(), 3058);

    puzzle.steps = 40;
    assert_eq!(puzzle.part2(), 3_447_389_044_530);
}
