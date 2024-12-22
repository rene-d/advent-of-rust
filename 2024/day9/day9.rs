//! [Day 9: Disk Fragmenter](https://adventofcode.com/2024/day/9)

const FREE_SPACE: u32 = u32::MAX;

struct Puzzle {
    data: String,
}

impl Puzzle {
    const fn new() -> Self {
        Self {
            data: String::new(),
        }
    }

    /// Get the puzzle input.
    fn configure(&mut self, path: &str) {
        let data = std::fs::read_to_string(path).unwrap();

        self.data = data.trim_ascii().to_string();
    }

    fn load_disk(&self) -> Vec<u32> {
        let mut disk = Vec::new();

        for (i, c) in self.data.chars().enumerate() {
            for _ in 0..c.to_digit(10).unwrap() {
                disk.push(if i % 2 == 1 {
                    FREE_SPACE
                } else {
                    u32::try_from(i).unwrap() / 2
                });
            }
        }

        disk
    }

    fn compute_checksum(disk: &[u32]) -> u64 {
        let mut checksum = 0;

        for (i, c) in disk.iter().enumerate() {
            if c != &FREE_SPACE {
                checksum += u64::from(*c) * (i as u64);
            }
        }

        checksum
    }

    /// Solve part one.
    fn part1(&self) -> u64 {
        let mut disk = self.load_disk();

        let mut i = 0;
        let mut j = disk.len() - 1;
        while i < j {
            if disk[i] == FREE_SPACE {
                while disk[j] == FREE_SPACE && i < j {
                    j -= 1;
                }
                disk.swap(i, j);
                j -= 1;
            }
            i += 1;
        }

        Self::compute_checksum(&disk)
    }

    /// Solve part two.
    fn part2(&self) -> u64 {
        let mut disk = self.load_disk();

        let mut j: usize = disk.len() - 1;
        let mut moved = vec![false; disk.len()];

        'outer: loop {
            // get file size
            while disk[j] == FREE_SPACE {
                if j == 0 {
                    break 'outer;
                }
                j -= 1;
            }

            let mut k = j;
            while disk[k] == disk[j] {
                if k == 0 {
                    break 'outer;
                }
                k -= 1;
            }

            let file_size = j - k;
            let next_j = k;

            if moved[j] {
                j = next_j;
                continue;
            }

            // find free space
            let mut i = 0;
            loop {
                while disk[i] != FREE_SPACE {
                    i += 1;
                }
                k = i;
                while k < disk.len() && disk[k] == FREE_SPACE {
                    k += 1;
                }
                let free_space = k - i;

                if free_space >= file_size && i < j {
                    for _ in 0..file_size {
                        disk.swap(i, j);
                        moved[i] = true;
                        i += 1;
                        j -= 1;
                    }
                    break;
                }

                i = k;
                if i >= disk.len() {
                    break;
                }
            }

            j = next_j;
        }

        Self::compute_checksum(&disk)
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
        assert_eq!(puzzle.part1(), 1928);
    }

    #[test]
    fn test02() {
        let mut puzzle = Puzzle::new();
        puzzle.configure("test.txt");
        assert_eq!(puzzle.part2(), 2858);
    }
}
