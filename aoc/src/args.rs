use colored::Colorize;
use std::{env::args, time::Instant};

use crate::load_input_data;

#[derive(Debug)]
pub struct Args {
    pub input: String,
    pub verbose: bool,
    elapsed: bool,
    instant: Instant,
}

impl Args {
    #[must_use]
    pub fn parse_args() -> Self {
        let help = args().any(|a| a == "--help" || a == "-h");
        if help {
            usage();
        }

        let filename = args()
            .skip(1)
            .find(|a| !a.starts_with('-'))
            .map_or("input.txt".to_string(), |a| a);

        let verbose = args().any(|a| a == "--verbose" || a == "-v");
        let elapsed = args().any(|a| a == "--elapsed");

        let input = load_input_data(&filename);

        Self {
            input,
            verbose,
            elapsed,
            instant: Instant::now(),
        }
    }

    fn elapsed(&self) -> String {
        let micros: u128 = self.instant.elapsed().as_micros();
        format!("elapsed: {}.{:03} ms", micros / 1000, micros % 1000)
    }

    #[must_use]
    pub fn has_option(option: &str) -> bool {
        args().any(|a| a == option)
    }
}

/// Automatically print the elapsed duration if asked
impl Drop for Args {
    fn drop(&mut self) {
        if self.elapsed {
            println!("{}", self.elapsed());
        }
    }
}

/// Show command-line usage.
fn usage() {
    let name = std::env::current_exe()
        .unwrap()
        .file_name()
        .unwrap()
        .to_os_string();
    let name = name.to_str().unwrap();

    println!("Advent of Code's puzzle solver");
    println!();
    println!(
        "{} {} {}",
        "Usage:".green().bold(),
        name.cyan().bold(),
        "[OPTIONS] [INPUT]".cyan()
    );
    println!();
    println!("{}", "Options:".green().bold());
    println!(
        "  {}, {}           Use verbose output",
        "-v".cyan().bold(),
        "--verbose".cyan().bold()
    );
    println!(
        "      {}          Show duration",
        "--duration".cyan().bold()
    );

    std::process::exit(0); //std::process::ExitCode::SUCCESS);
}
