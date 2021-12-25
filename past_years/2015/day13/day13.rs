use permutator::HeapPermutationIterator;
use regex::Regex;
use std::collections::{HashMap, HashSet};
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

    let mut names: HashSet<String> = HashSet::new();
    let mut happyness: HashMap<(String, String), i32> = HashMap::new();

    let re =
        Regex::new(r"^(.+) would (gain|lose) (\d+) happiness units by sitting next to (.+)\.$")
            .unwrap();

    for line in &data {
        if let Some(op) = re.captures(line) {
            names.insert(op[1].to_string());
            names.insert(op[4].to_string());

            let mut gain: i32 = op[3].parse().unwrap();
            if op[2].to_string() == "lose" {
                gain = -gain;
            }

            happyness.insert((op[1].to_string(), op[4].to_string()), gain);
        }
    }

    fn calc(names: &HashSet<String>, happyness: &HashMap<(String, String), i32>) -> i32 {
        let perm_names = &mut names.iter().collect::<Vec<&String>>();
        let permutator = HeapPermutationIterator::new(perm_names);

        let mut happyness_max = std::i32::MIN;

        for permutated in permutator {
            let mut happyness_sum = 0;
            for i in 0..permutated.len() {
                let n1 = permutated[i];
                let n2 = permutated[(i + 1) % permutated.len()];

                happyness_sum += happyness.get(&(n1.to_string(), n2.to_string())).unwrap();
                happyness_sum += happyness.get(&(n2.to_string(), n1.to_string())).unwrap();
            }

            if happyness_max < happyness_sum {
                happyness_max = happyness_sum;
            }
        }
        happyness_max
    }

    // part 1
    println!("{}", calc(&names, &happyness));

    // part 2
    for name in &names {
        happyness.insert((name.to_string(), "me".to_string()), 0);
        happyness.insert(("me".to_string(), name.to_string()), 0);
    }
    names.insert("me".to_string());

    println!("{}", calc(&names, &happyness));
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
