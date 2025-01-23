//! [Day 20: Jurassic Jigsaw](https://adventofcode.com/2020/day/20)

use aoc::math::UnsignedMathOps;
use aoc::{Counter, GridU, Square};
use rustc_hash::FxHashMap;

const NESSIE: &str = concat!(
    "                  # \n",
    "#    ##    ##    ###\n",
    " #  #  #  #  #  #   \n"
);

/// The tile signature is the binary value of its top row. It has to be unique (and hopefully it is).
/// Since the tils are 10 pixel wide, a u16 is sufficient.
type TileSig = u16;

type Tile = Square<u8>;

struct TileArrangement(Vec<(TileSig, Tile)>);

fn calc_sig(p: &Tile) -> TileSig {
    let first_row = p.iter_rows().next().unwrap();

    first_row
        .iter()
        .fold(0, |acc, pixel| (acc << 1) | TileSig::from(pixel == &b'#'))
}

/// Reverse the binary signature.
fn symmetric_sig(mut b: TileSig) -> TileSig {
    let mut r = 0;
    for _ in 0..10 {
        r *= 2;
        if b & 1 == 1 {
            r += 1;
        }
        b /= 2;
    }
    r
}

struct LochNess {
    monster_map: Square<u8>,
}

impl LochNess {
    fn new(n: usize) -> Self {
        Self {
            monster_map: Square::new(n * 8),
        }
    }

    /// Reconstruct the entire image with the fixed image tile
    /// at the its position
    fn add_fixed_tile(&mut self, tile: &TileArrangement, sig: TileSig, x: usize, y: usize) {
        //
        for (s, v) in &tile.0 {
            if *s == sig {
                for i in 0..8 {
                    for j in 0..8 {
                        self.monster_map[(x * 8 + i, y * 8 + j)] = v[(i + 1, j + 1)];
                    }
                }
                return;
            }
        }
        panic!()
    }

    /// Return the occurence of monster in the image.
    fn find_monster(&self, monster: &GridU<u8>) -> usize {
        let n = self.monster_map.size();
        let (width, height) = monster.size();

        let mut count = 0;

        for my in 0..(n - height) {
            for mx in 0..(n - width) {
                count += usize::from(
                    monster
                        .iter()
                        .all(|((x, y), &m)| m != b'#' || m == self.monster_map[(mx + x, my + y)]),
                );
            }
        }

        count
    }
}

struct Puzzle {
    tiles: FxHashMap<u32, TileArrangement>,
}

impl Puzzle {
    /// Find the `tile_id` for the four corners of the map.
    fn corners(&self) -> (Vec<u32>, Counter<TileSig>) {
        // counts the signature of each tile in any position
        // counts[]==1 <=> tile is on map boundaries.
        // if count of 4, it's a corner

        let mut counts = Counter::<_>::new();
        for tile in self.tiles.values() {
            for (sig, _) in &tile.0 {
                counts.insert(*sig);
            }
        }

        let corners = self
            .tiles
            .iter()
            .filter_map(|(&tile_id, tile)| {
                let sig_count = tile
                    .0
                    .iter()
                    .filter_map(|(sig, _)| (counts[sig] == 1).then_some(1))
                    .count();

                (sig_count == 4).then_some(tile_id)
            })
            .collect();

        (corners, counts)
    }

    /// Get the opposite edge of a tile relative to a sig.
    fn opposite_edge(&self, id: u32, sig: TileSig) -> TileSig {
        let t = &self.tiles[&id];
        let sig = symmetric_sig(sig);
        for (i, (b, _)) in t.0.iter().enumerate() {
            if sig == *b {
                let opposite_idx = ((i % 4) + 2) % 4 + 4 * (i / 4);
                return t.0[opposite_idx].0;
            }
        }
        panic!()
    }

