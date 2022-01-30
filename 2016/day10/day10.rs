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

    solve(&data);
}

/// `BotOutput`represents the output of a bot
#[derive(Copy, Clone)]
enum BotOutput {
    /// Destination is another bot
    Bot,
    /// Destination is an output bin
    Bin,
}

/// `BotInstruction` represents a bot move instruction
struct BotInstruction {
    /// ID of the bot to move from
    from: u32,
    /// Destination of the lower value chip
    low_to: BotOutput,
    /// ID of the bin or bot to move to the lower value chip
    low_to_id: u32,
    /// Destination of the higher value chip
    high_to: BotOutput,
    /// ID of the bin or bot to move to the higher value chip
    high_to_id: u32,
}

/// `solve` solves part 1 and part 2 of the puzzle.
/// First, it loads the move instructions and initializes the bots.
/// Then, it runs the instructions until the puzzle is done.
fn solve(data: &str) {
    let re_init = Regex::new(r"value ([\d]+) goes to bot (\d+)").unwrap();
    let re_move = Regex::new(r"bot (\d+) gives low to (bot|output) (\d+) and high to (bot|output) (\d+)").unwrap();

    let mut bots: HashMap<u32, HashSet<u32>> = HashMap::new();
    let mut moves: Vec<BotInstruction> = Vec::new();

    for line in data.split('\n') {
        if let Some(caps) = re_init.captures(line) {
            //
            let value = caps[1].parse::<u32>().unwrap();
            let bot = caps[2].parse::<u32>().unwrap();
            assert_ne!(value, 0);
            bots.entry(bot).or_insert_with(HashSet::new).insert(value);
        } else if let Some(caps) = re_move.captures(line) {
            //
            let low_to = caps[2].parse::<String>().unwrap();
            let high_to = caps[4].parse::<String>().unwrap();

            moves.push(BotInstruction {
                from: caps[1].parse::<u32>().unwrap(),
                low_to: if low_to == "bot" {
                    BotOutput::Bot
                } else {
                    BotOutput::Bin
                },
                low_to_id: caps[3].parse::<u32>().unwrap(),
                high_to: if high_to == "bot" {
                    BotOutput::Bot
                } else {
                    BotOutput::Bin
                },
                high_to_id: caps[5].parse::<u32>().unwrap(),
            });
        } else {
            panic!("bad line: {}", line);
        }
    }

    let mut found_first = false; // part1 completed (found first bot)

    let mut output_bin0 = 0; // count of values received by bin 0
    let mut output_bin1 = 0; // count of values received by bin 1
    let mut output_bin2 = 0; // count of values received by bin 2
    let mut output_values = Vec::new(); // values received by output bins 0,1,2
    let mut found_output = false; // part2 completed (found first matching output)

    'main_loop: loop {
        // loop until we have completed both parts of the puzzle

        // iterate over all bot instructions
        for m in &moves {
            //
            if let Some(v) = bots.get(&m.from) {
                // to process, the bot must have at least two chips
                if v.len() >= 2 {
                    let low_value: u32; // declare values out of the get_mut scope
                    let high_value: u32; // because the closure below needs to borrow the ownership of bots too

                    if let Some(values) = bots.get_mut(&m.from) {
                        low_value = *values.iter().min().unwrap();
                        high_value = *values.iter().max().unwrap();

                        values.remove(&low_value);
                        values.remove(&high_value);
                    } else {
                        low_value = 0; // value not possible
                        high_value = 0;
                    }

                    // move a microchip to a bot or a bin
                    let mut move_microchip = |to: BotOutput, to_id: u32, value: u32| match to {
                        BotOutput::Bot => {
                            bots.entry(to_id).or_insert_with(HashSet::new).insert(value);
                        }
                        BotOutput::Bin => match to_id {
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
                    move_microchip(m.low_to, m.low_to_id, low_value);
                    move_microchip(m.high_to, m.high_to_id, high_value);

                    // part 1 of the puzzle
                    if !found_first && low_value == 17 && high_value == 61 {
                        found_first = true;
                        println!("part1: {}", m.from);
                    }

                    // part 2 of the puzzle
                    if !found_output && output_bin0 != 0 && output_bin1 != 0 && output_bin2 != 0 {
                        found_output = true;
                        println!("part2: {}", output_values.iter().product::<u32>());
                    }
                }
            }
        }

        if found_first && found_output {
            // the both parts of the puzzle have been completed
            break 'main_loop;
        }
    }
}
