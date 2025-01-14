//! [Day 13: Care Package](https://adventofcode.com/2019/day/13)

use intcode::{Computer, State};

#[derive(Clone)]
struct ArcadeCabinet {
    computer: Computer,
    score: i64,
    paddle: (i64, i64),
    ball: (i64, i64),
}

impl ArcadeCabinet {
    // const TILE_EMPTY: i64 = 0;
    // const TILE_WALL: i64 = 1;
    const TILE_BLOCK: i64 = 2;
    const TILE_PADDLE: i64 = 3;
    const TILE_BALL: i64 = 4;

    fn new(code: &str) -> Self {
        Self {
            computer: Computer::load(code),
            score: 0,
            paddle: (0, 0),
            ball: (0, 0),
        }
    }

    fn run(&mut self) -> (Vec<i64>, bool) {
        let mut output = Vec::new();

        loop {
            match self.computer.run() {
                State::Halted => break (output, true),
                State::Input => break (output, false),
                State::Output(num) => output.push(num),
            }
        }
    }

    fn part1(&mut self) -> usize {
        self.run()
            .0
            .iter()
            .skip(2)
            .step_by(3)
            .filter(|num| **num == Self::TILE_BLOCK)
            .count()
    }

    fn part2(&mut self) -> i64 {
        self.computer.reset();
        self.computer.poke(0, 2); // play for free
        self.play();
        self.score
    }

    fn play(&mut self) {
        while self.frame() {
            self.computer.push(self.joystick());
        }
    }

    fn frame(&mut self) -> bool {
        let (output, halted) = self.run();

        for triplet in output.chunks(3) {
            //
            let x = triplet[0];
            let y = triplet[1];
            let tile = triplet[2];

            if x == -1 && y == 0 {
                self.score = tile;
            } else {
                match tile {
                    Self::TILE_BALL => self.ball = (x, y),
                    Self::TILE_PADDLE => self.paddle = (x, y),
                    _ => (),
                };
            }
        }

        !halted
    }

    const fn joystick(&self) -> i64 {
        if self.paddle.0 < self.ball.0 {
            1
        } else if self.paddle.0 > self.ball.0 {
            -1
        } else {
            0
        }
    }
}

fn main() {
    let args = aoc::parse_args();

    let mut cab = ArcadeCabinet::new(&args.input);

    println!("{}", cab.part1());
    println!("{}", cab.part2());
}
