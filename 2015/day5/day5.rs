use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use structopt::StructOpt;

/// parse command line arguments
#[derive(StructOpt)]
struct Cli {
    #[structopt(default_value = "input.txt", parse(from_os_str))]
    path: std::path::PathBuf,
}

/// main function
fn main() {
    let args = Cli::from_args();
    println!("reading data from: {}", args.path.display());
    let data = load_data(args.path);

    part1(&data);
    part2(&data);
}

fn part2(data: &[String]) {
    let mut nice_words = 0;
    for word in data {
        // pair of any two letters that appears at least twice in the string without overlapping
        let mut twice = false;
        for i in 0..word.len() - 3 {
            let c = word.chars().nth(i).unwrap();
            let d = word.chars().nth(i + 1).unwrap();

            for j in i + 2..word.len() - 1 {
                if c == word.chars().nth(j).unwrap() && d == word.chars().nth(j + 1).unwrap() {
                    twice = true;
                    break;
                }
            }

            if twice {
                break;
            }
        }

        if !twice {
            continue;
        }

        // letter which repeats with exactly one letter between them
        for i in 0..word.len() - 2 {
            if word.chars().nth(i).unwrap() == word.chars().nth(i + 2).unwrap() {
                nice_words += 1;
                break;
            }
        }
    }

    println!("nice words: {}", nice_words);
}

fn part1(data: &[String]) {
    let mut nice_words = 0;
    for word in data {
        if word.contains("ab") || word.contains("cd") || word.contains("pq") || word.contains("xy")
        {
            continue;
        }

        let mut vowel = 0;
        for c in word.chars() {
            if "aeiou".contains(c) {
                vowel += 1;
            }
        }
        if vowel < 3 {
            continue;
        }

        // finally, check the repeating letters
        for i in 0..word.len() - 1 {
            if word.chars().nth(i).unwrap() == word.chars().nth(i + 1).unwrap() {
                nice_words += 1;
                break;
            }
        }
    }

    println!("nice words: {}", nice_words);
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
