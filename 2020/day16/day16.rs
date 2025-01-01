//! [Day 16: Ticket Translation](https://adventofcode.com/2020/day/16)

use rustc_hash::{FxHashMap, FxHashSet};

use regex::Regex;

#[derive(Debug)]
struct Field {
    name: String,
    a: u32,
    b: u32,
    c: u32,
    d: u32,
}

struct Puzzle {
    verbose: bool,
    fields: Vec<Field>,
    your_tickets: Vec<u32>,
    tickets: Vec<Vec<u32>>,
}

impl Puzzle {
    const fn new() -> Self {
        Self {
            verbose: false,
            fields: Vec::new(),
            your_tickets: Vec::new(),
            tickets: Vec::new(),
        }
    }

    /// Get the puzzle input.
    fn configure(&mut self, data: &str) {
        let data: Vec<&str> = data.split("\n\n").collect();

        let fields_data = data[0];
        let your_tickets = data[1];
        let tickets = data[2];

        let re = Regex::new(r"^([\w ]+): (\d+)-(\d+) or (\d+)-(\d+)$").unwrap();
        for line in fields_data.lines() {
            let caps = re.captures(line).unwrap();
            self.fields.push(Field {
                name: caps[1].to_string(),
                a: caps[2].parse().unwrap(),
                b: caps[3].parse().unwrap(),
                c: caps[4].parse().unwrap(),
                d: caps[5].parse().unwrap(),
            });
        }

        // imperative approach
        // for line in your_tickets.lines().skip(1) {
        //     for s in line.split(',') {
        //         self.your_tickets.push(s.parse().unwrap())
        //     }
        // }

        // functional approach
        self.your_tickets.extend(
            your_tickets
                .lines()
                .skip(1) // skip the line "your ticket:"
                .flat_map(|line| line.split(','))
                .map(|s| s.parse::<u32>().unwrap()),
        );

        self.tickets.extend(
            tickets
                .lines()
                .skip(1) // skip the line "nearby tickets:"
                .map(|line| line.split(',').map(|s| s.parse().unwrap()).collect()),
        );
    }

    /// Solve part one.
    fn part1(&self) -> u32 {
        // println!("{:?}",self.fields);

        let mut error_rate = 0;
        for ticket in &self.tickets {
            for value in ticket {
                if !self
                    .fields
                    .iter()
                    .any(|f| (f.a..=f.b).contains(value) || (f.c..=f.d).contains(value))
                {
                    error_rate += value;
                }
            }
        }
        error_rate
    }

    /// Generic solver for part two and tests.
    fn solve_part2(&self, field_name: &str) -> u64 {
        let mut incompatible = FxHashMap::<usize, FxHashSet<usize>>::default();

        for ticket in &self.tickets {
            if ticket.iter().all(|value| {
                self.fields
                    .iter()
                    .any(|f| (f.a..=f.b).contains(value) || (f.c..=f.d).contains(value))
            }) {
                // good ticket
                for (i, f) in self.fields.iter().enumerate() {
                    for (j, value) in ticket.iter().enumerate() {
                        if !((f.a..=f.b).contains(value) || (f.c..=f.d).contains(value)) {
                            // mark the couple of indices as incompatible
                            // if the value is not valid for the current field
                            incompatible.entry(i).or_default().insert(j);
                        }
                    }
                }
            }
        }

        // ⚠️⚠️⚠️

        // this is an almost exact translation of the algorithm used in Python
        // the only difference is that the borrow checker for `incompatible``
        // must be taken into account and this leads to ugly code.

        // I preferred to do it this way because
        //   1. I don't have the courage to rewrite an algorithm closer to the Rust spirit
        //   2. it shows that the language and its constraints can influence the choice
        //      and writing of algorithms, and more generally the quality of the code
        //      you're aiming for
        //   3. who cares, really ? 😂

        // build the equivalence map between fields array and values array
        let mut equivalent = FxHashMap::default();
        while !incompatible.is_empty() {
            let &i = incompatible
                .iter()
                .filter(|(_, v)| v.len() == self.fields.len() - 1)
                .map(|(i, _)| i)
                .next()
                .unwrap();
            let v = incompatible[&i].clone();

            // all indices but one are incompatible:
            // the remaining index is the equivalence between fields and values
            for j in 0..self.fields.len() {
                if !v.contains(&j) {
                    equivalent.insert(i, j);

                    // we have found index for field i
                    incompatible.remove(&i);

                    // index is now incompatible for other values
                    for other in incompatible.values_mut() {
                        other.insert(j);
                    }

                    // restart the search for the next unique compatible index
                }
            }
        }

        // ⚠️⚠️⚠️

        if self.verbose {
            println!("equivalent: {equivalent:?}");
        }

        equivalent
            .iter()
            .filter(|(&i, _)| self.fields[i].name.starts_with(field_name))
            .fold(1, |acc, (_, &j)| acc * u64::from(self.your_tickets[j]))
    }

    /// Solve part two.
    fn part2(&self) -> u64 {
        self.solve_part2("departure")
    }
}

fn main() {
    let args = aoc::parse_args();
    let mut puzzle = Puzzle::new();
    puzzle.verbose = args.verbose;
    puzzle.configure(&args.input);
    println!("{}", puzzle.part1());
    println!("{}", puzzle.part2());
}

/// Test from puzzle input
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let mut puzzle = Puzzle::new();
        puzzle.configure(&aoc::load_input_data("sample_2.txt"));
        assert_eq!(puzzle.part1(), 71);
    }

    #[test]
    fn test_part2() {
        let mut puzzle = Puzzle::new();
        puzzle.configure(&aoc::load_input_data("sample_3.txt"));

        // Nota: the result is diverted for the test purpose
        assert_eq!(puzzle.solve_part2("class"), 12);
        // assert_eq!(puzzle.solve_part2("row"), 11);
        assert_eq!(puzzle.solve_part2("seat"), 13);
    }
}
