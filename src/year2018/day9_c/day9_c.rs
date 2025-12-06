//! [Day 9: Marble Mania](https://adventofcode.com/2018/day/9)

#[must_use]
pub fn c_solve(elves: u32, points: u32) -> u32 {
    unsafe extern "C" {
        fn c_solve(elves: u32, points: u32) -> u32;
    }

    unsafe { c_solve(elves, points) }
}

fn play(nb_players: u32, nb_marbles: u32) -> u32 {
    let mut marbles = vec![0u32];
    let mut scores = vec![0u32; nb_players as usize];

    let mut current = 1;

    for i in 1..=nb_marbles {
        if i % 23 == 0 {
            current = (current + marbles.len() - 7) % marbles.len();
            scores[(i % nb_players) as usize] += i + marbles[current];
            marbles.remove(current);
        } else {
            current = (current + 2) % marbles.len();
            marbles.insert(current, i);
        }
    }
    *scores.iter().max().unwrap()
}

struct Puzzle {
    elves: u32,
    points: u32,
}

impl Puzzle {
    fn new(data: &str) -> Self {
        let row = data.split_ascii_whitespace().collect::<Vec<_>>();
        match &row[..] {
            [n, _, _, _, _, _, p, _] => Self {
                elves: n.parse().unwrap(),
                points: p.parse().unwrap(),
            },
            _ => panic!("bad input: {data:?}"),
        }
    }

    /// Solve part one.
    fn part1(&self) -> u32 {
        play(self.elves, self.points)
    }

    /// Solve part two.
    fn part2(&self) -> u32 {
        c_solve(self.elves, self.points * 100)
    }
}

/// # Panics
#[must_use]
pub fn solve(data: &str) -> (u32, u32) {
    let puzzle = Puzzle::new(data);
    (puzzle.part1(), puzzle.part2())
}

pub fn main() {
    let args = aoc::parse_args();
    args.run(solve);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test01() {
        let puzzle = Puzzle::new("9 x x x x x 25 x");
        assert_eq!(puzzle.part1(), 32);
    }

    #[test]
    fn test02() {
        assert_eq!(play(10, 1618), 8317);
        assert_eq!(play(13, 7999), 146_373);
        assert_eq!(play(21, 6111), 54718);
        assert_eq!(play(30, 5807), 37305);
    }

    #[test]
    fn test03() {
        assert_eq!(c_solve(10, 1618), 8317);
        assert_eq!(c_solve(13, 7999), 146_373);
        assert_eq!(c_solve(21, 6111), 54718);
        assert_eq!(c_solve(30, 5807), 37305);
    }
}
