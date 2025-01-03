//! [Day 10: The Stars Align](https://adventofcode.com/2018/day/10)

#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_sign_loss)]

use aoc::ocr::scan_6x10;

struct Puzzle {
    pos: Vec<(i32, i32)>,
    vel: Vec<(i32, i32)>,

    message: String,
    seconds: u32,
}

impl Puzzle {
    const fn new() -> Self {
        Self {
            pos: vec![],
            vel: vec![],
            message: String::new(),
            seconds: 0,
        }
    }

    /// Get the puzzle input.
    fn configure(&mut self, data: &str) {
        for line in data.lines() {
            let row: Vec<_> = line.split(['<', '>', ',']).collect();

            self.pos.push((
                row[1].trim().parse().unwrap(),
                row[2].trim().parse().unwrap(),
            ));

            self.vel.push((
                row[4].trim().parse().unwrap(),
                row[5].trim().parse().unwrap(),
            ));
        }
    }

    fn solve(&mut self, verbose: bool) {
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

        let width = (xmax - xmin + 1) as usize;
        let height = (ymax - ymin + 1) as usize;

        let mut lcd = vec![vec!['.'; width]; height];
        for p in &pos {
            let x = (p.0 - xmin) as usize;
            let y = (p.1 - ymin) as usize;
            lcd[y][x] = '#';
        }

        let lcd = lcd
            .iter()
            .map(|row| row.iter().collect::<String>())
            .collect::<Vec<String>>()
            .join("\n");

        if verbose {
            println!("{lcd}");
        }

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

fn main() {
    let args = aoc::parse_args();
    let mut puzzle = Puzzle::new();
    puzzle.configure(&args.input);
    puzzle.solve(args.verbose);
    println!("{}", puzzle.part1());
    println!("{}", puzzle.part2());
}
