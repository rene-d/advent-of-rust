//! [Day 10: Elves Look, Elves Say](https://adventofcode.com/2015/day/10)

/// main function
fn main() {
    let data = aoc::load_input_data_vec(10);

    solve(&data[0], 40);
    solve(&data[0], 50);
}

fn solve(start_sequence: &str, turns: u32) {
    let mut look = start_sequence.chars().collect::<Vec<char>>();

    for _ in 0..turns {
        let mut say: Vec<char> = Vec::new();

        let mut count = 0;
        let mut previous = '\0';
        for current in look {
            if previous != '\0' && previous != current {
                say.extend(count.to_string().chars());
                say.push(previous);
                count = 0;
            }
            count += 1;
            previous = current;
        }

        say.extend(count.to_string().chars());
        say.push(previous);

        look = say.clone();
    }
    println!("{}", look.len());
}
