//! [Day 15: Beverage Bandits](https://adventofcode.com/2018/day/15)

use std::collections::{HashMap, HashSet};
use std::{cmp::Ordering, collections::VecDeque};

type Grid = aoc::grid::Grid<char>;

const ELF: char = 'E';
const GOBLIN: char = 'G';
const WALL: char = '#';

#[derive(Clone)]
struct Unit {
    x: usize,
    y: usize,
    hit_points: u32,
    attack_power: u32,
    race: char,
}

impl Unit {
    fn attack_order(&self, other: &Self) -> Ordering {
        self.y.cmp(&other.y).then(self.x.cmp(&other.x))
    }

    fn min_hp_order(&self, other: &Self) -> Ordering {
        self.hit_points
            .cmp(&other.hit_points)
            .then(Self::attack_order(self, other))
    }
}
struct Puzzle {
    wall: HashSet<(usize, usize)>,
    units: Vec<Unit>,
}

fn adjacent(x: usize, y: usize) -> Vec<(usize, usize)> {
    vec![(x, y - 1), (x - 1, y), (x + 1, y), (x, y + 1)]
}

fn get_opponents(targets: &[Unit], race: char) -> HashMap<(usize, usize), Vec<usize>> {
    let mut target_mapping: HashMap<(usize, usize), Vec<usize>> = HashMap::new();

    for (i, t) in targets.iter().enumerate() {
        if t.race != race && t.hit_points != 0 {
            for adj in adjacent(t.x, t.y) {
                target_mapping.entry(adj).or_default().push(i);
            }
        }
    }

    target_mapping
}

fn next_pos(
    u: usize,
    units: &[Unit],
    target_adj: &HashSet<(usize, usize)>,
    wall: &HashSet<(usize, usize)>,
) -> Option<(usize, usize)> {
    let others: HashSet<(usize, usize)> = units
        .iter()
        .enumerate()
        .filter_map(|(i, unit)| {
            if i != u && unit.hit_points != 0 {
                Some((unit.x, unit.y))
            } else {
                None
            }
        })
        .collect();

    let mut pos = vec![];

    let mut q = VecDeque::new();
    let mut visited = HashSet::new();

    q.extend(
        adjacent(units[u].x, units[u].y)
            .iter()
            .map(|(x, y)| (*x, *y, 1, (*x, *y))),
    );

    visited.insert((units[u].x, units[u].y));

    let mut min_path = u32::MAX;

    while let Some(e) = q.pop_front() {
        //
        let (x, y, steps, start) = e;

        if min_path != u32::MAX && steps > min_path {
            continue;
        }

        if wall.contains(&(x, y)) || others.contains(&(x, y)) {
            continue;
        }

        if target_adj.contains(&(x, y)) {
            min_path = min_path.min(steps);
            pos.push(e);
        } else if !visited.contains(&(x, y)) {
            visited.insert((x, y));

            q.extend(
                adjacent(x, y)
                    .iter()
                    .map(|(nx, ny)| (*nx, *ny, steps + 1, start)),
            );
        }
    }

    pos.iter()
        .min_by_key(|(x, y, steps, (sx, sy))| (steps, y, x, sy, sx))
        .map(|(_, _, _, start)| *start)
}

fn has_elves_and_goblins(units: &[Unit]) -> bool {
    let mut goblins = false;
    let mut elves = false;

    for unit in units {
        if unit.hit_points != 0 {
            match unit.race {
                GOBLIN => goblins = true,
                ELF => elves = true,
                _ => (),
            }

            if goblins && elves {
                return true;
            }
        }
    }

    false
}

impl Puzzle {
    fn new() -> Puzzle {
        Puzzle {
            wall: HashSet::new(),
            units: vec![],
        }
    }

    /// Get the puzzle input.
    fn configure(&mut self, path: &str) {
        let data = std::fs::read_to_string(path).unwrap();

        let grid = Grid::parse(&data);

        for (xy, u) in grid.iter() {
            match u {
                &GOBLIN | &ELF => {
                    self.units.push(Unit {
                        x: xy.0,
                        y: xy.1,
                        hit_points: 200,
                        attack_power: 3,
                        race: *u,
                    });
                }
                &WALL => {
                    self.wall.insert(xy);
                }
                _ => (),
            }
        }
    }

