//! [Day 23: Safe Cracking](https://adventofcode.com/2016/day/23)

use assembunny::{BunnyVM, REG_A};

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

fn factorial(n: i32) -> i32 {
    (1..=n).product()
}

fn main() {
    let args = aoc::parse_args();

    let part1 = compute_until_safe(7, &args.input);

    println!("{part1}");

    // as the program calculates n! + constant,
    // we advantageously can reuse the answer from part 1
    // nota: constant=n1*n2 where n1,n2 are immediate in the program:
    //  19:     cpy n1 c
    //  20:     jnz n2 d
    println!("{}", part1 + factorial(12) - factorial(7));
}

#[test]
fn test_compute_until_safe() {
    assert_eq!(compute_until_safe(0, "cpy 41 a"), 41);
}
