/*!
[Day 25: Clock Signal](https://adventofcode.com/2016/day/25)
*/

use assembunny::{Program, REG_A};

/// run the program and returns the value of register `a`
fn run_clock_signal(program: &mut Program, a: i32) -> bool {
    program.reset();

    program.registers[REG_A] = a;

    let mut output = [0; 256];
    let mut output_index = 0;

    while program.ip < program.len() && output_index < output.len() {
        program.step();

        if let Some(value) = program.output {
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
    let data = std::fs::read_to_string("input.txt").unwrap();

    let mut program = Program::new(&data);

    for a in 0..10000 {
        if run_clock_signal(&mut program, a) {
            println!("{}", a);
            break;
        }
    }
}
