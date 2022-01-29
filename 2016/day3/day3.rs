// Day 3: Squares With Three Sides
// https://adventofcode.com/2016/day/3

use scan_fmt::scan_fmt;

fn main() {
    let data = std::fs::read_to_string("input.txt").unwrap();

    println!("{}", part1(&data));
    println!("{}", part2(&data));
}

/// part 1
fn part1(data: &str) -> u32 {
    let mut triangles = 0;

    for line in data.split('\n') {
        if let Ok((x, y, z)) = scan_fmt!(line, "{} {} {}", i32, i32, i32) {
            triangles += is_triangle(x, y, z);
        }
    }

    triangles
}

/// ``is_triangle`` returns 1 if the given sides form a triangle, 0 otherwise
fn is_triangle(x: i32, y: i32, z: i32) -> u32 {
    if x + y > z && x + z > y && y + z > x {
        1
    } else {
        0
    }
}

/// part 2
fn part2(data: &str) -> u32 {
    let mut triangles = 0;
    let lines = data.split('\n').collect::<Vec<_>>();

    for i in (0..lines.len()).step_by(3) {
        if let Ok((x1, y1, z1)) = scan_fmt!(lines[i], "{} {} {}", i32, i32, i32) {
            if let Ok((x2, y2, z2)) = scan_fmt!(lines[i + 1], "{} {} {}", i32, i32, i32) {
                if let Ok((x3, y3, z3)) = scan_fmt!(lines[i + 2], "{} {} {}", i32, i32, i32) {
                    triangles += is_triangle(x1, x2, x3);
                    triangles += is_triangle(y1, y2, y3);
                    triangles += is_triangle(z1, z2, z3);
                }
            }
        }
    }

    triangles
}

#[cfg(test)]
#[test]
fn test_part1() {
    assert_eq!(part1("5 10 25"), 0);
    assert_eq!(part1("5 10 10"), 1);
}

#[test]
fn test_part2() {
    let data="101 301 501
102 302 502
103 303 503
201 401 601
202 402 602
203 403 603
";
    assert_eq!(part2(data), 6);
}
