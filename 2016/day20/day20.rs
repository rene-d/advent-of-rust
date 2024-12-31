//! [Day 20: Firewall Rules](https://adventofcode.com/2016/day/20)

fn main() {
    let args = aoc::parse_args();

    // load the blacklist and sort it ascending
    let mut blacklist = Vec::new();
    for line in args.input.lines() {
        if line.is_empty() {
            continue;
        }
        let min_max = line.split('-').map(|x| x.parse::<u32>().unwrap()).collect::<Vec<u32>>();

        blacklist.push((min_max[0], min_max[1]));
    }
    blacklist.sort_by(|a, b| a.0.cmp(&b.0));

    // find the min value outside a range
    let mut not_blocked = 0;
    for &(min, max) in &blacklist {
        if not_blocked < min {
            break;
        }
        not_blocked = not_blocked.max(max + 1);
    }
    println!("{not_blocked}");

    // merge the ranges
    let mut merged = Vec::<(u32, u32)>::new();
    for (min, max) in blacklist {
        if let Some(last) = merged.last_mut() {
            if last.1 >= min {
                last.1 = max.max(last.1);
            } else {
                merged.push((min, max));
            }
        } else {
            merged.push((min, max));
        }
    }
    println!(
        "{}",
        u32::MAX - merged.iter().map(|&(min, max)| max - min + 1).sum::<u32>() + 1
    );
}
