//! [Day 23: A Long Walk](https://adventofcode.com/2023/day/23)

use rustc_hash::{FxHashMap, FxHashSet};

use aoc::{Coord, Grid};

const FOREST: u8 = b'#';

struct Puzzle {
    grid: Grid<u8>,
}

impl Puzzle {
    /// Parse the puzzle input.
    fn new(data: &str) -> Self {
        Self {
            grid: Grid::<u8>::parse(data),
        }
    }

    /// Solve part one.
    fn part1(&self) -> u32 {
        let start = Coord::new(1, 0);
        let target = Coord::new(self.grid.width() - 2, self.grid.height() - 1);

        let mut visited = FxHashSet::default();
        // Since the path could be long, we might still hit stack limits on large inputs if we just recurse.
        // But for Part 1 with slopes, usually the branching is small and it's fast.
        // Let's see if simple DFS works.
        self.dfs_part1(start, target, &mut visited)
    }

    fn dfs_part1(&self, cur: Coord, target: Coord, visited: &mut FxHashSet<Coord>) -> u32 {
        if cur == target {
            return 0;
        }

        visited.insert(cur);
        let mut max_dist = 0;
        let mut found = false;

        let neighbors = [
            (Coord::new(cur.x + 1, cur.y), b'>'),
            (Coord::new(cur.x.wrapping_sub(1), cur.y), b'<'),
            (Coord::new(cur.x, cur.y + 1), b'v'),
            (Coord::new(cur.x, cur.y.wrapping_sub(1)), b'^'),
        ];

        for (next, _) in neighbors {
            if next.x >= self.grid.width() || next.y >= self.grid.height() {
                continue;
            }
            if visited.contains(&next) {
                continue;
            }
            let tile = self.grid[next];
            if tile == FOREST {
                continue;
            }

            let allowed = match (next.x - cur.x, next.y - cur.y) {
                (1, 0) => tile == b'.' || tile == b'>',
                (-1, 0) => tile == b'.' || tile == b'<',
                (0, 1) => tile == b'.' || tile == b'v',
                (0, -1) => tile == b'.' || tile == b'^',
                _ => false,
            };

            if allowed {
                let d = self.dfs_part1(next, target, visited);
                // If path found (or we are at target, handled by base case? No, if next==target, dfs returns 0)
                // If dfs returns 0, it means it reached target (distance 0 remaining) OR failed?
                // We need to distinguish.
                // Let's change return type to Option<u32> to be safe?
                // Or just use sentinel.
                // If d == 0, it might be target.
                // Valid path length >= 0.
                // Let's check if next == target in loop or handle `Some`.
                // If I change signature, I must update recursive calls.
                // I'll stick to: if next==target, d=0. If result=0 and next!=target?
                // If next != target and dfs returns 0, does it mean failure?
                // My previous code: `if d > 0 || next == target`.
                // This covers it.
                if d > 0 || next == target {
                    max_dist = max_dist.max(1 + d);
                    found = true;
                }
            }
        }

        visited.remove(&cur);

        if found { max_dist } else { 0 }
    }

    /// Solve part two.
    fn part2(&self) -> i32 {
        let start = Coord::new(1, 0);
        let target = Coord::new(self.grid.width() - 2, self.grid.height() - 1);

        // 1. Build the graph of interesting nodes (nodes with > 2 neighbors, + start & end)
        let mut nodes = vec![start, target];
        for (pos, &c) in self.grid.iter_cells() {
            if c == FOREST {
                continue;
            }
            if pos == start || pos == target {
                continue;
            }

            let mut neighbors_count = 0;
            for (_, n) in self.grid.iter_directions(pos) {
                if self.grid[n] != FOREST {
                    neighbors_count += 1;
                }
            }
            if neighbors_count > 2 {
                nodes.push(pos);
            }
        }

        // Map Coord -> Index
        let node_indices: FxHashMap<Coord, usize> =
            nodes.iter().enumerate().map(|(i, &p)| (p, i)).collect();

        // 2. Build adjacency list with distances
        let mut adj = vec![Vec::new(); nodes.len()];

        for (idx, &u) in nodes.iter().enumerate() {
            // BFS/DFS from u to find direct neighbors in the simplified graph
            let mut stack = vec![(u, 0)];
            let mut visited = FxHashSet::default();
            visited.insert(u);

            while let Some((curr, dist)) = stack.pop() {
                if dist > 0 && node_indices.contains_key(&curr) {
                    let v_idx = node_indices[&curr];
                    adj[idx].push((v_idx, dist));
                    continue;
                }

                for (_, next) in self.grid.iter_directions(curr) {
                    if self.grid[next] != FOREST && !visited.contains(&next) {
                        visited.insert(next);
                        stack.push((next, dist + 1));
                    }
                }
            }
        }

        // 3. Longest Path using DFS with Bitmask
        // Start is index 0, Target is index 1
        let target_idx = node_indices[&target];
        Self::dfs_part2(0, target_idx, 0, &adj, 0)
    }

    fn dfs_part2(
        curr: usize,
        target: usize,
        dist: i32,
        adj: &[Vec<(usize, i32)>],
        visited: u64,
    ) -> i32 {
        if curr == target {
            return dist;
        }

        let mut max_dist = 0;
        let new_visited = visited | (1 << curr);

        for &(next, d) in &adj[curr] {
            if (new_visited & (1 << next)) == 0 {
                max_dist = max_dist.max(Self::dfs_part2(next, target, dist + d, adj, new_visited));
            }
        }

        max_dist
    }
}

/// # Panics
#[must_use]
pub fn solve(data: &str) -> (u32, i32) {
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
    fn test01() {
        let puzzle = Puzzle::new(TEST_INPUT);
        assert_eq!(puzzle.part1(), 94);
    }

    #[test]
    fn test02() {
        let puzzle = Puzzle::new(TEST_INPUT);
        assert_eq!(puzzle.part2(), 154);
    }
}
