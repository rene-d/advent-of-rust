/*!
[Day 8: Two-Factor Authentication](https://adventofcode.com/2016/day/8)

You come across a door implementing what you can only assume is an
implementation of [two-factor authentication](https://en.wikipedia.org/wiki/Multi-factor_authentication)
after a long game of [requirements telephone](https://en.wikipedia.org/wiki/Chinese_whispers).

To get past the door, you first swipe a keycard (no problem; there was
one on a nearby desk). Then, it displays a code on a
[little screen](https://www.google.com/search?q=tiny+lcd&tbm=isch), and
you type that code on a keypad. Then, presumably, the door unlocks.

Unfortunately, the screen has been smashed. After a few minutes, you've
taken everything apart and figured out how it works. Now you just have
to work out what the screen **would** have displayed.

The magnetic strip on the card you swiped encodes a series of instructions
for the screen; these instructions are your puzzle input. The screen is
**50 pixels wide and 6 pixels tall**, all of which start off, and is
capable of three somewhat peculiar operations:

- `rect AxB` turns on all of the pixels in a rectangle at the top-left of
   the screen which is A wide and B tall.
- `rotate row y=A by B` shifts all of the pixels in row `A` (`0` is the top
  row) *right* by `B` pixels. Pixels that would fall off the right end appear
  at the left end of the row.
- `rotate column x=A by B` shifts all of the pixels in column `A` (`0` is the
  left column) *down* by `B` pixels. Pixels that would fall off the bottom appear
  at the top of the column.

For example, here is a simple sequence on a smaller screen:

- `rect 3x2` creates a small rectangle in the top-left corner:

```text
###....
###....
.......
````

- `rotate column x=1 by 1` rotates the second column down by one pixel:

```text
#.#....
###....
.#.....
```

- `rotate row y=0 by 4` rotates the top row right by four pixels:

```text
....#.#
###....
.#.....
```

- `rotate column x=1 by 1` again rotates the second column down by one
  pixel, causing the bottom pixel to wrap back to the top:

```text
.#..#.#
#.#....
.#.....
```

As you can see, this display technology is extremely powerful, and will
soon dominate the tiny-code-displaying-screen market. That's what the
advertisement on the back of the display tries to convince you, anyway.

There seems to be an intermediate check of the voltage used by the
display: after you swipe your card, if the screen did work, **how many
pixels should be lit**?

Your puzzle answer was 116.

The first half of this puzzle is complete! It provides one gold star: *

--- Part Two ---

You notice that the screen is only capable of displaying capital letters; in the font it uses, each letter is 5 pixels wide and 6 tall.

After you swipe your card, what code is the screen trying to display?
*/

#![allow(clippy::manual_memcpy)]
#![allow(clippy::needless_range_loop)] // assumed. code is much comprehensible

use regex::Regex;

/// ``main`` reads the puzzle input then solves part 1 and part 2
fn main() {
    let data = std::fs::read_to_string("input.txt").unwrap();

    println!("{}", part1(&data));
}

fn part1(data: &str) -> usize {
    const WIDTH: usize = 50;
    const HEIGHT: usize = 6;

    let re_rect = Regex::new(r"rect (\d+)x(\d+)").unwrap();
    let re_row = Regex::new(r"rotate row y=(\d+) by (\d+)").unwrap();
    let re_col = Regex::new(r"rotate column x=(\d+) by (\d+)").unwrap();

    let mut grid = vec![vec![false; WIDTH]; HEIGHT];

    for line in data.split('\n') {
        if let Some(caps) = re_rect.captures(line) {
            let width = caps[1].parse::<usize>().unwrap();
            let height = caps[2].parse::<usize>().unwrap();

            // println!("rect {}x{}", width, height);

            for x in 0..width {
                for y in 0..height {
                    grid[y][x] = true;
                }
            }
        } else if let Some(caps) = re_row.captures(line) {
            let y = caps[1].parse::<usize>().unwrap();
            let by = caps[2].parse::<usize>().unwrap();

            // println!("row {} by {}", y, by);

            let mut new_row = vec![false; WIDTH];
            for x in 0..WIDTH {
                new_row[(x + by) % WIDTH] = grid[y][x];
            }
            for x in 0..WIDTH {
                grid[y][x] = new_row[x];
            }
        } else if let Some(caps) = re_col.captures(line) {
            let x = caps[1].parse::<usize>().unwrap();
            let by = caps[2].parse::<usize>().unwrap();

            // println!("col {} by {}", x, by);

            let mut new_col = vec![false; HEIGHT];
            for y in 0..HEIGHT {
                new_col[(y + by) % HEIGHT] = grid[y][x];
            }
            for y in 0..HEIGHT {
                grid[y][x] = new_col[y];
            }
        } else {
            // panic!("bad line: {}", line);
        }
    }

    let mut lit = 0;
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            print!(
                "{}",
                if grid[y][x] {
                    lit += 1;
                    '#'
                } else {
                    '.'
                }
            );
        }
        println!();
    }

    lit
}

// UPOJFLBCEZ
