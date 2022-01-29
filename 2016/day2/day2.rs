/*!
[Day 2: Bathroom Security](https://adventofcode.com/2016/day/2)

You arrive at **Easter Bunny Headquarters** under cover of darkness. However,
you left in such a rush that you forgot to use the bathroom! Fancy office
buildings like this one usually have keypad locks on their bathrooms, so
you search the front desk for the code.

"In order to improve security," the document you find says, "bathroom
codes will no longer be written down. Instead, please memorize and follow
the procedure below to access the bathrooms."

The document goes on to explain that each button to be pressed can be
found by starting on the previous button and moving to adjacent buttons
on the keypad: `U` moves up, `D` moves down, `L` moves left, and `R` moves right.
Each line of instructions corresponds to one button, starting at the
previous button (or, for the first line, the **"5" button**); press whatever
button you're on at the end of each line. If a move doesn't lead to a
button, ignore it.

You can't hold it much longer, so you decide to figure out the code as
you walk to the bathroom. You picture a keypad like this:

```text
1 2 3
4 5 6
7 8 9
````

Suppose your instructions are:

```text
ULL
RRDDD
LURDL
UUUUD
```

- You start at "5" and move up (to "2"), left (to "1"), and left (you can't, and stay on "1"), so the first button is `1`.
- Starting from the previous button ("1"), you move right twice (to "3") and then down three times (stopping at "9" after two moves and ignoring the third), ending up with `9`.
- Continuing from "9", you move left, up, right, down, and left, ending with `8`.
- Finally, you move up four times (stopping at "2"), then down once, ending with `5`.

So, in this example, the bathroom code is `1985`.

Your puzzle input is the instructions from the document you found at the front desk. What is the **bathroom code**?

--- Part Two ---

You finally arrive at the bathroom (it's a several minute walk from the lobby so visitors can behold the many fancy conference rooms and water coolers on this floor) and go to punch in the code. Much to your bladder's dismay, the keypad is not at all like you imagined it. Instead, you are confronted with the result of hundreds of man-hours of bathroom-keypad-design meetings:

    1
  2 3 4
5 6 7 8 9
  A B C
    D
You still start at "5" and stop when you're at an edge, but given the same instructions as above, the outcome is very different:

You start at "5" and don't move at all (up and left are both edges), ending at 5.
Continuing from "5", you move right twice and down three times (through "6", "7", "B", "D", "D"), ending at D.
Then, from "D", you move five more times (through "D", "B", "C", "C", "B"), ending at B.
Finally, after five more moves, you end at 3.
So, given the actual keypad layout, the code would be 5DB3.

Using the same instructions in your puzzle input, what is the correct **bathroom code**?
*/

/// ``main`` reads the puzzle input then solves part 1 and part 2
fn main() {
    let data = std::fs::read_to_string("input.txt").unwrap();

    println!("{}", part1(&data));
    println!("{}", part2(&data));
}

/// part 1
fn part1(data: &str) -> String {
    let mut bathroom_code = String::new();

    let mut x = 1u8; // position on the 3x3 grid
    let mut y = 1u8; // starting at the middle of the grid

    for line in data.split('\n') {
        for c in line.chars() {
            match c {
                'U' => y = y.saturating_sub(1),
                'D' => {
                    if y < 2 {
                        y += 1;
                    }
                }
                'L' => x = x.saturating_sub(1),
                'R' => {
                    if x < 2 {
                        x += 1;
                    }
                }
                _ => panic!("unknown direction: {}", c),
            }
        }

        let key = (x + y * 3 + b'1') as char;

        bathroom_code.push(key);
    }

    bathroom_code
}

/// part 2
fn part2(data: &str) -> String {
    let mut bathroom_code = String::new();
    let mut pos = '5';

    for line in data.split('\n') {
        for c in line.chars() {
            pos = match pos {
                '1' => match c {
                    'D' => '3',
                    _ => pos,
                },
                '2' => match c {
                    'D' => '6',
                    'R' => '3',
                    _ => pos,
                },
                '3' => match c {
                    'U' => '1',
                    'D' => '7',
                    'L' => '2',
                    'R' => '4',
                    _ => pos,
                },
                '4' => match c {
                    'D' => '8',
                    'L' => '3',
                    _ => pos,
                },
                '5' => match c {
                    'R' => '6',
                    _ => pos,
                },
                '6' => match c {
                    'U' => '2',
                    'D' => 'A',
                    'L' => '5',
                    'R' => '7',
                    _ => pos,
                },
                '7' => match c {
                    'U' => '3',
                    'D' => 'B',
                    'L' => '6',
                    'R' => '8',
                    _ => pos,
                },
                '8' => match c {
                    'U' => '4',
                    'D' => 'C',
                    'L' => '7',
                    'R' => '9',
                    _ => pos,
                },
                '9' => match c {
                    'L' => '8',
                    _ => pos,
                },
                'A' => match c {
                    'U' => '6',
                    'R' => 'B',
                    _ => pos,
                },
                'B' => match c {
                    'U' => '7',
                    'D' => 'D',
                    'L' => 'A',
                    'R' => 'C',
                    _ => pos,
                },
                'C' => match c {
                    'U' => '8',
                    'L' => 'B',
                    _ => pos,
                },
                'D' => match c {
                    'U' => 'B',
                    _ => pos,
                },
                _ => panic!("unknown position: {}", pos),
            }
        }
        bathroom_code.push(pos);
    }
    bathroom_code
}

#[cfg(test)]
#[test]
fn test_part1() {
    let data = "ULL
RRDDD
LURDL
UUUUD";

    assert_eq!(part1(data), "1985");
}

#[test]
fn test_part2() {
    let data = "ULL
RRDDD
LURDL
UUUUD";

    assert_eq!(part2(data), "5DB3");
}
