//! [Day 3: Perfectly Spherical Houses in a Vacuum](https://adventofcode.com/2015/day/3)

/// main function
fn main() {
    let args = aoc::parse_args();
    let data = args
        .input
        .lines()
        .map(std::string::ToString::to_string)
        .collect::<Vec<String>>();

    println!("{}", part1(&data[0]));
    println!("{}", part2(&data[0]));
}

fn part2(line: &str) -> usize {
    let mut visited = std::collections::HashSet::new();

    let mut position_santa = (0, 0);
    let mut position_robot = (0, 0);

    visited.insert(position_santa);

    for (i, dir) in line.chars().enumerate() {
        if i % 2 == 0 {
            match dir {
                '<' => position_santa.0 -= 1,
                '>' => position_santa.0 += 1,
                '^' => position_santa.1 -= 1,
                'v' => position_santa.1 += 1,
                _ => panic!("invalid direction: {dir}"),
            }

            visited.insert(position_santa);
        } else {
            match dir {
                '<' => position_robot.0 -= 1,
                '>' => position_robot.0 += 1,
                '^' => position_robot.1 -= 1,
                'v' => position_robot.1 += 1,
                _ => panic!("invalid direction: {dir}"),
            }

            visited.insert(position_robot);
        }
    }

    visited.len()
}

fn part1(line: &str) -> usize {
    let mut visited = std::collections::HashSet::new();

    let mut position = (0, 0);

    visited.insert(position);

    for dir in line.chars() {
        match dir {
            '<' => position.0 -= 1,
            '>' => position.0 += 1,
            '^' => position.1 -= 1,
            'v' => position.1 += 1,
            _ => panic!("invalid direction: {dir}"),
        }

        visited.insert(position);
    }

    visited.len()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(part1(">"), 2);
        assert_eq!(part1("^>v<"), 4);
        assert_eq!(part1("^v^v^v^v^v"), 2);
    }

    #[test]
    fn test2() {
        assert_eq!(part2("^v"), 3);
        assert_eq!(part2("^>v<"), 3);
        assert_eq!(part2("^v^v^v^v^v"), 11);
    }
}
