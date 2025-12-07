//! [Day 6: Trash Compactor](https://adventofcode.com/2025/day/6)

/// # Panics
#[must_use]
pub fn solve(data: &str) -> (u64, u64) {
    // parse input data
    let mut rows = vec![];
    let mut operators = "".as_bytes();

    for line in data.lines() {
        if line.contains('+') || line.contains('*') {
            operators = line.as_bytes();
        } else {
            rows.push(line.as_bytes());
        }
    }

    let mut part1 = 0;
    let mut part2 = 0;

    // iterate over operator positions
    for (pos, op) in operators.iter().enumerate().filter(|&(_, op)| op != &b' ') {
        // find the column width
        let mut end = pos + 1;
        while end < operators.len() && operators[end] == b' ' {
            end += 1;
        }
        if end != operators.len() {
            end -= 1;
        }

        // compute part 1
        let mut htotal = u64::from(op != &b'+');
        for row in &rows {
            let mut hnum = 0;

            for i in pos..end {
                let ch = row[i];
                if ch != b' ' {
                    hnum = hnum * 10 + u64::from(ch - b'0');
                }
            }

            if op == &b'+' {
                htotal += hnum;
            } else {
                htotal *= hnum;
            };
        }
        part1 += htotal;

        // compute part 2
        let mut vtotal = u64::from(op != &b'+');
        for i in pos..end {
            let mut vnum = 0;
            for row in &rows {
                let ch = row[i];
                if ch != b' ' {
                    vnum = vnum * 10 + u64::from(ch - b'0');
                }
            }
            if op == &b'+' {
                vtotal += vnum;
            } else {
                vtotal *= vnum;
            };
        }
        part2 += vtotal;
    }

    (part1, part2)
}

pub fn main() {
    let args = aoc::parse_args();
    args.run(solve);
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = include_str!("test.txt");

    #[test]
    fn parts() {
        let (part1, part2) = solve(TEST_INPUT);
        assert_eq!(part1, 4277556);
        assert_eq!(part2, 3263827);
    }
}
