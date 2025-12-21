//! [Day 20: Grove Positioning System](https://adventofcode.com/2022/day/20)

struct Puzzle {
    numbers: Vec<i64>,
}

impl Puzzle {
    fn new(data: &str) -> Self {
        Self {
            numbers: data.lines().map(|x| x.parse().unwrap()).collect(),
        }
    }

    // Solves part one
    fn part1(&self) -> i64 {
        self.decrypt(1, 1)
    }

    // Solve part two
    fn part2(&self) -> i64 {
        self.decrypt(811_589_153, 10)
    }

    fn decrypt(&self, key: i64, rounds: usize) -> i64 {
        let mut mixed: Vec<(usize, i64)> =
            self.numbers.iter().map(|&n| n * key).enumerate().collect();

        let nb = mixed.len();
        let nb_i64 = i64::try_from(nb).unwrap();

        for _ in 0..rounds {
            for i in 0..nb {
                let shift = mixed.iter().position(|&(idx, _)| idx == i).unwrap();

                let val = mixed.remove(shift).1;

                let new_i = (i64::try_from(shift).unwrap() + val).rem_euclid(nb_i64 - 1);

                mixed.insert(usize::try_from(new_i).unwrap(), (i, val));
            }
        }

        let zero_pos = mixed.iter().position(|&(_, val)| val == 0).unwrap();

        mixed[(zero_pos + 1000) % nb].1
            + mixed[(zero_pos + 2000) % nb].1
            + mixed[(zero_pos + 3000) % nb].1
    }
}

#[must_use]
pub fn solve(data: &str) -> (i64, i64) {
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
        assert_eq!(puzzle.part1(), 3);
        assert_eq!(puzzle.part2(), 1_623_178_306);
    }
}
