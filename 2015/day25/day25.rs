//! [Day 25: Let It Snow](http://adventofcode.com/2015/day/25)

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
    let filename = if let Some(x) = std::env::args().collect::<Vec<String>>().get(1) {
        x.clone()
    } else {
        "input.txt".to_string()
    };

    let data = std::fs::read_to_string(filename)
        .unwrap()
        .trim()
        .to_string();
    let data = data.split(' ').collect::<Vec<&str>>();

    let row = data[16].strip_suffix(',').unwrap().parse::<i32>().unwrap();
    let column = data[18].strip_suffix('.').unwrap().parse::<i32>().unwrap();

    (row, column)
}
