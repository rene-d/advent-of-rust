
// Day 2: Bathroom Security
// https://adventofcode.com/2016/day/2

// day2.rs

fn main() {
    let data = std::fs::read_to_string("input.txt").unwrap();

    println!("{}", part1(&data));
    println!("{}", part2(&data));
}

/// part 1
fn part1(data: &str) -> String {
    let mut bathroom_code = String::new();

    let mut x = 1u8; // position on the 3x3 grid
    let mut y = 1u8; // starting at the middle of the grid

    for line in data.split('\n') {
        for c in line.chars() {
            match c {
                'U' => y = y.saturating_sub(1),
                'D' => {
                    if y < 2 {
                        y += 1;
                    }
                }
                'L' => x = x.saturating_sub(1),
                'R' => {
                    if x < 2 {
                        x += 1;
                    }
                }
                _ => panic!("unknown direction: {}", c),
            }
        }

        let key = (x + y * 3 + b'1') as char;

        bathroom_code.push(key);
    }

    bathroom_code
}

/// part 2
fn part2(data: &str) -> String {
    let mut bathroom_code = String::new();
    let mut pos = '5';

    for line in data.split('\n') {
        for c in line.chars() {
            pos = match pos {
                '1' => match c {
                    'D' => '3',
                    _ => pos,
                },
                '2' => match c {
                    'D' => '6',
                    'R' => '3',
                    _ => pos,
                },
                '3' => match c {
                    'U' => '1',
                    'D' => '7',
                    'L' => '2',
                    'R' => '4',
                    _ => pos,
                },
                '4' => match c {
                    'D' => '8',
                    'L' => '3',
                    _ => pos,
                },
                '5' => match c {
                    'R' => '6',
                    _ => pos,
                },
                '6' => match c {
                    'U' => '2',
                    'D' => 'A',
                    'L' => '5',
                    'R' => '7',
                    _ => pos,
                },
                '7' => match c {
                    'U' => '3',
                    'D' => 'B',
                    'L' => '6',
                    'R' => '8',
                    _ => pos,
                },
                '8' => match c {
                    'U' => '4',
                    'D' => 'C',
                    'L' => '7',
                    'R' => '9',
                    _ => pos,
                },
                '9' => match c {
                    'L' => '8',
                    _ => pos,
                },
                'A' => match c {
                    'U' => '6',
                    'R' => 'B',
                    _ => pos,
                },
                'B' => match c {
                    'U' => '7',
                    'D' => 'D',
                    'L' => 'A',
                    'R' => 'C',
                    _ => pos,
                },
                'C' => match c {
                    'U' => '8',
                    'L' => 'B',
                    _ => pos,
                },
                'D' => match c {
                    'U' => 'B',
                    _ => pos,
                },
                _ => panic!("unknown position: {}", pos),
            }
        }
        bathroom_code.push(pos);
    }
    bathroom_code
}

#[cfg(test)]
#[test]
fn test_part1() {
    let data = "ULL
RRDDD
LURDL
UUUUD";

    assert_eq!(part1(data), "1985");
}

#[test]
fn test_part2() {
    let data = "ULL
RRDDD
LURDL
UUUUD";

    assert_eq!(part2(data), "5DB3");
}
