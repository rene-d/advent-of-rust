//! [Day 25: Let It Snow](https://adventofcode.com/2015/day/25)

fn main() {
    let (row, column) = read_data();

    let mut x = 1;
    let mut y = 1;
    let mut value: u64 = 20_151_125;

    while x != column || y != row {
        x += 1;
        y -= 1;
        if y == 0 {
            y = x;
            x = 1;
        }
        value = (value * 252_533) % 33_554_393;
    }
    println!("{value}");
}

fn read_data() -> (i32, i32) {
    let args = aoc::parse_args();
    let mut data = args.input.clone();

    data.retain(|c| c.is_ascii_digit() || c.is_whitespace());

    let data: Vec<_> = data.trim().split_ascii_whitespace().collect();

    let row = data[0].parse::<i32>().unwrap();
    let column = data[1].parse::<i32>().unwrap();

    (row, column)
}
