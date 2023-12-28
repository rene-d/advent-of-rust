//! [Day 15: Science for Hungry People](https://adventofcode.com/2015/day/15)

use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use structopt::StructOpt;

#[derive(Debug)]
struct Ingredient {
    // name: String,
    capacity: i64,
    durability: i64,
    flavor: i64,
    texture: i64,
    calories: i64,
}

/// parse command line arguments
#[derive(StructOpt)]
struct Cli {
    #[structopt(default_value = "input.txt", parse(from_os_str))]
    path: std::path::PathBuf,
}

/// main function
fn main() {
    let args = Cli::from_args();
    // println!("reading data from: {}", args.path.display());
    let data = load_data(args.path);

    // load data
    let mut ingredients = Vec::new();
    let re = Regex::new(r"(\w+): capacity (-?\d+), durability (-?\d+), flavor (-?\d+), texture (-?\d+), calories (-?\d+)").unwrap();

    for line in &data {
        re.captures(line).map(|cap| {
            let ingredient = Ingredient {
                // name: cap[1].to_string(),
                capacity: cap[2].parse().unwrap(),
                durability: cap[3].parse().unwrap(),
                flavor: cap[4].parse().unwrap(),
                texture: cap[5].parse().unwrap(),
                calories: cap[6].parse().unwrap(),
            };
            // println!("{:?}", ingredient);
            ingredients.push(ingredient);
            0
        });
    }

    for part in 1..=2 {
        let mut score_max = 0;

        // we can deal at most 4 ingredients
        for a in 0..100 {
            for b in 0..100 {
                for c in 0..100 {
                    let d = 100 - (a + b + c);

                    let quantity = [a, b, c, d];

                    // take into account the number of ingredients
                    if quantity.iter().take(ingredients.len()).sum::<i64>() != 100 {
                        continue;
                    }

                    // in the second part of the puzzle, we need to make sure that the calories are 500
                    if part == 2
                        && quantity
                            .iter()
                            .take(ingredients.len())
                            .enumerate()
                            .map(|(k, &x)| x * ingredients[k].calories)
                            .sum::<i64>()
                            != 500
                    {
                        continue;
                    }

                    let capacity = ingredients
                        .iter()
                        .enumerate()
                        .map(|(k, i)| i.capacity * quantity[k])
                        .sum::<i64>();

                    let durability = ingredients
                        .iter()
                        .enumerate()
                        .map(|(k, i)| i.durability * quantity[k])
                        .sum::<i64>();

                    let flavor = ingredients
                        .iter()
                        .enumerate()
                        .map(|(k, i)| i.flavor * quantity[k])
                        .sum::<i64>();

                    let texture = ingredients
                        .iter()
                        .enumerate()
                        .map(|(k, i)| i.texture * quantity[k])
                        .sum::<i64>();

                    // if any score if negative, ignore the recipe
                    if capacity < 0 || durability < 0 || flavor < 0 || texture < 0 {
                        continue;
                    }

                    let score = capacity * durability * flavor * texture;
                    if score > score_max {
                        score_max = score;
                        // println!("{}: {:?} - {} * {} * {} * {}", score_max, quantity, capacity, durability, flavor, texture);
                    }
                }
            }
        }

        println!("{score_max}");
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

/// load data from file
fn load_data(path: std::path::PathBuf) -> Vec<String> {
    let mut data = vec![];
    if let Ok(lines) = read_lines(path) {
        for line in lines.flatten() {
            data.push(line);
        }
    }
    data
}
