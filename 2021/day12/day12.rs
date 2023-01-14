//! [Day 12: Passage Pathing](https://adventofcode.com/2021/day/12)

use std::collections::HashMap;

fn compute_paths(data: &[&str], small_twice: bool) -> u32 {
    // Map containing each cave and its neighbors as a list
    let mut map: HashMap<String, Vec<String>> = HashMap::new();

    // Fill the map with caves
    for entry in data {
        // Split line between "left" cave and "right" cave
        let mut split = entry.split('-');
        let left = split.next().unwrap();
        let right = split.next().unwrap();

        // Update map with eventual new caves and neighbors
        let left_cave = map.entry(String::from(left)).or_default();
        left_cave.push(String::from(right));

        let right_cave = map.entry(String::from(right)).or_default();
        right_cave.push(String::from(left));
    }

    // Initialize paths list
    let mut path_list = vec![(String::from("start"), vec![String::from("start")], false)];
    let mut path_count = 0;

    while !path_list.is_empty() {
        let (node, path, twice) = path_list.pop().unwrap();

        if node == "end" {
            // Count this path and let it be removed from the paths list
            path_count += 1;
        } else {
            // Get neighbors from last node
            let neighbor_list = map.get(&node).unwrap();

            for neighbor in neighbor_list {
                if neighbor.to_uppercase() == *neighbor {
                    // We add this new path
                    let mut path_new = path.clone();

                    path_new.push(String::from(neighbor));
                    path_list.push((String::from(neighbor), path_new, twice));
                } else {
                    // Check wether this small cave was visited
                    if !path.contains(neighbor) {
                        // We add this new path
                        let mut path_new = path.clone();

                        path_new.push(String::from(neighbor));
                        path_list.push((String::from(neighbor), path_new, twice));
                    }
                    // Check wether we already visited twice a small cave
                    else if small_twice && !twice && neighbor != "start" && neighbor != "end" {
                        // We add this new path
                        let mut path_new = path.clone();

                        path_new.push(String::from(neighbor));
                        path_list.push((String::from(neighbor), path_new, true));
                    }
                }
            }
        }
    }

    path_count
}

/// main function
fn main() {
    let filename = if let Some(x) = std::env::args().collect::<Vec<String>>().get(1) {
        x.clone()
    } else {
        "input.txt".to_string()
    };

    let data = std::fs::read_to_string(filename).unwrap();
    let data = data.lines().collect::<Vec<_>>();

    let small_once = compute_paths(&data, false);
    let small_twice = compute_paths(&data, true);

    println!("{}", small_once);
    println!("{}", small_twice);
}

#[test]
fn test_slightly_larger() {
    let data = "\
dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc
"
    .lines()
    .collect::<Vec<_>>();

    assert_eq!(compute_paths(&data, false), 19);
    assert_eq!(compute_paths(&data, true), 103);
}

#[test]
fn test_even_larger() {
    let data = "\
fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW
"
    .lines()
    .collect::<Vec<_>>();

    assert_eq!(compute_paths(&data, false), 226);
    assert_eq!(compute_paths(&data, true), 3509);
}
