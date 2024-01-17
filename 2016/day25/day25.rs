//! [Day 25: Clock Signal](https://adventofcode.com/2016/day/25)

use assembunny::{BunnyVM, REG_A};

fn run_clock_signal(bunny_vm: &mut BunnyVM, a: i32) -> bool {
    bunny_vm.reset();

    bunny_vm.registers[REG_A] = a;

    let mut output = [0; 256];
    let mut output_index = 0;

    while !bunny_vm.is_terminated() && output_index < output.len() {
        bunny_vm.step();

        if let Some(value) = bunny_vm.output {
            if value != i32::try_from(output_index).unwrap() % 2 {
                return false;
            }
            output[output_index] = value;
            output_index += 1;
        }
    }

    output_index == output.len()
}

fn main() {
    let args = aoc::parse_args();
    let data = std::fs::read_to_string(args.path).unwrap();

    let mut bunny_vm = BunnyVM::new(&data);

    for a in 0..10000 {
        if run_clock_signal(&mut bunny_vm, a) {
            println!("{a}");
            break;
        }
    }
}
