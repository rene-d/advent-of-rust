//! Run a assembunny program and print its output

use assembunny::BunnyVM;

fn main() {
    let args = aoc::parse_args();

    let mut vm = BunnyVM::new(&args.input);
    let output = vm.run_output(usize::MAX);

    println!("{:?}", vm.registers);

    print!("{output}");
}
