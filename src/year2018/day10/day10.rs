//! [Day 10: The Stars Align](https://adventofcode.com/2018/day/10)

use aoc::ocr::scan_6x10;

struct Puzzle {
    pos: Vec<(i32, i32)>,
    vel: Vec<(i32, i32)>,

    message: String,
    seconds: u32,
}

impl Puzzle {
    fn new(data: &str) -> Self {
        let mut pos = vec![];
        let mut vel = vec![];

        for line in data.lines() {
            let row: Vec<_> = line.split(['<', '>', ',']).collect();

            pos.push((
                row[1].trim().parse().unwrap(),
                row[2].trim().parse().unwrap(),
            ));

            vel.push((
                row[4].trim().parse().unwrap(),
                row[5].trim().parse().unwrap(),
            ));
        }

        Self {
            pos,
            vel,
            message: String::new(),
            seconds: 0,
        }
    }

    fn solve(&mut self) {
        let mut pos = self.pos.clone();

        let mut prev_height = i32::MAX;
        for seconds in 0.. {
            let mut ymin = i32::MAX;
            let mut ymax = i32::MIN;

            for (i, (vx, vy)) in self.vel.iter().enumerate() {
                pos[i].0 += vx;
                pos[i].1 += vy;

                ymin = ymin.min(pos[i].1);
                ymax = ymax.max(pos[i].1);
            }

            let height = ymax - ymin + 1;

            if prev_height < height {
                // height increases : time to stop
                // rollback last move
                for (i, (vx, vy)) in self.vel.iter().enumerate() {
                    pos[i].0 -= vx;
                    pos[i].1 -= vy;
                }
                self.seconds = seconds;
                break;
            }

            prev_height = height;
        }

        let xmin = pos.iter().map(|p| p.0).min().unwrap();
        let xmax = pos.iter().map(|p| p.0).max().unwrap();

        let ymin = pos.iter().map(|p| p.1).min().unwrap();
        let ymax = pos.iter().map(|p| p.1).max().unwrap();

        let width = usize::try_from(xmax - xmin + 1).unwrap();
        let height = usize::try_from(ymax - ymin + 1).unwrap();

        let mut lcd = vec![vec!['.'; width]; height];
        for p in &pos {
            let x = usize::try_from(p.0 - xmin).unwrap();
            let y = usize::try_from(p.1 - ymin).unwrap();
            lcd[y][x] = '#';
        }

        let lcd = lcd
            .iter()
            .map(|row| row.iter().collect::<String>())
            .collect::<Vec<String>>()
            .join("\n");

        if height == 10 {
            self.message = scan_6x10(&lcd);
        }
        if self.message.len() != 8 {
            println!("{lcd}");
            println!("decoded: {}", self.message);
            std::process::exit(2);
        }
    }

    /// Solve part one.
    fn part1(&self) -> String {
        self.message.clone()
    }

    /// Solve part two.
    const fn part2(&self) -> u32 {
        self.seconds
    }
}

/// # Panics
#[must_use]
pub fn solve(data: &str) -> (String, u32) {
    let mut puzzle = Puzzle::new(data);
    puzzle.solve();
    (puzzle.part1(), puzzle.part2())
}

pub fn main() {
    let args = aoc::parse_args();
    args.run(solve);
}
