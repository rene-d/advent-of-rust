use colored::Colorize;
use std::time::Instant;

use crate::load_input_data;

#[derive(Debug)]
pub struct Args {
    pub input: String,
    pub verbose: bool,
    elapsed: bool,
    instant: Instant,
    pub has_option: fn(&str) -> bool,
}

impl Args {
    #[must_use]
    pub fn parse_args() -> Self {
        let help = std::env::args().any(|a| a == "--help" || a == "-h");
        if help {
            usage();
        }

        let filename = std::env::args()
            .skip(1)
            .find(|a| !a.starts_with('-'))
            .map_or("input.txt".to_string(), |a| a);

        let verbose = std::env::args().any(|a| a == "--verbose" || a == "-v");
        let elapsed = std::env::args().any(|a| a == "--elapsed");
        let input = load_input_data(&filename);
        let has_option = |option: &str| -> bool { std::env::args().any(|a| a == option) };

        Self {
            input,
            verbose,
            elapsed,
            instant: Instant::now(),
            has_option,
        }
    }

    fn elapsed(&self) -> String {
        let micros: u128 = self.instant.elapsed().as_micros();
        format!("elapsed: {}.{:03} ms", micros / 1000, micros % 1000)
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
    println!("      {}          Show duration", "--elapsed".cyan().bold());

    std::process::exit(0); //std::process::ExitCode::SUCCESS);
}
