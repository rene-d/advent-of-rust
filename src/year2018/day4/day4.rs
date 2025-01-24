//! [Day 4: Repose Record](https://adventofcode.com/2018/day/4)

use rustc_hash::FxHashMap;

struct Puzzle {
    /// Number of minutes asleep for each minute from 00:00 to 00:59 by guard ID
    sleeping: FxHashMap<u32, [u32; 60]>,
}

impl Puzzle {
    fn new(data: &str) -> Self {
        let mut sleeping = FxHashMap::default();

        let mut lines: Vec<_> = data.lines().collect();
        lines.sort_unstable();

        let mut asleep: u32 = 0;
        let mut current_guard: u32 = 0;

        for line in &lines {
            let minute: u32 = line[15..17].parse().unwrap();

            if let Some(guard) = line[19..].strip_prefix("Guard #") {
                current_guard = guard
                    .strip_suffix(" begins shift")
                    .unwrap()
                    .parse()
                    .unwrap();
            } else if &line[19..] == "falls asleep" {
                asleep = minute;
            } else if &line[19..] == "wakes up" {
                let e = sleeping.entry(current_guard).or_insert([0; 60]);

                for m in asleep..minute {
                    e[m as usize] += 1;
                }
            }
        }

        Self { sleeping }
    }

    /// Solve part one.
    fn part1(&self) -> u32 {
        let guard_most_asleep = self
            .sleeping
            .iter()
            .map(|(guard, v)| (v.iter().sum::<u32>(), guard)) // sum the minutes spent a sleep
            .max() // keep the highest value
            .unwrap()
            .1; // retrieve the id of the guard

        let minute_asleep_most: u32 = self.sleeping[guard_most_asleep]
            .iter()
            .enumerate() // index=minute, value=minutes spent asleep
            .max_by_key(|&(_, n)| n) // select the max value within the array
            .unwrap()
            .0 // get the minute between 0 and 59
            .try_into()
            .unwrap();

        guard_most_asleep * minute_asleep_most
    }

    /// Solve part two.
    fn part2(&self) -> u32 {
        let ((_n, guard), n) = (0..60)
            .map(|m| {
                (
                    self.sleeping
                        .iter()
                        .map(|(guard, v)| (v[m], guard))
                        .max() // get guard ID who is most asleep at minute m
                        .unwrap(),
                    m,
                )
            })
            .max_by_key(|&((n, _guard), _m)| n)
            .unwrap();

        guard * u32::try_from(n).unwrap()
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
        assert_eq!(puzzle.part1(), 240);
    }

    #[test]
    fn test02() {
        let puzzle = Puzzle::new(TEST_INPUT);
        assert_eq!(puzzle.part2(), 4455);
    }
}
