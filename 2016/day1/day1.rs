/*!
[Day 1: No Time for a Taxicab](https://adventofcode.com/2016/day/1)

Santa's sleigh uses a very high-precision clock to guide its movements,
and the clock's oscillator is regulated by stars. Unfortunately, the
stars have been stolen... by the Easter Bunny. To save Christmas, Santa
needs you to retrieve all fifty stars by December 25th.

Collect stars by solving puzzles. Two puzzles will be made available on
each day in the Advent calendar; the second puzzle is unlocked when you
complete the first. Each puzzle grants one star. Good luck!

You're airdropped near **Easter Bunny Headquarters** in a city somewhere.
"Near", unfortunately, is as close as you can get - the instructions on
the Easter Bunny Recruiting Document the Elves intercepted start here,
and nobody had time to work them out further.

The Document indicates that you should start at the given coordinates
(where you just landed) and face North. Then, follow the provided
sequence: either turn left (`L`) or right (`R`) 90 degrees, then walk forward
the given number of blocks, ending at a new intersection.

There's no time to follow such ridiculous instructions on foot, though,
so you take a moment and work out the destination. Given that you can
only walk on the [street grid of the city](https://en.wikipedia.org/wiki/Taxicab_geometry),
how far is the shortest path to the destination?

For example:

- Following `R2, L3` leaves you `2` blocks East and `3` blocks North, or `5` blocks away.
- `R2, R2, R2` leaves you `2` blocks due South of your starting position, which is `2` blocks away.
- `R5, L5, R5, R3` leaves you `12` blocks away.

**How many blocks away** is Easter Bunny HQ?

--- Part Two ---

Then, you notice the instructions continue on the back of the Recruiting
Document. Easter Bunny HQ is actually at the first location you visit twice.

For example, if your instructions are `R8, R4, R4, R8`, the first location
you visit twice is `4` blocks away, due East.

How many blocks away is the **first location you visit twice**?
*/

use std::collections::HashSet;

/// ``main`` reads the puzzle input then solves part 1 and part 2
fn main() {
    let data = std::fs::read_to_string("input.txt").unwrap();

    let mut x = 0i32;
    let mut y = 0i32;
    let mut angle = 0;

    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    let mut twice = false;

    for op2 in data.split(',') {
        let op = op2.trim();

        let direction = op.chars().next().unwrap();
        let distance = op[1..].parse::<i32>().unwrap();

        match direction {
            'L' => angle = (angle + 90) % 360,
            'R' => angle = (angle + 270) % 360,
            _ => panic!("unknown direction: {}", direction),
        }

        for _ in 1..=distance {
            match angle {
                0 => y += 1,
                90 => x += 1,
                180 => y -= 1,
                270 => x -= 1,
                _ => panic!("unknown angle: {}", angle),
            }

            if !twice && visited.contains(&(x, y)) {
                println!("twice: {} (part 2)", x.abs() + y.abs());
                twice = true;
            } else {
                visited.insert((x, y));
            }
        }
    }

    println!("Easter Bunny HQ: {} (part 1)", x.abs() + y.abs());
}
