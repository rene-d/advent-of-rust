/*!
[Day 23: Safe Cracking](https://adventofcode.com/2016/day/23)
*/

use assembunny::{Program, REG_A};

fn compute_until_safe(a: i32, program: &str) -> i32 {
    let mut program = Program::new(program);
    program.registers[REG_A] = a;

    loop {
        program.step();
        if program.ip >= program.len() {
            break program.registers[REG_A];
        }
    }
}

fn main() {
    let data = std::fs::read_to_string("input.txt").unwrap();

    println!("{}", compute_until_safe(7, &data));

    println!("{}", compute_until_safe(12, &data));
}

#[test]
fn test_compute_until_safe() {
    assert_eq!(compute_until_safe(0, "cpy 41 a"), 41);
}
