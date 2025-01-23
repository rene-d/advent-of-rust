//! [Day 12: JSAbacusFramework.io](https://adventofcode.com/2015/day/12)

use regex::Regex;
use serde_json::Value;

/// main function
pub fn main() {
    let args = aoc::parse_args();
    args.run(solve);
}

/// # Panics
/// over malformed input
#[must_use]
pub fn solve(data: &str) -> (i32, i32) {
    (part1(data), part2(data))
}

fn part1(data: &str) -> i32 {
    let re = Regex::new(r"(\-?\d+)").unwrap();
    data.lines()
        .map(|line| {
            re.find_iter(line)
                .map(|m| m.as_str().parse::<i32>().unwrap())
                .sum::<i32>()
        })
        .sum::<i32>()
}

fn sum(v: &Value) -> i32 {
    match v {
        Value::Number(n) => n.as_i64().unwrap().try_into().unwrap(),
        Value::Array(a) => a.iter().map(sum).sum(),
        Value::Object(o) => {
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

fn part2(data: &str) -> i32 {
    let json: Value = serde_json::from_str(data).expect("JSON was not well-formatted");
    sum(&json)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(part1("[1,2,3]"), 6);
        assert_eq!(part1(r#"{"a":2,"b":4}"#), 6);

        assert_eq!(part1("[[[3]]]"), 3);
        assert_eq!(part1(r#"{"a":{"b":4},"c":-1}"#), 3);

        assert_eq!(part1(r#"{"a":[-1,1]}"#), 0);
        assert_eq!(part1(r#"[-1,{"a":1}]"#), 0);

        assert_eq!(part1("[]"), 0);
        assert_eq!(part1("{}"), 0);
    }

    #[test]
    fn test2() {
        assert_eq!(part2("[1,2,3]"), 6);
        assert_eq!(part2(r#"[1,{"c":"red","b":2},3]"#), 4);
        assert_eq!(part2(r#"{"d":"red","e":[1,2,3,4],"f":5}"#), 0);
        assert_eq!(part2(r#"[1,"red",5]"#), 6);
    }
}
