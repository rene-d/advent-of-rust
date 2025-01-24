//! [Day 10: Elves Look, Elves Say](https://adventofcode.com/2015/day/10)

pub fn main() {
    let args = aoc::parse_args();
    args.run(solve);
}

/// # Panics
#[must_use]
pub fn solve(data: &str) -> (usize, usize) {
    let data = data.trim_ascii();

    (calc(data, 40), calc(data, 50))
}

fn calc(start_sequence: &str, turns: u32) -> usize {
    let mut look = start_sequence.bytes().collect::<Vec<_>>();

    for _ in 0..turns {
        let mut say: Vec<_> = Vec::new();

        let mut count = 0;
        let mut previous = 0;
        for current in &look {
            if previous != 0 && previous != *current {
                extend(&mut say, count);
                say.push(previous);
                count = 0;
            }
            count += 1;
            previous = *current;
        }

        extend(&mut say, count);
        say.push(previous);

        look.clone_from(&say);
    }

    look.len()
}

fn extend(say: &mut Vec<u8>, num: u32) {
    let mut tmp: u32 = num;
    let mut base = 1;
    loop {
        tmp /= 10;
        if tmp == 0 {
            break;
        }
        base *= 10;
    }

    while base != 0 {
        say.push(b'0' + ((num / base) % 10) as u8);
        base /= 10;
    }
}
