//! [Day 6: Lanternfish](https://adventofcode.com/2021/day/6)

pub fn main() {
    let args = aoc::parse_args();
    args.run(solve);
}

/// # Panics
/// over malformed input
#[must_use]
pub fn solve(data: &str) -> (u64, u64) {
    let mut timers = [0u64; 9];
    for timer in data.trim().split(',').map(|s| s.parse::<usize>().unwrap()) {
        timers[timer] += 1;
    }
    (lanterfishes(&timers, 80), lanterfishes(&timers, 256))
}

fn lanterfishes(initial: &[u64; 9], days: u64) -> u64 {
    let mut timers = *initial;

    for _day in 0..days {
        let mut new = [0u64; 9];
        for (i, &timer) in timers.iter().enumerate() {
            if i == 0 {
                new[6] += timer;
                new[8] += timer;
            } else {
                new[i - 1] += timer;
            }
        }
        timers = new;
    }

    timers.iter().sum::<u64>()
}
