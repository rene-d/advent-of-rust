/*!
[Day 10: Balance Bots](https://adventofcode.com/2016/day/10)

You come upon a factory in which many robots are zooming around handing
small microchips to each other.

Upon closer examination, you notice that each bot only proceeds when it
has **two** microchips, and once it does, it gives each one to a different
bot or puts it in a marked "output" bin. Sometimes, bots take microchips
from "input" bins, too.

Inspecting one of the microchips, it seems like they each contain a
single number; the bots must use some logic to decide what to do with
each chip. You access the local control computer and download the bots'
instructions (your puzzle input).

Some of the instructions specify that a specific-valued microchip should
be given to a specific bot; the rest of the instructions indicate what a
given bot should do with its **lower-value** or **higher-value** chip.

For example, consider the following instructions:

``
value 5 goes to bot 2
bot 2 gives low to bot 1 and high to bot 0
value 3 goes to bot 1
bot 1 gives low to output 1 and high to bot 0
bot 0 gives low to output 2 and high to output 0
value 2 goes to bot 2
``

- Initially, bot `1` starts with a value-`3` chip, and bot `2` starts with
  a value-`2` chip and a value-`5` chip.
- Because bot `2` has two microchips, it gives its lower one (`2`) to bot
  `1` and its higher one (`5`) to bot `0`.
- Then, bot `1` has two microchips; it puts the value-`2` chip in output `1`
  and gives the value-`3` chip to bot `0`.
- Finally, bot `0` has two microchips; it puts the `3` in output `2` and the
  `5` in output `0`.

In the end, output bin `0` contains a value-`5` microchip, output bin `1`
contains a value-`2` microchip, and output bin `2` contains a value-`3`
microchip. In this configuration, bot number `2` is responsible for
comparing value-`5` microchips with value-`2` microchips.

Based on your instructions, **what is the number of the bot** that is
responsible for comparing value-`61` microchips with value-`17` microchips?

--- Part Two ---

What do you get if you **multiply together the values** of one chip in each
of outputs `0`, `1`, and `2`?
*/

use regex::Regex;
use std::collections::{HashMap, HashSet};

/// `main` reads the puzzle input then solves part 1 and part 2
fn main() {
    let data = std::fs::read_to_string("input.txt").unwrap();

    let data = data.split('\n').collect::<Vec<&str>>();

    let (part1, part2) = solve(data);

    println!("part1: {}", part1);
    println!("part2: {}", part2);
}

/// `BotOutput`represents the output of a bot
#[derive(Copy, Clone)]
enum BotOutput {
    /// Destination is another bot
    Bot(u32),
    /// Destination is an output bin
    Bin(u32),
}

impl BotOutput {
    /// `new` creates a new `BotOutput` from string and id
    fn new(dest: &str, id: u32) -> BotOutput {
        match dest {
            "bot" => BotOutput::Bot(id),
            "output" => BotOutput::Bin(id),
            _ => panic!("invalid destination"),
        }
    }
}

/// `BotInstruction` represents a bot move instruction
struct BotInstruction {
    /// ID of the bot to move from
    from: u32,
    /// Destination of the lower value chip
    low_to: BotOutput,
    /// Destination of the higher value chip
    high_to: BotOutput,
}

