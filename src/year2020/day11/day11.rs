//! [Day 11: Seating System](https://adventofcode.com/2020/day/11)

const EMPTY: u8 = b'L';
const OCCUPIED: u8 = b'#';
const FLOOR: u8 = b'.';

const NEIGHBORS: &[(i32, i32)] = &[
    (1, -1),
    (1, 0),
    (1, 1),
    (0, -1),
    (0, 1),
    (-1, -1),
    (-1, 0),
    (-1, 1),
];

struct Puzzle {
    grid: Vec<u8>,
    width: usize,
    height: usize,
}

impl Puzzle {
    fn new(data: &str) -> Self {
        let width = data.lines().next().map_or(0, str::len);
        let grid: Vec<u8> = data.lines().flat_map(str::bytes).collect();
        let height = if width == 0 { 0 } else { grid.len() / width };
        Self {
            grid,
            width,
            height,
        }
    }

    fn build_adjacency(&self, visibility: i32) -> Vec<Vec<usize>> {
        let n = self.grid.len();
        let iw = i32::try_from(self.width).expect("width fits i32");
        let ih = i32::try_from(self.height).expect("height fits i32");

        // grid_to_seat[i] = index dans le tableau de sièges ; usize::MAX si floor
        let mut grid_to_seat = vec![usize::MAX; n];
        let seats: Vec<usize> = (0..n).filter(|&i| self.grid[i] == EMPTY).collect();
        for (si, &gi) in seats.iter().enumerate() {
            grid_to_seat[gi] = si;
        }

        seats
            .iter()
            .map(|&gi| {
                let gx = i32::try_from(gi % self.width).expect("x fits i32");
                let gy = i32::try_from(gi / self.width).expect("y fits i32");
                let mut neighbors = Vec::new();
                for &(dx, dy) in NEIGHBORS {
                    for v in 1..=visibility {
                        let nx = gx + dx * v;
                        let ny = gy + dy * v;
                        if nx < 0 || nx >= iw || ny < 0 || ny >= ih {
                            break;
                        }
                        let ni = usize::try_from(ny).expect("ny >= 0") * self.width
                            + usize::try_from(nx).expect("nx >= 0");
                        if self.grid[ni] != FLOOR {
                            neighbors.push(grid_to_seat[ni]);
                            break;
                        }
                    }
                }
                neighbors
            })
            .collect()
    }

    fn simulate(adjacency: &[Vec<usize>], tolerance: usize) -> usize {
        let mut current = vec![EMPTY; adjacency.len()];
        let mut next = vec![EMPTY; adjacency.len()];

        loop {
            let mut changed = false;
            for (i, neighbors) in adjacency.iter().enumerate() {
                let occ = neighbors
                    .iter()
                    .filter(|&&j| current[j] == OCCUPIED)
                    .count();
                next[i] = if current[i] == EMPTY && occ == 0 {
                    changed = true;
                    OCCUPIED
                } else if current[i] == OCCUPIED && occ >= tolerance {
                    changed = true;
                    EMPTY
                } else {
                    current[i]
                };
            }
            if !changed {
                break;
            }
            std::mem::swap(&mut current, &mut next);
        }

        bytecount::count(&current, OCCUPIED)
    }

    fn part1(&self) -> usize {
        Self::simulate(&self.build_adjacency(1), 4)
    }

    fn part2(&self) -> usize {
        let vis = i32::try_from(self.width.max(self.height)).expect("fits i32");
        Self::simulate(&self.build_adjacency(vis), 5)
    }
}

/// # Panics
#[must_use]
pub fn solve(data: &str) -> (usize, usize) {
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

    const TEST_INPUT: &str = include_str!("test.txt");

    #[test]
    fn test_part1() {
        let puzzle = Puzzle::new(TEST_INPUT);
        assert_eq!(puzzle.part1(), 37);
    }

    #[test]
    fn test_part2() {
        let puzzle = Puzzle::new(TEST_INPUT);
        assert_eq!(puzzle.part2(), 26);
    }
}
