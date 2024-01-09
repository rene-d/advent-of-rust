//! Utility functions

use clap::Parser;

#[derive(Parser)]
pub struct Args {
    /// Puzzle input
    #[arg(default_value = "input.txt")]
    pub path: String,

    /// Verbose output
    #[arg(short, long)]
    pub verbose: bool,
}

/// Parse the command line arguments for puzzle.
pub fn parse_args() -> Args {
    let args = Args::parse();
    args
}

/// Read the puzzle input
/// # Panics
/// If the file cannot be found or read
#[must_use]
pub fn load_input_data(day: u8) -> String {
    let args = parse_args();

    let mut filename = if args.path == "-" {
        "/dev/stdin".to_string()
    } else {
        args.path
    };

    if filename == "input.txt" && !std::path::Path::new(&filename).is_file() {
        let txt = format!("day{day}/input.txt");
        if std::path::Path::new(&txt).is_file() {
            filename = txt;
        }
    }

    std::fs::read_to_string(filename).unwrap()
}

pub fn load_input_data_vec(day: u8) -> Vec<String> {
    load_input_data(day).lines().map(String::from).collect()
}

pub mod grid;
pub mod hex;
pub mod knot;
pub mod ocr;
