//! [Day 12: Leonardo's Monorail](https://adventofcode.com/2016/day/12)

use assembunny::{BunnyVM, REG_A, REG_C};

fn run_program(bunny_vm: &mut BunnyVM, c: i32) -> i32 {
    bunny_vm.reset();
    bunny_vm.registers[REG_C] = c;

    bunny_vm.run(1_000_000_000);

    bunny_vm.registers[REG_A]
}

/// # Panics
/// over malformed input
#[must_use]
pub fn solve(data: &str) -> (i32, i32) {
    let mut bunny_vm = BunnyVM::new(data);

    (run_program(&mut bunny_vm, 0), run_program(&mut bunny_vm, 1))
}

pub fn main() {
    let args = aoc::parse_args();
    args.run(solve);
}