    /// Get the left edge of a tile relative to a sig.
    fn adjacent_edge(&self, id: u32, sig: TileSig) -> TileSig {
        let t = &self.tiles[&id];
        for (i, (b, _)) in t.0.iter().enumerate() {
            if sig == *b {
                let adj_idx = ((i % 4) + 3) % 4 + 4 * (i / 4);
                return t.0[adj_idx].0;
            }
        }
        panic!()
    }

    /// Find the next tile (same sig, but different id).
    fn next_tile(&self, prev_id: u32, sig: TileSig) -> Option<u32> {
        for (id, v) in &self.tiles {
            if id != &prev_id {
                for (square_sig, _) in &v.0 {
                    if square_sig == &sig {
                        return Some(*id);
                    }
                }
            }
        }
        None
    }
}

impl Puzzle {
    /// Initialize from the puzzle input.
    fn new(data: &str) -> Self {
        let mut tiles = FxHashMap::default();

        for tile_def in data.trim_ascii().split("\n\n") {
            let (tile_id, grid) = tile_def.split_once('\n').unwrap();

            // "Tile XXX:"
            let tile_id = tile_id
                .strip_prefix("Tile ")
                .unwrap()
                .trim_end_matches(':')
                .parse()
                .unwrap();

            let square = Square::parse(grid, '\n');

            let mut tile = TileArrangement(Vec::with_capacity(8));

            for p in square.iter_pos() {
                let sig = calc_sig(&p);
                tile.0.push((sig, p));
            }

            tiles.insert(tile_id, tile);
        }

        Self { tiles }
    }

    /// Solve part one.
    fn part1(&self) -> u64 {
        self.corners().0.iter().copied().map(u64::from).product()
    }

    /// Solve part two.
    fn part2(&self) -> usize {
        let n = self.tiles.len().sqrt();
        let mut lochness = LochNess::new(n);

        // find the first corner
        // the both clockwise sigs have be count of 2
        let mut edge1 = 0;
        let mut edg2 = 0;
        let (corners, counts) = self.corners();
        let mut id_top = corners[0];
        let corner = &self.tiles[&id_top];

        for i in 0..4 {
            let sig1 = corner.0[i].0;
            let sig2 = corner.0[(i + 1) % 4].0;

            if counts[&sig1] == 2 && counts[&sig2] == 2 {
                edge1 = sig1;
                edg2 = sig2;
                break;
            }
        }

        // reconstruct the map

        let mut bottom = self.opposite_edge(id_top, edg2);
        let mut right = edge1;
        let mut x = 0;
        loop {
            let mut id = id_top;
            let mut y = 0;

            loop {
                lochness.add_fixed_tile(&self.tiles[&id], bottom, x, y);
                y += 1;

                bottom = self.opposite_edge(id, bottom);
                if let Some(next_id) = self.next_tile(id, bottom) {
                    id = next_id;
                } else {
                    break;
                }
            }

            x += 1;
            if let Some(next_id_top) = self.next_tile(id_top, right) {
                id_top = next_id_top;
                id = id_top;
                bottom = self.adjacent_edge(id, right);
                right = self.opposite_edge(id, right);
            } else {
                break;
            }
        }

        // look for the monster
        let nessie = GridU::<u8>::parse(NESSIE);

        let found: usize = nessie.iter_pos().map(|r| lochness.find_monster(&r)).sum();

        let rough = lochness
            .monster_map
            .iter()
            .filter(|(_, &c)| c == b'#')
            .count();

        let monster_length = nessie.iter().filter(|(_, &c)| c == b'#').count();

        rough - found * monster_length
    }
}

/// # Panics
/// over malformed input
#[must_use]
pub fn solve(data: &str) -> (u64, usize) {
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
    fn part1() {
        let puzzle = Puzzle::new(TEST_INPUT);
        assert_eq!(puzzle.part1(), 20899048083289);
    }

    #[test]
    fn part2() {
        let puzzle = Puzzle::new(TEST_INPUT);
        assert_eq!(puzzle.part2(), 273);
    }
}