    fn fight(&self, elf_attack_power: u32, elves_must_win: bool) -> Option<u32> {
        let wall = &self.wall;
        let mut units = self.units.clone();

        for unit in &mut units {
            if unit.race == ELF {
                unit.attack_power = elf_attack_power;
            }
        }

        let mut round = 0;

        loop {
            units.retain(|unit| unit.hit_points != 0);

            units.sort_unstable_by(Unit::attack_order);

            for u in 0..units.len() {
                if units[u].hit_points == 0 {
                    continue;
                }

                let opponents = get_opponents(&units, units[u].race);

                if !opponents.contains_key(&(units[u].x, units[u].y)) {
                    // not in range

                    let target_adj: HashSet<(usize, usize)> = opponents.keys().copied().collect();

                    if let Some(xy) = next_pos(u, &units, &target_adj, wall) {
                        units[u].x = xy.0;
                        units[u].y = xy.1;
                    }
                }

                if opponents.contains_key(&(units[u].x, units[u].y)) {
                    // attack
                    let damage = units[u].attack_power;

                    let target_indices = opponents.get(&(units[u].x, units[u].y)).unwrap();

                    let &target_index = target_indices
                        .iter()
                        .min_by(|&&a, &&b| Unit::min_hp_order(&units[a], &units[b]))
                        .unwrap();

                    let target = &mut units[target_index];

                    target.hit_points = target.hit_points.saturating_sub(damage);

                    if elves_must_win && target.hit_points == 0 && target.race == ELF {
                        return None;
                    }

                    if target.hit_points == 0 && !has_elves_and_goblins(&units) {
                        if u == units.len() - 1 {
                            round += 1;
                        }

                        let sum_of_hp: u32 = units.iter().map(|unit| unit.hit_points).sum();

                        return Some(round * sum_of_hp);
                    }
                }
            }

            round += 1;
        }
    }

    /// Solve part one.
    fn part1(&self) -> u32 {
        self.fight(3, false).unwrap()
    }

    /// Solve part two.
    fn part2(&self) -> u32 {
        for elf_attack_power in 4..100 {
            if let Some(outcome) = self.fight(elf_attack_power, true) {
                return outcome;
            }
        }
        0
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
    fn test_6_1() {
        let mut puzzle = Puzzle::new();
        puzzle.configure("sample_6.txt");
        assert_eq!(puzzle.part1(), 27730);
    }

    #[test]
    fn test_6_2() {
        let mut puzzle = Puzzle::new();
        puzzle.configure("sample_6.txt");
        assert_eq!(puzzle.part2(), 4988);
    }

    #[test]
    fn test_7_1() {
        let mut puzzle = Puzzle::new();
        puzzle.configure("sample_7.txt");
        assert_eq!(puzzle.part1(), 36334);
    }

    // #[test]
    // fn test_7_2() {
    //     let mut puzzle = Puzzle::new();
    //     puzzle.configure("sample_7.txt");
    //     assert_eq!(puzzle.part2(), 0);
    // }

    #[test]
    fn test_8_1() {
        let mut puzzle = Puzzle::new();
        puzzle.configure("sample_8.txt");
        assert_eq!(puzzle.part1(), 39514);
    }

    #[test]
    fn test_8_2() {
        let mut puzzle = Puzzle::new();
        puzzle.configure("sample_8.txt");
        assert_eq!(puzzle.part2(), 31284);
    }

    #[test]
    fn test_9_1() {
        let mut puzzle = Puzzle::new();
        puzzle.configure("sample_9.txt");
        assert_eq!(puzzle.part1(), 27755);
    }

    #[test]
    fn test_9_2() {
        let mut puzzle = Puzzle::new();
        puzzle.configure("sample_9.txt");
        assert_eq!(puzzle.part2(), 3478);
    }

    #[test]
    fn test_10_1() {
        let mut puzzle = Puzzle::new();
        puzzle.configure("sample_10.txt");
        assert_eq!(puzzle.part1(), 28944);
    }

    #[test]
    fn test_10_2() {
        let mut puzzle = Puzzle::new();
        puzzle.configure("sample_10.txt");
        assert_eq!(puzzle.part2(), 6474);
    }

    #[test]
    fn test_11_1() {
        let mut puzzle = Puzzle::new();
        puzzle.configure("sample_11.txt");
        assert_eq!(puzzle.part1(), 18740);
    }

    #[test]
    fn test_11_2() {
        let mut puzzle = Puzzle::new();
        puzzle.configure("sample_11.txt");
        assert_eq!(puzzle.part2(), 1140);
    }
}
