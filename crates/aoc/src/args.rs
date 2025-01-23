use colored::Colorize;
use std::{
    fmt::Display,
    time::{Duration, Instant},
};

use crate::load_input_data;

#[derive(Debug)]
pub struct Args {
    pub input: String,       // puzzle input
    pub verbose: bool,       // activate the verbose flag
    options: Vec<String>,    // copy of Args() (with a leading -)
    pub params: Vec<String>, // copy of Args() (without the leading -)
    elapsed: bool,           // flag to show elapsed time
}

impl Args {
    #[must_use]
    pub fn parse_args() -> Self {
        let mut args = Self::parse_args_raw();

        let path = args.params.first().map_or("input.txt", |f| f.as_str());

        args.input = load_input_data(path);

        args
    }

    #[must_use]
    pub fn parse_args_raw() -> Self {
        let help = std::env::args().any(|a| a == "--help" || a == "-h");
        if help {
            usage();
        }

        let verbose = std::env::args().any(|a| a == "--verbose" || a == "-v");
        let elapsed = std::env::args().any(|a| a == "--elapsed");

        let options = std::env::args().filter(|a| a.starts_with('-')).collect();
        let params: Vec<String> = std::env::args()
            .skip(1)
            .filter(|a| !a.starts_with('-'))
            .collect();

        Self {
            input: String::new(),
            verbose,
            options,
            params,
            elapsed,
        }
    }

    pub fn has_option(&self, option: &str) -> bool {
        self.options.iter().filter(|s| *s == option).count() != 0
        //self.options.contains(option)
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
    pub fn run<U, V, T>(&self, solve: T)
    where
        U: Display,
        V: Display,
        T: Fn(&str) -> (U, V),
    {
        self.run_data(solve, &self.input);
    }

    pub fn run_data<U, V, T>(&self, solve: T, data: &str)
    where
        U: Display,
        V: Display,
        T: Fn(&str) -> (U, V),
    {
        let instant = Instant::now();

        let (p1, p2) = solve(data);

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
        }
    }
}
