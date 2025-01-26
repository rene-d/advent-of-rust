//! [Day 25: Cryostasis](https://adventofcode.com/2019/day/25)

use intcode::{Computer, State};
use rustc_hash::{FxHashMap, FxHashSet};

type Room = String;
type Direction = String;
type Map = FxHashMap<Room, FxHashMap<Direction, Room>>;

fn run(computer: &mut Computer, command: &str) -> String {
    computer.input_flush();

    if !command.is_empty() {
        command.bytes().for_each(|byte| computer.push_byte(byte));
        computer.push_byte(b'\n');
    }

    let mut output = String::new();
    for _ in 1..10000 {
        match computer.run() {
            State::Output(ch) => {
                output.push(char::from_u32(u32::try_from(ch).unwrap()).unwrap());
            }
            State::Halted | State::Input => return output,
        }
    }

    String::new()
}

fn parse(output: &str) -> (&str, Vec<&str>, Vec<&str>) {
    let mut dirs = Vec::new();
    let mut items = Vec::new();
    let mut room = "";

    for line in output.lines() {
        if let Some(s) = line.strip_prefix("== ") {
            if let Some(s) = s.strip_suffix(" ==") {
                if room.is_empty() {
                    room = s;
                }
            }
        }

        if let Some(s) = line.strip_prefix("- ") {
            match s {
                "north" | "east" | "south" | "west" => dirs.push(s),
                _ => items.push(s),
            };
        }
    }

    (room, dirs, items)
}

fn reverse(direction: &str) -> &'static str {
    match direction {
        "north" => "south",
        "south" => "north",
        "east" => "west",
        "west" => "east",
        _ => "none",
    }
}

fn explore(computer: &mut Computer, map: &mut Map, output: &str) {
    //

    let (room, dirs, items) = parse(output);

    for dir in &dirs {
        if map.contains_key(room) && map[room].contains_key(*dir) {
            continue;
        }

        // go next room
        let output = run(computer, dir);

        let (newroom, _, _) = parse(&output);
        let known = map.contains_key(newroom);

        map.entry(room.to_string())
            .or_default()
            .insert((*dir).to_string(), newroom.to_string());
        map.entry(newroom.to_string())
            .or_default()
            .insert(reverse(dir).to_string(), room.to_string());

        if output.contains("A loud") {
            continue;
        }

        if !known {
            explore(computer, map, &output);
        }

        run(computer, reverse(dir));
    }

    for item in items {
        let mut temp_computer = computer.clone();

        let command = format!("take {item}");

        let output = run(&mut temp_computer, &command);
        if output.is_empty() {
            // stuck
            continue;
        }

        let output = run(&mut temp_computer, "inv");
        if output.contains("Items in your inventory") {
            run(computer, &command);
        }
    }
}

fn find_path<'a>(map: &'a Map, start: &str, target: &str) -> Vec<&'a str> {
    let mut seen = FxHashSet::default();

    let mut stack = Vec::new();

    stack.push((start, Vec::new()));

    while let Some((current, path)) = stack.pop() {
        if current == target {
            return path;
        }

        for (dir, cell) in &map[current] {
            if seen.insert(cell) {
                let mut new_path = path.clone();
                new_path.push(dir);
                stack.push((cell, new_path));
            }
        }
    }

    Vec::new()
}

fn find_weight(computer: &mut Computer, inventory: &[&str], checkpoint_dir: &str) -> u64 {
    let re = regex::Regex::new(
        r"You should be able to get in by typing (\d+) on the keypad at the main airlock.",
    )
    .unwrap();

    let mut prev_code = 0i32;

    for code in 0..(1 << inventory.len()) {
        let gray_code = code ^ (code >> 1);
        let diff = gray_code - prev_code;

        if diff != 0 {
            let action = if diff > 0 { "drop" } else { "take" };

            let mut diff = diff.abs();
            let mut item = 0;
            while diff & 1 == 0 {
                diff >>= 1;
                item += 1;
            }

            let command = format!("{action} {}", inventory[item]);
            run(computer, &command);
        }

        let output = run(computer, checkpoint_dir);

        if let Some(caps) = re.captures(&output) {
            return caps[1].parse().unwrap();
        }

        prev_code = gray_code;
    }

    0
}

/// # Panics
#[must_use]
pub fn solve(program: &str) -> (u64, aoc::Christmas) {
    let mut computer = Computer::load(program);

    let mut map = FxHashMap::default();

    let output = run(&mut computer, "");
    let (start_room, _, _) = parse(&output);

    // visit all rooms and collect items
    explore(&mut computer, &mut map, &output);

    // find path to the Pressure-Sensitive Floor
    let path = find_path(&map, start_room, "Pressure-Sensitive Floor");

    // the direction to go to Security Checkpoint from Pressure-Sensitive Floor
    let checkpoint_dir = map["Security Checkpoint"]
        .iter()
        .filter(|(_, room)| room == &"Pressure-Sensitive Floor")
        .map(|(dir, _)| dir)
        .next()
        .unwrap();

    // go to the Pressure-Sensitive Floor
    for step in path {
        run(&mut computer, step);
    }

    // the inventory
    let inventory = run(&mut computer, "inv");
    let inventory = inventory
        .lines()
        .filter_map(|line| line.strip_prefix("- "))
        .collect::<Vec<&str>>();

    // get the unlock code
    (
        find_weight(&mut computer, &inventory, checkpoint_dir),
        aoc::CHRISTMAS,
    )
}

pub fn main() {
    let args = aoc::parse_args();
    args.run(solve);
}
