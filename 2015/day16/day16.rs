use regex::Regex;
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

    let re = Regex::new(r"Sue (\d+): (\w+): (\d+), (\w+): (\d+), (\w+): (\d+)").unwrap();

    let mut aunts: HashMap<u32, HashMap<String, u32>> = HashMap::new();

    for line in &data {
        let m = re.captures(line).unwrap();
        let sue = m.get(1).unwrap().as_str().parse::<u32>().unwrap();

        for i in (2..=6).step_by(2) {
            let key = m.get(i).unwrap().as_str();
            let value = m.get(i + 1).unwrap().as_str().parse::<u32>().unwrap();
            aunts
                .entry(sue)
                .or_insert_with(HashMap::new)
                .insert(key.to_string(), value);
        }
    }

    // part 1
    for (sue, aunt) in &aunts {
        if aunt.get("children").unwrap_or(&3) == &3
            && aunt.get("cats").unwrap_or(&7) == &7
            && aunt.get("samoyeds").unwrap_or(&2) == &2
            && aunt.get("pomeranians").unwrap_or(&3) == &3
            && aunt.get("akitas").unwrap_or(&0) == &0
            && aunt.get("vizslas").unwrap_or(&0) == &0
            && aunt.get("goldfish").unwrap_or(&5) == &5
            && aunt.get("trees").unwrap_or(&3) == &3
            && aunt.get("cars").unwrap_or(&2) == &2
            && aunt.get("perfumes").unwrap_or(&1) == &1
        {
            println!("{}", sue);
        }
    }

    // part 2
    for (sue, aunt) in &aunts {
        if aunt.get("children").unwrap_or(&3) == &3
            && aunt.get("cats").unwrap_or(&8) > &7          // should be greater than
            && aunt.get("samoyeds").unwrap_or(&2) == &2
            && aunt.get("pomeranians").unwrap_or(&2) < &3   // should be fewer than
            && aunt.get("akitas").unwrap_or(&0) == &0
            && aunt.get("vizslas").unwrap_or(&0) == &0
            && aunt.get("goldfish").unwrap_or(&4) < &5      // should be fewer than
            && aunt.get("trees").unwrap_or(&4) > &3         // should be greater than
            && aunt.get("cars").unwrap_or(&2) == &2
            && aunt.get("perfumes").unwrap_or(&1) == &1
        {
            println!("{}", sue);
        }
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
