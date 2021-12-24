use permutator::LargeCombinationIterator;
use std::collections::HashMap;
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

    let values: Vec<u32> = data
        .iter()
        .map(|s| s.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    let mut part1 = 0;
    let mut part2: HashMap<usize, usize> = HashMap::new();
    for i in 1..=values.len() {
        let combinator = LargeCombinationIterator::new(&values, i);
        for c in combinator {
            let mut sum = 0;
            for j in &c {
                sum += j.clone();
            }
            if sum == 150 {
                part1 += 1;
                let sum = c.len();
                let y = part2.entry(c.len()).or_default().clone(); //TODO?
                part2.insert(c.len(), y + 1);
            }
        }
    }
    println!("{}", part1);
    println!("{:?}", part2); //TODO
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
