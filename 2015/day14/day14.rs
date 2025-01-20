//! [Day 14: Reindeer Olympics](https://adventofcode.com/2015/day/14)

use regex::Regex;

#[derive(Debug)]
struct Reinder {
    // name: String,
    speed: u32,
    duration: u32,
    rest: u32,
}

fn solve_duration(data: &str, max_duration: u32) -> (u32, u32) {
    let mut reinders = Vec::new();

    let re = Regex::new(
        r"(\w+) can fly (\d+) km/s for (\d+) seconds, but then must rest for (\d+) seconds.",
    )
    .unwrap();

    for line in data.lines() {
        re.captures(line).map(|cap| {
            let reinder = Reinder {
                // name: cap[1].to_string(),
                speed: cap[2].parse().unwrap(),
                duration: cap[3].parse().unwrap(),
                rest: cap[4].parse().unwrap(),
            };
            // println!("{:?}", reinder);
            reinders.push(reinder);
            0
        });
    }

    // part 1
    let max_distance = reinders
        .iter()
        .map(|reinder: &Reinder| -> u32 {
            let mut seconds = max_duration;
            let mut distance = 0;
            while seconds >= reinder.duration + reinder.rest {
                seconds -= reinder.duration + reinder.rest;
                distance += reinder.speed * reinder.duration;
            }
            distance += reinder.speed * std::cmp::min(seconds, reinder.duration);
            // println!("{:10} -> {}", reinder.name, distance);
            distance
        })
        .max()
        .unwrap();

    // part 2
    let mut scores: Vec<u32> = vec![0; reinders.len()];
    let mut distances: Vec<u32> = vec![0; reinders.len()];

    for elapsed in 1..max_duration {
        for i in 0..reinders.len() {
            let reinder = &reinders[i];

            let mut seconds = elapsed;
            let mut distance = 0;
            while seconds >= reinder.duration + reinder.rest {
                seconds -= reinder.duration + reinder.rest;
                distance += reinder.speed * reinder.duration;
            }
            distance += reinder.speed * std::cmp::min(seconds, reinder.duration);
            distances[i] = distance;

            // println!("{:4}: {:10} -> {:4} {:4}", elapsed, reinder.name, distance, scores[i]);
        }

        let distance_max = distances.iter().max().unwrap();
        for i in 0..reinders.len() {
            if distances[i] == *distance_max {
                scores[i] += 1;
            }
        }
    }

    (max_distance, *scores.iter().max().unwrap())
}

fn solve(data: &str) -> (u32, u32) {
    solve_duration(data, 2503)
}

/// Main function.
fn main() {
    let mut args = aoc::parse_args();
    args.run(solve);
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = include_str!("test.txt");

    #[test]
    fn test_solve() {
        assert_eq!(solve_duration(TEST_INPUT, 1000), (1120, 689));
    }
}
