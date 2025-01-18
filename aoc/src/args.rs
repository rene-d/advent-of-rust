use colored::Colorize;
use std::{
    fmt::Display,
    time::{Duration, Instant},
};

use crate::load_input_data;

#[derive(Debug)]
pub struct Args {
    pub input: String,    // puzzle input
    pub verbose: bool,    // activate the verbose flag
    options: Vec<String>, // copy of Args()
    elapsed: bool,        // show elspaed time on exit
    instant: Instant,     // time after input read
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
        let options = std::env::args().collect();

        let instant = Instant::now();

        Self {
            input,
            verbose,
            options,
            elapsed,
            instant,
        }
    }

    pub fn has_option(&self, option: &str) -> bool {
        self.options.iter().filter(|s| *s == option).count() != 0
        //self.options.contains(option)
    }
}

/// Automatically print the elapsed duration if asked
impl Drop for Args {
    fn drop(&mut self) {
        if self.elapsed {
            #[allow(clippy::cast_possible_truncation)]
            let micros = Duration::from_micros(self.instant.elapsed().as_micros() as u64);
            println!("elapsed:  {micros:?}");
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
        "  {}, {}          Use verbose output",
        "-v".cyan().bold(),
        "--verbose".cyan().bold()
    );
    println!("      {}          Show duration", "--elapsed".cyan().bold());

    std::process::exit(0); //std::process::ExitCode::SUCCESS);
}

impl Args {
    pub fn run<U, V, T>(&mut self, solve: T)
    where
        U: Display,
        V: Display,
        T: Fn(&str) -> (U, V),
    {
        let instant = Instant::now();

        let (p1, p2) = solve(&self.input);

        #[allow(clippy::cast_possible_truncation)]
        let micros = Duration::from_micros(instant.elapsed().as_micros() as u64);

        println!("{p1}");

        // day 25 should print only one answer
        let p2 = format!("{p2}");
        if !p2.is_empty() {
            println!("{p2}");
        }

        if self.elapsed {
            println!("elapsed: {micros:?}");
            self.elapsed = false;
        }
    }
}
