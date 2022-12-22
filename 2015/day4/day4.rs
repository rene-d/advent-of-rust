//! [Day 4: The Ideal Stocking Stuffer](https://adventofcode.com/2015/day/4)

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
    // println!("reading data from: {}", args.path.display());
    let data = std::fs::read_to_string(args.path).unwrap();

    let mut key = 0;

    loop {
        let key_str = key.to_string();
        let mut x = data.clone();

        x.push_str(&key_str);
        let digest = md5::compute(x);

        if format!("{:x}", digest).starts_with("00000") {
            println!("{}\t{:?}", key, digest);
            break;
        }

        key += 1;
    }

    loop {
        let key_str = key.to_string();
        let mut x = data.clone();

        x.push_str(&key_str);
        let digest = md5::compute(x);

        if format!("{:x}", digest).starts_with("000000") {
            println!("{}\t{:?}", key, digest);
            break;
        }

        key += 1;
    }
}
