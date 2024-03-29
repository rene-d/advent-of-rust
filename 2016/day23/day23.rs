//! [Day 23: Safe Cracking](https://adventofcode.com/2016/day/23)

use assembunny::{BunnyVM, REG_A};
// use std::time::Instant;

fn compute_until_safe(a: i32, program: &str) -> i32 {
    let mut program = BunnyVM::new(program);
    program.registers[REG_A] = a;

    loop {
        program.step();
        if program.is_terminated() {
            break program.registers[REG_A];
        }
    }
}

fn main() {
    let data = aoc::load_input_data(23);

    // let now = Instant::now();

    println!("{}", compute_until_safe(7, &data));

    println!("{}", compute_until_safe(12, &data));

    // let micros = f64::from_u128(now.elapsed().as_micros()).unwrap();
    // println!("elapsed: {} s", micros / 1_000_000.);
}

#[test]
fn test_compute_until_safe() {
    assert_eq!(compute_until_safe(0, "cpy 41 a"), 41);
}
