//! [Day 19: An Elephant Named Joseph](https://adventofcode.com/2016/day/19)

fn main() {
    let elves = || {
        if let Some(arg) = std::env::args().nth(1) {
            if let Ok(n) = arg.parse::<usize>() {
                return n;
            }
        }
        let args = aoc::parse_args();
        args.input.trim().parse::<usize>().unwrap()
    };
    let elves = elves();

    println!("{}", part1(elves));
    println!("{}", part2(elves));
}

#[cfg(test)]
fn part1_naive(elves: usize) -> usize {
    let mut gifts = Vec::new();

    gifts.resize(elves, 1_u32);

    let mut current = 0;
    let mut turn = 0;
    loop {
        current = (current + 1) % elves;

        if gifts[current] == 0 {
            continue;
        }

        gifts[turn] += gifts[current];
        // println!("elf {} takes gift from {} and has {} gifts", turn + 1, current + 1, gifts[turn]);

        if gifts[turn] as usize == elves {
            // println!("elft {} wins", turn + 1);
            break turn + 1;
        }
        gifts[current] = 0;

        loop {
            turn = (turn + 1) % elves;
            if gifts[turn] != 0 {
                break;
            }
        }
        current = turn;
    }
}

const fn part1(elves: usize) -> usize {
    let mut x = elves;
    let mut p = 1;
    while x > 1 {
        x /= 2;
        p *= 2;
    }
    (elves - p) * 2 + 1
}

const fn part2(elves: usize) -> usize {
    let mut mst = elves;
    let mut power_of_3 = 1;
    while mst > 2 {
        mst /= 3;
        power_of_3 *= 3;
    }

    if power_of_3 == elves {
        elves
    } else if mst == 1 {
        elves - power_of_3
    } else {
        elves * 2 - power_of_3 * 3
    }
}

#[test]
fn test_part1() {
    assert_eq!(part1_naive(5), 3);
    assert_eq!(part1(5), 3);
}
