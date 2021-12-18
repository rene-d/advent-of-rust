#![allow(unused_imports)]
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

    let mut total_paper = 0;
    let mut total_ribbon = 0;

    for line in data {
        let mut dimensions = line
            .split("x")
            .map(|s| s.parse::<u32>().unwrap())
            .collect::<Vec<_>>();

        dimensions.sort(); // trick for rubbon computation (no matter order of l,w,h)

        if dimensions.len() != 3 {
            continue;
        }

        // dimensions (length l, width w, and height h) of each present
        let l = dimensions[0];
        let w = dimensions[1];
        let h = dimensions[2];

        // required wrapping paper for each gift a little easier: find the surface area of the box, which is 2*l*w + 2*w*h + 2*h*l
        let paper = 2 * l * w + 2 * w * h + 2 * h * l;

        // little extra paper for each present: the area of the smallest side.
        let slack = std::cmp::min(l * w, std::cmp::min(w * h, h * l));

        // total square feet of wrapping paper
        total_paper += paper + slack;

        // The ribbon required to wrap a present is the shortest distance around its sides, or the smallest perimeter of any one face
        let ribbon = 2 * l + 2 * w;

        // the perfect bow is equal to the cubic feet of volume of the present
        let bow = l * w * h;

        total_ribbon += ribbon + bow;
    }

    println!("{}", total_paper);
    println!("{}", total_ribbon);
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
        for line in lines {
            if let Ok(bits) = line {
                data.push(bits);
            }
        }
    }
    data
}