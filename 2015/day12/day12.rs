use regex::Regex;
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

fn sum(v: &serde_json::Value) -> i32 {
    match v {
        serde_json::Value::Number(n) => n.as_i64().unwrap().try_into().unwrap(),
        serde_json::Value::Array(a) => a.iter().map(sum).sum(),
        serde_json::Value::Object(o) => {
            // Ignore any object (and all of its children) which has any property with the value "red".
            for v in o.values() {
                if v.as_str() == Some("red") {
                    return 0;
                }
            }
            o.values().map(sum).sum()
        }
        _ => 0,
    }
}

/// main function
fn main() {
    let args = Cli::from_args();
    println!("reading data from: {}", args.path.display());
    let data = load_data(args.path);

    // part 1
    let re = Regex::new(r"(\-?\d+)").unwrap();
    let part1 = &data
        .iter()
        .map(|line| {
            re.find_iter(line)
                .map(|m| m.as_str().parse::<i32>().unwrap())
                .sum::<i32>()
        })
        .sum::<i32>();
    println!("{}", part1);

    // part 2
    let json: serde_json::Value =
        serde_json::from_str(&data[0]).expect("JSON was not well-formatted");
    println!("{}", sum(&json));
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
