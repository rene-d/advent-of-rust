fn main() {
    let args = aoc::parse_args();

    let program = intcode::Computer::load(args.input());

    println!("{program}");
}
