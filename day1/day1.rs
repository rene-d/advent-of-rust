use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {

    let mut prev_num = 999999999u32;
    let mut result = 0;

    if let Ok(lines) = read_lines("input.txt") {

        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(line_ok) = line {
                // convertit string -> u32
                let num: u32 = line_ok.parse().unwrap();

                // est-ce que on est en "increase" ?
                if prev_num < num {
                    result += 1;
                }

                prev_num = num;
            }
        }
    }

    println!("{}", result);
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
