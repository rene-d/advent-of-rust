/*!
[Day 12: Leonardo's Monorail](https://adventofcode.com/2016/day/12)

You finally reach the top floor of this building: a garden with a slanted glass ceiling. Looks like there are no more stars to be had.

While sitting on a nearby bench amidst some [tiger lilies](https://www.google.com/search?q=tiger+lilies&tbm=isch), you manage to decrypt some of the files you extracted from the servers downstairs.

According to these documents, Easter Bunny HQ isn't just this building - it's a collection of buildings in the nearby area. They're all connected by a local monorail, and there's another building not far from here! Unfortunately, being night, the monorail is currently not operating.

You remotely connect to the monorail control systems and discover that the boot sequence expects a password. The password-checking logic (your puzzle input) is easy to extract, but the code it uses is strange: it's assembunny code designed for the [new computer](https://adventofcode.com/2016/day/11) you just assembled. You'll have to execute the code and get the password.

The assembunny code you've extracted operates on four [registers](https://en.wikipedia.org/wiki/Processor_register) (`a`, `b`, `c`, and `d`) that start at `0` and can hold any [integer](https://en.wikipedia.org/wiki/Integer). However, it seems to make use of only a few [instructions](https://en.wikipedia.org/wiki/Instruction_set):

- `cpy x y` **copies** `x` (either an integer or the **value** of a register) into register `y`.
- `inc x` **increases** the value of register `x` by one.
- `dec x` **decreases** the value of register `x` by one.
- `jnz x y` **jumps** to an instruction `y` away (positive means forward; negative means backward), but only if `x` is **not zero**.

The `jnz` instruction moves relative to itself: an offset of `-1` would continue at the previous instruction, while an offset of `2` would **skip over** the next instruction.

After executing the assembunny code in your puzzle input, **what value is left in register `a`**?

--- Part Two ---

As you head down the fire escape to the monorail, you notice it didn't start; register `c` needs to be initialized to the position of the ignition key.

If you instead **initialize register `c` to be `1`**, what value is now left in register `a`?
*/

use assembunny::{Program, REG_A, REG_C};

fn solve(program: &mut Program, c: i32) -> i32 {
    program.reset();
    program.registers[REG_C] = c;

    program.run(1_000_000_000);

    program.registers[REG_A]
}

fn main() {
    let data = std::fs::read_to_string("input.txt").unwrap();

    let mut program = Program::new(&data);

    println!("{}", solve(&mut program, 0));
    println!("{}", solve(&mut program, 1));
}
