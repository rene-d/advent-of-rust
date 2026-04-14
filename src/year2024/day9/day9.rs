//! [Day 9: Disk Fragmenter](https://adventofcode.com/2024/day/9)

use std::cmp::Reverse;
use std::collections::BinaryHeap;

struct Puzzle<'a> {
    data: &'a str,
}

impl<'a> Puzzle<'a> {
    const fn new(data: &'a str) -> Self {
        Self {
            data: data.trim_ascii(),
        }
    }

    /// Solve part one.
    fn part1(&self) -> u64 {
        let bytes = self.data.as_bytes();
        let n = bytes.len();
        if n == 0 {
            return 0;
        }

        let mut checksum: u64 = 0;
        let mut pos: u64 = 0;
        let mut left: usize = 0;
        let mut right: usize = if n.is_multiple_of(2) { n - 2 } else { n - 1 };
        let mut remaining_right = u64::from(bytes[right] - b'0');

        while left < right {
            if left.is_multiple_of(2) {
                let size = u64::from(bytes[left] - b'0');
                let id = (left / 2) as u64;
                checksum += id * (size * pos + size * (size - 1) / 2);
                pos += size;
            } else {
                let mut free = u64::from(bytes[left] - b'0');
                while free > 0 {
                    if remaining_right == 0 {
                        if right < 2 {
                            break;
                        }
                        right -= 2;
                        if right <= left {
                            break;
                        }
                        remaining_right = u64::from(bytes[right] - b'0');
                        continue;
                    }
                    let take = free.min(remaining_right);
                    let rid = (right / 2) as u64;
                    checksum += rid * (take * pos + take * (take - 1) / 2);
                    pos += take;
                    free -= take;
                    remaining_right -= take;
                }
            }
            left += 1;
        }

        if left == right && remaining_right > 0 {
            let rid = (right / 2) as u64;
            checksum += rid * (remaining_right * pos + remaining_right * (remaining_right - 1) / 2);
        }

        checksum
    }

    /// Solve part two.
    fn part2(&self) -> u64 {
        let bytes = self.data.as_bytes();
        let n_files = bytes.len().div_ceil(2);

        let mut file_sizes: Vec<u8> = Vec::with_capacity(n_files);
        let mut file_positions: Vec<u64> = Vec::with_capacity(n_files);
        let mut free_heaps: [BinaryHeap<Reverse<u64>>; 10] =
            std::array::from_fn(|_| BinaryHeap::new());

        let mut pos: u64 = 0;
        for (i, &b) in bytes.iter().enumerate() {
            let size = b - b'0';
            if i.is_multiple_of(2) {
                file_sizes.push(size);
                file_positions.push(pos);
            } else if size > 0 {
                free_heaps[usize::from(size)].push(Reverse(pos));
            }
            pos += u64::from(size);
        }

        let mut checksum: u64 = 0;

        for id in (0..n_files).rev() {
            let size = usize::from(file_sizes[id]);
            if size == 0 {
                continue;
            }
            let orig_pos = file_positions[id];

            let mut best_size = 0usize;
            let mut best_pos = u64::MAX;
            for (s, heap) in free_heaps.iter().enumerate().skip(size) {
                if let Some(&Reverse(p)) = heap.peek()
                    && p < best_pos
                {
                    best_pos = p;
                    best_size = s;
                }
            }

            let final_pos = if best_pos < orig_pos {
                free_heaps[best_size].pop();
                let remainder = best_size - size;
                if remainder > 0 {
                    free_heaps[remainder].push(Reverse(best_pos + size as u64));
                }
                best_pos
            } else {
                orig_pos
            };

            let s = size as u64;
            checksum += (id as u64) * (s * final_pos + s * (s - 1) / 2);
        }

        checksum
    }
}

/// # Panics
#[must_use]
pub fn solve(data: &str) -> (u64, u64) {
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
        assert_eq!(puzzle.part1(), 1928);
    }

    #[test]
    fn test02() {
        let puzzle = Puzzle::new(TEST_INPUT);
        assert_eq!(puzzle.part2(), 2858);
    }
}
