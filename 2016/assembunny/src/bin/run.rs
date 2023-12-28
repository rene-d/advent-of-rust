//! Run a assembunny program and print its output

use assembunny::BunnyVM;

fn main() {
    let data = aoc::load_input_data(0);

    let mut vm = BunnyVM::new(&data);
    let output = vm.run_output(usize::MAX);

    println!("{:?}", vm.registers);

    print!("{output}");
}
