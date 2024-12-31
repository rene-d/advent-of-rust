use std::time::Duration;

fn main() {
    let args = aoc::parse_args();

    println!("{args:?}");
    std::thread::sleep(Duration::from_micros(2983));
}
