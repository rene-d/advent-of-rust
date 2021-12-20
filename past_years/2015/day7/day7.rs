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

    let mut registers: HashMap<String, u16> = HashMap::new();

    let re_num = Regex::new(r"^(\d+) \-> (\w+)$").unwrap();
    let re_copy = Regex::new(r"^(\w+) \-> (\w+)$").unwrap();
    let re_binary = Regex::new(r"^(\w+) (AND|OR) (\w+) \-> (\w+)$").unwrap();
    let re_unary = Regex::new(r"^(NOT) (\w+) \-> (\w+)$").unwrap();
    let re_shift = Regex::new(r"^(\w+) (RSHIFT|LSHIFT) (\d+) \-> (\w+)$").unwrap();

    for line in &data {
        if let Some(op) = re_num.captures(&line) {
            // 123 -> x

            let value = op[1].parse::<u16>().unwrap();
            let dst = op[2].to_string();
            registers.insert(dst, value);


        } else if let Some(op) = re_copy.captures(&line) {
            let src = op[1].to_string();
            let dst = op[2].to_string();

            let v = registers.get(&src).cloned().unwrap_or(0);
            registers.insert(dst, v);


        } else if let Some(op) = re_binary.captures(&line) {
            // println!("opbinary {}", line);

            let src1 = op[1].to_string();
            let opx = op[2].to_string();
            let src2 = op[3].to_string();
            let dst = op[4].to_string();

            let v1 = registers.get(&src1).cloned().unwrap_or(0);
            let v2 = registers.get(&src2).cloned().unwrap_or(0);

            let v;
            if opx == "AND" {
                v = v1 & v2;
            } else {
                v = v1 | v2;
            }

            registers.insert(dst, v);


        } else if let Some(op) = re_unary.captures(&line) {
            // println!("opunary {}", line);

            let src = op[2].to_string();
            let dst = op[3].to_string();

            let v = registers.get(&src).cloned().unwrap_or(0);

            registers.insert(dst, ! v );


        } else if let Some(op) = re_shift.captures(&line) {
            // println!("opshift {}", line);

            let src = op[1].to_string();
            let opx = op[2].to_string();
            let shift = op[3].parse::<u8>().unwrap();
            let dst = op[4].to_string();

            let v = registers.get(&src).cloned().unwrap_or(0);
            if v != 0 {
                println!("shift by zero {} {} {}", v, v << shift, v >> shift);
            }
            match opx.as_ref() {
                "RSHIFT" => {
                    registers.insert(dst, v >> shift);
                }
                "LSHIFT" => {
                    registers.insert(dst, v << shift);
                }
                _ => {
                    panic!("unknown shift operation");
                }
            }
        } else {
            println!("{}", line);
            panic!("unknown operation");
        }
        //println!("{}", line);
    }

    // println!("{:?}", registers);
    for (k,v) in &registers {
        println!("{} = {}", k, v);
    }

    println!("{:?}", registers.get("a").cloned().unwrap_or(0));
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
