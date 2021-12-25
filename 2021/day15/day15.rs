// Day 15: Chiton
// https://adventofcode.com/2021/day/15

use std::collections::BinaryHeap;
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

    // read the grid
    let n = data.len();
    let mut grid = vec![vec![0i32; n]; n];
    for (y, line) in data.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            grid[y][x] = c.to_digit(10).unwrap() as i32;
        }
    }

    // step 1
    println!("{:?}", min_cost_dp(&grid) - grid[0][0]);
    println!("{:?}", min_cost(&grid) - grid[0][0]);

    // build the five times larger grid
    let mut grid5 = vec![vec![0i32; 5 * n]; 5 * n];
    for y in 0..n {
        for x in 0..n {
            let v = grid[y][x];

            for yy in 0..5 {
                for xx in 0..5 {
                    grid5[y + n * yy][x + n * xx] = (v - 1 + (xx + yy) as i32) % 9 + 1;
                }
            }
        }
    }

    // // step 2
    println!("{:?}", min_cost_dp(&grid5) - grid5[0][0]);
    println!("{:?}", min_cost(&grid5) - grid5[0][0]);
}

struct Cost {
    cost: i32,
    x: usize,
    y: usize,
}

impl Ord for Cost {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Cost {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Cost {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}

impl Eq for Cost {}

fn min_cost(grid: &Vec<Vec<i32>>) -> i32 {
    let n = grid.len();
    let mut d = vec![vec![0i32; n]; n];

    let mut heap = BinaryHeap::new();

    heap.push(Cost {
        cost: 0,
        x: 0,
        y: 0,
    });

    while !heap.is_empty() {
        let cur = heap.pop().unwrap();

        let new_cost = cur.cost + grid[cur.y][cur.x];

        if new_cost >= d[cur.y][cur.x] && d[cur.y][cur.x] != 0 {
            continue;
        }

        d[cur.y][cur.x] = new_cost;

        if cur.x == n - 1 && cur.y == n - 1 {
            break;
        }

        if cur.x + 1 < n {
            heap.push(Cost {
                cost: new_cost,
                x: cur.x + 1,
                y: cur.y,
            });
        }
        if cur.y + 1 < n {
            heap.push(Cost {
                cost: new_cost,
                x: cur.x,
                y: cur.y + 1,
            });
        }
        if cur.x > 0 {
            heap.push(Cost {
                cost: new_cost,
                x: cur.x - 1,
                y: cur.y,
            });
        }
        if cur.y > 0 {
            heap.push(Cost {
                cost: new_cost,
                x: cur.x,
                y: cur.y - 1,
            });
        }
    }

    d[n - 1][n - 1]
}

fn min_cost_dp(grid: &Vec<Vec<i32>>) -> i32 {
    let n = grid.len();
    let mut cost = vec![vec![0i32; n]; n];

    for y in 0..n {
        for x in 0..n {
            if x == 0 && y == 0 {
                cost[y][x] = grid[y][x];
            } else if x == 0 {
                cost[y][x] = grid[y][x] + cost[y - 1][x];
            } else if y == 0 {
                cost[y][x] = grid[y][x] + cost[y][x - 1];
            } else {
                cost[y][x] = grid[y][x] + min(cost[y - 1][x], cost[y][x - 1]);
            }
        }
    }

    cost[n - 1][n - 1]
}

fn min(a: i32, b: i32) -> i32 {
    if a < b {
        a
    } else {
        b
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
