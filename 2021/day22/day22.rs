//! [Day 22: Reactor Reboot](https://adventofcode.com/2021/day/22)

use indicatif::ProgressBar;
use regex::Regex;
use std::cmp::max;
use std::cmp::min;
use std::collections::HashSet;
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
    // println!("reading data from: {}", args.path.display());
    let data = load_data(args.path);

    part1(&data);
    part2(&data);
}

fn part2(data: &[String]) {
    let re =
        Regex::new(r"^(on|off) x=(-?\d+)\.\.(-?\d+),y=(-?\d+)\.\.(-?\d+),z=(-?\d+)\.\.(-?\d+)$")
            .unwrap();

    let mut x_slices: HashSet<i32> = HashSet::new();
    let mut y_slices: HashSet<i32> = HashSet::new();
    let mut z_slices: HashSet<i32> = HashSet::new();

    let mut cubes: Vec<(i32, i32, i32, i32, i32, i32, bool)> = Vec::new();

    for line in data {
        let cube = re.captures(line).unwrap();

        let flag_on = cube[1].to_string() == "on";

        let x1 = cube[2].parse::<i32>().unwrap();
        let x2 = cube[3].parse::<i32>().unwrap();
        let y1 = cube[4].parse::<i32>().unwrap();
        let y2 = cube[5].parse::<i32>().unwrap();
        let z1 = cube[6].parse::<i32>().unwrap();
        let z2 = cube[7].parse::<i32>().unwrap();

        assert!(x1 <= x2);
        assert!(y1 <= y2);
        assert!(z1 <= z2);

        cubes.push((x1, x2, y1, y2, z1, z2, flag_on));

        x_slices.insert(x1);
        x_slices.insert(x2 + 1);
        y_slices.insert(y1);
        y_slices.insert(y2 + 1);
        z_slices.insert(z1);
        z_slices.insert(z2 + 1);
    }

    let mut xx: Vec<i32> = x_slices.iter().copied().collect();
    let mut yy: Vec<i32> = y_slices.iter().copied().collect();
    let mut zz: Vec<i32> = z_slices.iter().copied().collect();

    xx.sort_unstable();
    yy.sort_unstable();
    zz.sort_unstable();

    let mut size: i64 = 0;

    let bar = ProgressBar::new((xx.len() - 1) as u64);

    for x in 0..xx.len() - 1 {
        bar.inc(1);

        for y in 0..yy.len() - 1 {
            for z in 0..zz.len() - 1 {
                let x1 = xx[x];
                let x2 = xx[x + 1];
                let y1 = yy[y];
                let y2 = yy[y + 1];
                let z1 = zz[z];
                let z2 = zz[z + 1];

                let mut sign = false;

                for cube in &cubes {
                    if (cube.0 <= x1 && x2 - 1 <= cube.1)
                        && (cube.2 <= y1 && y2 - 1 <= cube.3)
                        && (cube.4 <= z1 && z2 - 1 <= cube.5)
                    {
                        sign = cube.6;
                    }
                }
                if sign {
                    size += (i64::try_from(x2 - x1).unwrap())
                        * (i64::try_from(y2 - y1).unwrap())
                        * (i64::try_from(z2 - z1).unwrap());
                }
            }
        }
    }
    bar.finish();

    println!("part2: {}", size);
}

fn part1(data: &[String]) {
    let re =
        Regex::new(r"^(on|off) x=(-?\d+)\.\.(-?\d+),y=(-?\d+)\.\.(-?\d+),z=(-?\d+)\.\.(-?\d+)$")
            .unwrap();

    let mut cubes_on: HashSet<(i32, i32, i32)> = HashSet::new();

    for line in data {
        let cube = re.captures(line).unwrap();

        let flag_on = cube[1].to_string() == "on";

        let x1 = cube[2].parse::<i32>().unwrap();
        let x2 = cube[3].parse::<i32>().unwrap();
        let y1 = cube[4].parse::<i32>().unwrap();
        let y2 = cube[5].parse::<i32>().unwrap();
        let z1 = cube[6].parse::<i32>().unwrap();
        let z2 = cube[7].parse::<i32>().unwrap();

        for x in max(-50, x1)..=min(50, x2) {
            for y in max(-50, y1)..=min(50, y2) {
                for z in max(-50, z1)..=min(50, z2) {
                    if flag_on {
                        cubes_on.insert((x, y, z));
                    } else {
                        cubes_on.remove(&(x, y, z));
                    }
                }
            }
        }
    }

    println!("part1: {}", cubes_on.len());
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
