//! [Day 16: The Floor Will Be Lava](https://adventofcode.com/2023/day/16)

struct Puzzle {
    mirrors: Vec<Vec<char>>,
    beams: Vec<Vec<u8>>,
    sx: usize,
    sy: usize,
}

impl Puzzle {
    fn new() -> Puzzle {
        Puzzle {
            mirrors: vec![],
            beams: vec![],
            sx: 0,
            sy: 0,
        }
    }

    /// Get the puzzle input.
    fn configure(&mut self, path: &str) {
        let data = std::fs::read_to_string(path).unwrap();

        self.mirrors.push(vec![]);
        for line in data.lines() {
            let mut row = vec![];
            row.push(' ');
            for c in line.chars() {
                row.push(c);
            }
            self.mirrors.push(row);
        }
        self.sx = self.mirrors[1].len() - 1;
        self.sy = self.mirrors.len() - 1;
    }

    /// Set beams state to initial value.
    fn reset_beams(&mut self) {
        self.beams.clear();
        for _y in 0..=self.sy {
            let row = vec![0; self.sx + 1];
            self.beams.push(row);
        }
    }

    /// Compute recursively the path of beam.
    fn beam(&mut self, x: usize, y: usize, d: char) {
        // point inside the grid ?
        if x == 0 || y == 0 || x > self.sx || y > self.sy {
            return;
        }

        // if point has already traversed by the beam, stop
        let id = 1 << u8::try_from("^v<>".find(d).unwrap()).unwrap();
        if (self.beams[y][x] & id) == id {
            return;
        }
        self.beams[y][x] |= id;

        match self.mirrors[y].get(x) {
            Some('.') => match d {
                '>' => self.beam(x + 1, y, d), // traverse
                '<' => self.beam(x - 1, y, d), // traverse
                'v' => self.beam(x, y + 1, d), // traverse
                '^' => self.beam(x, y - 1, d), // traverse
                _ => panic!(),
            },

            Some('\\') => match d {
                '>' => self.beam(x, y + 1, 'v'), // reflect
                '<' => self.beam(x, y - 1, '^'), // reflect
                'v' => self.beam(x + 1, y, '>'), // reflect
                '^' => self.beam(x - 1, y, '<'), // reflect
                _ => panic!(),
            },

            Some('/') => match d {
                '>' => self.beam(x, y - 1, '^'), // reflect
                '<' => self.beam(x, y + 1, 'v'), // reflect
                'v' => self.beam(x - 1, y, '<'), // reflect
                '^' => self.beam(x + 1, y, '>'), // reflect
                _ => panic!(),
            },

            Some('-') => match d {
                '>' => self.beam(x + 1, y, d), // traverse
                '<' => self.beam(x - 1, y, d), // traverse
                'v' | '^' => {
                    self.beam(x - 1, y, '<'); // split the beam
                    self.beam(x + 1, y, '>'); // split the beam
                }
                _ => panic!(),
            },

            Some('|') => match d {
                'v' => self.beam(x, y + 1, d), // traverse
                '^' => self.beam(x, y - 1, d), // traverse
                '>' | '<' => {
                    self.beam(x, y - 1, '^'); // split the beam
                    self.beam(x, y + 1, 'v'); // split the beam
                }
                _ => panic!(),
            },

            _ => panic!(),
        }
    }

    /// Get the number of energized cells.
    fn energized(&self) -> usize {
        self.beams
            .iter()
            .map(|row| row.iter().filter(|&&e| e != 0).count())
            .sum()
    }

    /// Turn on the beam light and get the number of energized cells.
    fn calc(&mut self, x: usize, y: usize, d: char) -> usize {
        self.reset_beams();
        self.beam(x, y, d);
        self.energized()
    }

    /// Solve part one.
    fn part1(&mut self) -> usize {
        self.calc(1, 1, '>')
    }

    /// Solve part two.
    fn part2(&mut self) -> usize {
        let mut m = 0;

        m = m.max(
            (1..=self.sx)
                .map(|x| self.calc(x, 1, 'v').max(self.calc(x, self.sy, '^')))
                .max()
                .unwrap(),
        );

        m = m.max(
            (1..=self.sy)
                .map(|y| self.calc(1, y, '>').max(self.calc(self.sx, y, '<')))
                .max()
                .unwrap(),
        );

        m
    }
}

fn main() {
    let args = aoc::parse_args();
    let mut puzzle = Puzzle::new();
    puzzle.configure(args.path.as_str());
    println!("{}", puzzle.part1());
    println!("{}", puzzle.part2());
}

/// Test from puzzle input
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test01() {
        let mut puzzle = Puzzle::new();
        puzzle.configure("test.txt");
        assert_eq!(puzzle.part1(), 46);
    }

    #[test]
    fn test02() {
        let mut puzzle = Puzzle::new();
        puzzle.configure("test.txt");
        assert_eq!(puzzle.part2(), 51);
    }
}
