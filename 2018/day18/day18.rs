//! [Day 18: Settlers of The North Pole](https://adventofcode.com/2018/day/18)

use rustc_hash::FxHashMap;

const OPEN_ACRE: u8 = 0;
const TREE: u8 = 1;
const LUMBERYARD: u8 = 2;

type Area = aoc::GridU<u8>;

fn collect(area: &Area) -> Area {
    let mut new_area = Area::with_size(area.size().0, area.size().1);

    for (xy, acre) in area.iter() {
        let (trees, lumberyards) = {
            let mut trees = 0;
            let mut lumberyards = 0;

            for c in area.iter_neighbors(xy) {
                match area[c] {
                    TREE => trees += 1,
                    LUMBERYARD => lumberyards += 1,
                    _ => (),
                }
            }

            (trees, lumberyards)
        };

        let mut acre = *acre;

        match acre {
            OPEN_ACRE => {
                if trees >= 3 {
                    acre = TREE;
                }
            }
            TREE => {
                if lumberyards >= 3 {
                    acre = LUMBERYARD;
                }
            }
            LUMBERYARD => {
                if trees == 0 || lumberyards == 0 {
                    acre = OPEN_ACRE;
                }
            }
            _ => (),
        }

        new_area[xy] = acre;
    }

    new_area
}

fn value(area: &Area) -> u32 {
    let mut trees = 0;
    let mut lumberyards = 0;

    for (_, c) in area.iter() {
        match *c {
            TREE => trees += 1,
            LUMBERYARD => lumberyards += 1,
            _ => (),
        }
    }

    trees * lumberyards
}

fn hashable(area: &Area) -> Vec<u8> {
    let mut s = Vec::new();

    for (_, &c) in area.iter() {
        s.push(c);
    }
    s
}

struct Puzzle {
    area: Area,
    n: usize,
}

impl Puzzle {
    fn new() -> Self {
        Self {
            area: Area::default(),
            n: 0,
        }
    }

    /// Get the puzzle input.
    fn configure(&mut self, data: &str) {
        self.n = 0;
        for (y, line) in data.lines().enumerate() {
            if self.n == 0 {
                self.n = line.len();
                self.area.resize(self.n, self.n);
            }
            for (x, acre) in line.chars().enumerate() {
                self.area[(x, y)] = match acre {
                    '|' => TREE,
                    '#' => LUMBERYARD,
                    _ => OPEN_ACRE,
                };
            }
        }
    }

    /// Solve part one.
    fn part1(&self) -> u32 {
        let mut area = self.area.clone();

        for _ in 0..10 {
            area = collect(&area);
        }

        value(&area)
    }

    /// Solve part two.
    fn part2(&self) -> u32 {
        let mut area = self.area.clone();

        let mut values = vec![];
        let mut seen = FxHashMap::default();

        for i in 0.. {
            values.push(value(&area));

            if seen.contains_key(&hashable(&area)) {
                let cycle_start = seen.get(&hashable(&area)).unwrap();
                let cycle_end = i;

                let n = 1_000_000_000;
                let cycle = cycle_end - cycle_start;
                return values[cycle_start + (n - cycle_end) % cycle as usize];
            }

            seen.insert(hashable(&area), i);

            area = collect(&area);
        }

        0
    }
}

fn main() {
    let args = aoc::parse_args();
    let mut puzzle = Puzzle::new();
    puzzle.configure(&args.input);
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
        puzzle.configure(&aoc::load_input_data("test.txt"));
        assert_eq!(puzzle.part1(), 1147);
    }
}
