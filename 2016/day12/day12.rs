//! [Day 12: Leonardo's Monorail](https://adventofcode.com/2016/day/12)

use assembunny::{BunnyVM, REG_A, REG_C};

fn solve(bunny_vm: &mut BunnyVM, c: i32) -> i32 {
    bunny_vm.reset();
    bunny_vm.registers[REG_C] = c;

    bunny_vm.run(1_000_000_000);

    bunny_vm.registers[REG_A]
}

fn main() {
    let mut args = aoc::parse_args();

    args.run(|program| {
        let mut bunny_vm = BunnyVM::new(program);

        (solve(&mut bunny_vm, 0), solve(&mut bunny_vm, 1))
    });
}
