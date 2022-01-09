// Day 10: Passage Pathing
// https://adventofcode.com/2021/day/12

use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

/// main function
fn main()
{
    let data = load_data("input.txt");

    // Map containing each cave and its neighbors as a list
    let mut map: HashMap<String, Vec<String>> = HashMap::new();

    // Fill the map with caves
    for entry in data
    {
        // Split line between "left" cave and "right" cave
        let mut split = entry.split('-');
        let left = split.next().unwrap();
        let right = split.next().unwrap();

        // Update map with eventual new caves and neighbors
        let left_cave = map.entry(String::from(left)).or_insert(Vec::new());
        left_cave.push(String::from(right));

        let right_cave = map.entry(String::from(right)).or_insert(Vec::new());
        right_cave.push(String::from(left));
    }

    // Initialize paths list
    let mut path_list = vec![(String::from("start"), vec![String::from("start")])];
    let mut path_count = 0;

    while !path_list.is_empty()
    {
        let path = path_list.pop().unwrap();
        let node = path.0;

        if node == "end"
        {
            // Count this path and let it be removed from the paths list
            path_count += 1;
        }
        else
        {
            // Get neighbors from last node
            let neighbor_list = map.get(&node).unwrap();

            for neighbor in neighbor_list
            {
                if neighbor.to_uppercase() == String::from(neighbor)
                {
                    // We add this new path
                    let mut path_new = path.1.clone();

                    path_new.push(String::from(neighbor));
                    path_list.push((String::from(neighbor), path_new));
                }
                else
                {
                    // Check wether this small cave was visited
                    if !path.1.contains(neighbor)
                    {
                        // We add this new path
                        let mut path_new = path.1.clone();

                        path_new.push(String::from(neighbor));
                        path_list.push((String::from(neighbor), path_new));
                    }
                }
            }
        }
    }

    println!("{}", path_count);
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
fn load_data<P>(path: P) -> Vec<String>
where
    P: AsRef<Path>,
{
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
