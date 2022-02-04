/*!
[Day 15: Timing is Everything](https://adventofcode.com/2016/day/15)


*/
use regex::Regex;

fn main() {
    let input_file = std::env::args().nth(1).unwrap_or_else(|| "input.txt".to_string());
    let data = std::fs::read_to_string(input_file).unwrap();

    let mut table_a = Vec::new();
    let mut table_n = Vec::new();

    for (i, line) in data.lines().enumerate() {
        let re = Regex::new(r"Disc #\d+ has (\d+) positions; at time=0, it is at position (\d+).").unwrap();

        let caps = re.captures(line).unwrap();

        let a = caps[2].parse::<usize>().unwrap();
        let n = caps[1].parse::<usize>().unwrap();

        println!("{}: {}  -->  {:?}", i, line, (a, n));

        table_a.push(a);
        table_n.push(n);
    }

    // part 1
    let nb = table_a.len();
    for time in 0.. {
        if (0..nb).all(|i| (table_a[i] + i + time + 1) % table_n[i] == 0) {
            println!("{}", time);
            break;
        }
    }

    // part 2
    table_a.push(0);
    table_n.push(11);

    let nb = table_a.len();
    for time in 0.. {
        if (0..nb).all(|i| (table_a[i] + i + time + 1) % table_n[i] == 0) {
            println!("{}", time);
            break;
        }
    }
}
