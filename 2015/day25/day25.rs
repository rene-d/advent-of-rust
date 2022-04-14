//! [Day 25: Let It Snow](http://adventofcode.com/2015/day/25)

fn main() {
    let column = 2981;
    let row = 3075;

    let mut x = 1;
    let mut y = 1;
    let mut value: u64 = 20_151_125;

    while x != row || y != column {
        x += 1;
        y -= 1;
        if y == 0 {
            y = x;
            x = 1;
        }
        value = (value * 252_533) % 33_554_393;
    }
    println!("{}", value);
}
