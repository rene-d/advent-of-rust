use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    println!("Advent of Rust - Day 2");

    let mut x_position = 0;
    let mut y_position = 0;
    let mut aim = 0;

    if let Ok(lines) = read_lines("input.txt") {
        for line in lines {
            if let Ok(value) = line {

                // On ne gère pas le cas ou l'input n'aurait pas le bon format, ce n'est pas le but ici
                let (action, distance) = parse_line(&value);
                
                match action {
                    "forward" => {
                        x_position = x_position + distance;
                        y_position = y_position + (aim * distance);
                    },
                    "up" => aim = aim - distance,
                    "down" => aim = aim + distance,
                    _ => (),
                }                
            }
        }
        println!("Result => {} [x={}; y={}; aim={}] ", x_position*y_position, x_position, y_position, aim);
    }
}

fn parse_line(line: &String) -> (&str, i32) {

    let bytes = line.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            let action = &line[0..i];
            let distance: i32 = (&line[i+1..]).parse().expect("Error - Cannot parse distance to integer");
            return (action, distance);
        }
    }

    // Erreur
    (&line[..], -1)
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}