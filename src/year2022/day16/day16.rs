//! [Day 16: Proboscidea Volcanium](https://adventofcode.com/2022/day/16)

use rayon::prelude::*;
use rustc_hash::FxHashMap;

struct Valve {
    rate: u32,
    tunnels: Vec<String>,
}

struct Puzzle {
    flows: Vec<u32>,
    adj: Vec<Vec<u32>>,
    start_dists: Vec<u32>,
    num_important: usize,
}

impl Puzzle {
    fn from_data(data: &str) -> Self {
        // 1. Parse Input
        let mut name_to_id: FxHashMap<&str, usize> = FxHashMap::default();
        let mut valves = Vec::new();

        // Parse raw valves
        for line in data.lines() {
            let (left, right) = line.split_once("; ").unwrap();
            let name = &left[6..8];
            let rate = left[23..].parse::<u32>().unwrap();
            let tunnels = right
                .strip_prefix("tunnels lead to valves ")
                .or_else(|| right.strip_prefix("tunnel leads to valve "))
                .unwrap()
                .split(", ")
                .map(std::string::ToString::to_string)
                .collect();

            name_to_id.insert(name, valves.len());
            valves.push(Valve { rate, tunnels });
        }

        let n = valves.len();

        // 2. Compute full distances (Floyd-Warshall)
        let mut dist = vec![vec![u32::MAX / 2; n]; n];

        for (i, v) in valves.iter().enumerate() {
            dist[i][i] = 0;
            for t in &v.tunnels {
                let j = name_to_id[t.as_str()];
                dist[i][j] = 1;
            }
        }

        for k in 0..n {
            for i in 0..n {
                for j in 0..n {
                    dist[i][j] = dist[i][j].min(dist[i][k] + dist[k][j]);
                }
            }
        }

        // 3. Compress Graph
        let start_node = name_to_id["AA"];

        let mut important_valves = Vec::new();
        for (i, v) in valves.iter().enumerate() {
            if v.rate > 0 {
                important_valves.push(i);
            }
        }

        let num_important = important_valves.len();

        let mut flows = Vec::with_capacity(num_important);
        for &idx in &important_valves {
            flows.push(valves[idx].rate);
        }

        let mut adj = vec![vec![0; num_important]; num_important];
        for i in 0..num_important {
            for j in 0..num_important {
                adj[i][j] = dist[important_valves[i]][important_valves[j]];
            }
        }

        let mut start_dists = Vec::with_capacity(num_important);
        for &idx in &important_valves {
            start_dists.push(dist[start_node][idx]);
        }

        Self {
            flows,
            adj,
            start_dists,
            num_important,
        }
    }

    // 4. Solve function
    fn find_max_flows(&self, total_time: u32) -> Vec<u32> {
        let num_valves = self.num_important;
        let max_mask = 1 << num_valves;

        (0..num_valves)
            .into_par_iter()
            .map(|i| {
                let mut local_max_pressure = vec![0u32; max_mask];
                let d = self.start_dists[i];

                if d + 1 < total_time {
                    // +1 to open
                    let rem_time = total_time - d - 1;
                    let pressure = self.flows[i] * rem_time;
                    let mask = 1 << i;

                    local_max_pressure[mask] = pressure;

                    let mut stack = Vec::with_capacity(64);
                    stack.push((i, rem_time, mask, pressure));

                    while let Some((u, time, mask, pressure)) = stack.pop() {
                        for v in 0..num_valves {
                            if (mask >> v) & 1 == 0 {
                                let d = self.adj[u][v];
                                if d + 1 < time {
                                    let rem_time = time - d - 1;
                                    let new_pressure = pressure + self.flows[v] * rem_time;
                                    let new_mask = mask | (1 << v);

                                    if new_pressure > local_max_pressure[new_mask] {
                                        local_max_pressure[new_mask] = new_pressure;
                                    }

                                    stack.push((v, rem_time, new_mask, new_pressure));
                                }
                            }
                        }
                    }
                }
                local_max_pressure
            })
            .reduce(
                || vec![0u32; max_mask],
                |mut a, b| {
                    for (x, y) in a.iter_mut().zip(b.iter()) {
                        *x = (*x).max(*y);
                    }
                    a
                },
            )
    }

    fn part1(&self) -> u32 {
        let scores_30 = self.find_max_flows(30);
        *scores_30.iter().max().unwrap()
    }

    fn part2(&self) -> u32 {
        // Solve Part 2
        let scores_26 = self.find_max_flows(26);

        // Propagate subset maximums
        let max_mask = 1 << self.num_important;
        let mut best_subset = scores_26;

        for i in 0..self.num_important {
            let bit = 1 << i;
            for mask in 0..max_mask {
                if mask & bit != 0 {
                    best_subset[mask] = best_subset[mask].max(best_subset[mask ^ bit]);
                }
            }
        }

        // Parallelize the final combination check as well
        let all_mask = max_mask - 1;
        (0..max_mask / 2)
            .into_par_iter()
            .map(|mask| {
                let complement = all_mask ^ mask;
                best_subset[mask] + best_subset[complement]
            })
            .max()
            .unwrap_or(0)
    }
}

/// # Panics
pub fn solve(data: &str) -> (u32, u32) {
    let puzzle = Puzzle::from_data(data);
    let part1 = puzzle.part1();
    let part2 = puzzle.part2();
    (part1, part2)
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
        let (p1, _) = solve(TEST_INPUT);
        assert_eq!(p1, 1651);
    }

    #[test]
    fn test_part2() {
        let (_, p2) = solve(TEST_INPUT);
        assert_eq!(p2, 1707);
    }
}
