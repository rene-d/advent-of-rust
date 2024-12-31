//! [Day 1: Sonar Sweep](https://adventofcode.com/2021/day/1)

fn main() {
    let args = aoc::parse_args();

    let mut prev_num = 999_999_999_u32;
    let mut result = 0;

    let mut data = vec![];

    // step 1
    for line in args.input.lines() {
        // convertit string -> u32
        let num: u32 = line.parse().unwrap();

        // est-ce que on est en "increase" ?
        if prev_num < num {
            result += 1;
        }
        prev_num = num;

        // pour le step 2
        data.push(num);
    }

    println!("{result}");

    // step 2
    prev_num = 999_999_999_u32;
    result = 0;

    for i in 0..data.len() - 2 {
        let num = data[i] + data[i + 1] + data[i + 2];

        if prev_num < num {
            result += 1;
        }
        prev_num = num;
    }
    println!("{result}");
}
