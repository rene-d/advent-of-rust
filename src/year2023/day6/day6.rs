//! [Day 6: Wait For It](https://adventofcode.com/2023/day/6)

struct Puzzle {
    time: String,
    distance: String,
}

impl Puzzle {
    fn new(data: &str) -> Self {
        let mut lines = data.lines();

        Self {
            time: lines
                .next()
                .unwrap()
                .strip_prefix("Time:")
                .unwrap()
                .to_string(),
            distance: lines
                .next()
                .unwrap()
                .strip_prefix("Distance:")
                .unwrap()
                .to_string(),
        }
    }

    // fn win(t: u64, d: u64) -> u64 {
    //     // nota: see below for the elegant math solution 🤓
    //     let mut win = 0;
    //     for hold in 1..t {
    //         let speed = hold;
    //         let remaining = t - hold;
    //         let travelled = speed * remaining;
    //         if travelled > d {
    //             win += 1;
    //         }
    //     }
    //     win
    // }

    fn win(t: u64, d: u64) -> u64 {
        // solve hold * (t - hold) > d
        // e.g find (a,b) such that a < hold < b

        let t = u128::from(t); // use u128 to avoid overflow
        let d = u128::from(d);

        let sqrt_disc = (t * t - 4 * d).isqrt(); // Δ = t² - 4 d
        let mut a = (t - sqrt_disc) / 2; //  ⌊ (t - √Δ) / 2 ⌋
        let mut b = (t + sqrt_disc).div_ceil(2); // ⌈ (t + √Δ) / 2 ⌉

        if a * (t - a) > d {
            a -= 1;
        }
        if b * (t - b) > d {
            b += 1;
        }

        u64::try_from(b - a - 1).unwrap()
    }

    /// Solve part one.
    fn part1(&self) -> u64 {
        let time = self
            .time
            .split_whitespace()
            .map(|x| x.parse::<u64>().unwrap());
        let distance = self
            .distance
            .split_whitespace()
            .map(|x| x.parse::<u64>().unwrap());

        let mut result = 1;
        for (t, d) in time.zip(distance) {
            result *= Self::win(t, d);
        }
        result
    }

    /// Solve part two.
    fn part2(&self) -> u64 {
        let time = self.time.replace(' ', "").parse::<u64>().unwrap();
        let distance = self.distance.replace(' ', "").parse::<u64>().unwrap();

        Self::win(time, distance)
    }
}

/// # Panics
#[must_use]
pub fn solve(data: &str) -> (u64, u64) {
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
        assert_eq!(puzzle.part1(), 288);
    }

    #[test]
    fn test02() {
        let puzzle = Puzzle::new(TEST_INPUT);
        assert_eq!(puzzle.part2(), 71503);
    }
}
