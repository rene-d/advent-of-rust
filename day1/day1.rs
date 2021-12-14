use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    // file
    let filename = "input.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    // vector to store data
    let mut vec: Vec<i32> = Vec::new();

    // read line into vector
    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();

        let val: i32 = line.parse().unwrap();
        vec.push(val);

        //println!("{}: {}", _index + 1, line);
    }

    // ----------
    // step1

    // not optimal loop for vector
    let mut index = 0;
    let mut count = 0;
    println!("0: {} (N/A)", vec[0]);
    while index < vec.len() - 1 {
        if vec[index] < vec[index + 1] {
            count += 1;
            println!("{}: {} (increased)", index + 1, vec[index + 1]);
        }
        if vec[index] > vec[index + 1] {
            println!("{}: {} (decreased)", index + 1, vec[index + 1]);
        }
        if vec[index] == vec[index + 1] {
            println!("{}: {} (no change)", index + 1, vec[index + 1]);
        }
        index += 1;
    }
    println!("{} ", count);

    // ----------
    // step2

    let mut prev = 0;
    index = 0;
    count = 0;
    while index < vec.len() - 3 {
        let sum = vec[index] + vec[index + 1] + vec[index + 2];
        if sum > prev {
            count += 1;
        }
        index += 1;
        prev = sum;
    }
    println!("{} ", count);
}