/// `solve` solves part 1 and part 2 of the puzzle.
/// First, it loads the move instructions and initializes the bots.
/// Then, it runs the instructions until the puzzle is done.
fn solve(data: Vec<&str>) -> (u32, u32) {
    let re_init = Regex::new(r"value ([\d]+) goes to bot (\d+)").unwrap();
    let re_move = Regex::new(r"bot (\d+) gives low to (bot|output) (\d+) and high to (bot|output) (\d+)").unwrap();

    let mut bots: HashMap<u32, HashSet<u32>> = HashMap::new();
    let mut moves: Vec<BotInstruction> = Vec::new();

    for line in data {
        if let Some(caps) = re_init.captures(line) {
            //
            let value = caps[1].parse::<u32>().unwrap();
            let bot = caps[2].parse::<u32>().unwrap();
            assert_ne!(value, 0);
            bots.entry(bot).or_insert_with(HashSet::new).insert(value);
        } else if let Some(caps) = re_move.captures(line) {
            //
            moves.push(BotInstruction {
                from: caps[1].parse::<u32>().unwrap(),
                low_to: BotOutput::new(
                    caps[2].parse::<String>().unwrap().as_str(),
                    caps[3].parse::<u32>().unwrap(),
                ),
                high_to: BotOutput::new(
                    caps[4].parse::<String>().unwrap().as_str(),
                    caps[5].parse::<u32>().unwrap(),
                ),
            });
        } else {
            panic!("bad line: {}", line);
        }
    }

    let mut found_first: Option<u32> = Option::None; // part1 completed (found first bot)

    let mut output_bin0 = 0; // count of values received by bin 0
    let mut output_bin1 = 0; // count of values received by bin 1
    let mut output_bin2 = 0; // count of values received by bin 2
    let mut output_values = Vec::new(); // values received by output bins 0,1,2
    let mut found_output: Option<u32> = Option::None; // part2 completed (found first matching output)

    let mut max_iterations = 10000;

    'main_loop: loop {
        // loop until we have completed both parts of the puzzle

        // iterate over all bot instructions
        for m in &moves {
            assert!(max_iterations > 0, "too many iterations");
            max_iterations -= 1;

            // has the bot been initialized?
            if let Some(v) = bots.get_mut(&m.from) {
                // to process, the bot must also have two chips or more
                if v.len() >= 2 {
                    // find lower and higher values chips
                    let low_value = *v.iter().min().unwrap();
                    let high_value = *v.iter().max().unwrap();

                    // remove them from the bot
                    v.remove(&low_value);
                    v.remove(&high_value);

                    // closure to move a microchip to a bot or a bin
                    let mut move_microchip = |to: BotOutput, value: u32| match to {
                        BotOutput::Bot(to_id) => {
                            bots.entry(to_id).or_insert_with(HashSet::new).insert(value);
                        }
                        BotOutput::Bin(to_id) => match to_id {
                            0 => {
                                output_values.push(value);
                                output_bin0 += 1;
                            }
                            1 => {
                                output_values.push(value);
                                output_bin1 += 1;
                            }
                            2 => {
                                output_values.push(value);
                                output_bin2 += 1;
                            }
                            _ => (), // ignore other bins
                        },
                    };

                    // process low and high microchip values
                    move_microchip(m.low_to, low_value);
                    move_microchip(m.high_to, high_value);

                    // part 1 of the puzzle
                    if found_first.is_none() && low_value == 17 && high_value == 61 {
                        found_first = Some(m.from);
                    }

                    // part 2 of the puzzle
                    if found_output.is_none() && output_bin0 != 0 && output_bin1 != 0 && output_bin2 != 0 {
                        found_output = Some(output_values.iter().product::<u32>());
                    }
                }
            }
        }

        if let Some(part1) = found_first {
            if let Some(part2) = found_output {
                // the both parts of the puzzle have been completed
                break 'main_loop (part1, part2);
            }
        }
    }
}

#[test]
fn test_solve() {
    let instructions = [
        // verify part 1
        "value 17 goes to bot 20",                     // add value-17 chip to bot 20
        "value 61 goes to bot 20",                     // add value-61 chip to bot 20
        "bot 20 gives low to bot 1 and high to bot 1", // should complete part 1 with bot id = 20
        // verify part 2
        "bot 1 gives low to output 0 and high to output 1", // bin 0: 17, bin 1: 61
        "value 29 goes to bot 2",
        "value 56 goes to bot 2",
        "bot 2 gives low to output 2 and high to bot 0", // bin 2: 29, part 2 should be completed with 17*61*29=30073
    ];

    let (part1, part2) = solve(instructions.to_vec());

    assert_eq!(part1, 20);
    assert_eq!(part2, 30073);
}
