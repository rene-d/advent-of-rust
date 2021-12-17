fn main() {
    let column = 2981;
    let row = 3075;

    let mut x = 1;
    let mut y = 1;
    let mut value: u64 = 20151125;

    while x != row || y != column {
        x += 1;
        y -= 1;
        if y == 0 {
            y = x;
            x = 1;
        }
        value = (value * 252533) % 33554393;
    }
    println!("{}", value);
}
