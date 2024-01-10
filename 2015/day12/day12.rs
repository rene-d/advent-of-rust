//! [Day 12: JSAbacusFramework.io](https://adventofcode.com/2015/day/12)

use regex::Regex;

fn sum(v: &serde_json::Value) -> i32 {
    match v {
        serde_json::Value::Number(n) => n.as_i64().unwrap().try_into().unwrap(),
        serde_json::Value::Array(a) => a.iter().map(sum).sum(),
        serde_json::Value::Object(o) => {
            // Ignore any object (and all of its children) which has any property with the value "red".
            for v in o.values() {
                if v.as_str() == Some("red") {
                    return 0;
                }
            }
            o.values().map(sum).sum()
        }
        _ => 0,
    }
}

/// main function
fn main() {
    let data = aoc::load_input_data_vec(12);

    // part 1
    let re = Regex::new(r"(\-?\d+)").unwrap();
    let part1 = &data
        .iter()
        .map(|line| {
            re.find_iter(line)
                .map(|m| m.as_str().parse::<i32>().unwrap())
                .sum::<i32>()
        })
        .sum::<i32>();
    println!("{part1}");

    // part 2
    let json: serde_json::Value =
        serde_json::from_str(&data[0]).expect("JSON was not well-formatted");
    println!("{}", sum(&json));
}
