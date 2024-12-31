//! [Day 10: Elves Look, Elves Say](https://adventofcode.com/2015/day/10)

/// main function
fn main() {
    let args = aoc::parse_args();
    let data = args
        .input
        .lines()
        .map(std::string::ToString::to_string)
        .collect::<Vec<String>>();

    solve(&data[0], 40);
    solve(&data[0], 50);
}

fn solve(start_sequence: &str, turns: u32) {
    let mut look = start_sequence.chars().collect::<Vec<char>>();

    for _ in 0..turns {
        let mut say: Vec<char> = Vec::new();

        let mut count = 0;
        let mut previous = '\0';
        for current in &look {
            if previous != '\0' && previous != *current {
                say.extend(count.to_string().chars());
                say.push(previous);
                count = 0;
            }
            count += 1;
            previous = *current;
        }

        say.extend(count.to_string().chars());
        say.push(previous);

        look.clone_from(&say);
    }
    println!("{}", look.len());
}
