/*!

*/

use assembunny::{Program, REG_A};

fn run_until_safe(a: i32, program: &mut Program) -> i32 {
    program.reset();
    program.registers[REG_A] = a;

    loop {
        let a_before = program.registers[REG_A];
        program.run(100);
        let a_after = program.registers[REG_A];

        if a_before == a_after {
            break a_after;
        }
    }
}

fn main() {
    let data = std::fs::read_to_string("input.txt").unwrap();

    let mut program = Program::new(&data);

    println!("{}", run_until_safe(7, &mut program));
}
