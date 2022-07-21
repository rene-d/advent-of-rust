//! [Day 19: An Elephant Named Joseph](https://adventofcode.com/2016/day/19)

fn main() {
    let data = aoc::load_input_data(17);
    let length = data.trim().parse::<usize>().unwrap();

    println!("{}", part1(length));
}

fn part1(length: usize) -> usize {
    let mut gifts = Vec::new();

    gifts.resize(length, 1_u32);

    let mut current = 0;
    let mut turn = 0;
    loop {
        current = (current + 1) % length;

        if gifts[current] == 0 {
            continue;
        }

        gifts[turn] += gifts[current];
        // println!("elf {} takes gift from {} and has {} gifts", turn + 1, current + 1, gifts[turn]);

        if gifts[turn] as usize == length {
            // println!("elft {} wins", turn + 1);
            break turn + 1;
        }
        gifts[current] = 0;

        loop {
            turn = (turn + 1) % length;
            if gifts[turn] != 0 {
                break;
            }
        }
        current = turn;
    }
}

#[test]
fn test_part1() {
    assert_eq!(part1(5), 3);
}
