//! [Day 4: The Ideal Stocking Stuffer](https://adventofcode.com/2015/day/4)

use rayon::prelude::*;

pub fn main() {
    let args = aoc::parse_args();

    args.run(solve);
}

/// # Panics
#[must_use]
pub fn solve(data: &str) -> (u32, u32) {
    let mut secret = [0u8; 32];
    let mut base_len = 0;

    // fill a byte buffer with the seed
    for b in data.trim().bytes() {
        secret[base_len] = b;
        base_len += 1;
    }

    let mut start = 0u32;
    let mut secret_len = base_len + 1;
    let mut next_dozen = 10;

    let mut find_key = |mask| {
        loop {
            // Use find_first to stop as soon as we find a match and return it
            let found = (start..next_dozen).into_par_iter().find_first(|&number| {
                // Int To String directly into buffer
                let mut local_secret = secret;
                let mut number_tmp = number;
                for i in (base_len..secret_len).rev() {
                    local_secret[i] = b'0' + (number_tmp % 10) as u8;
                    number_tmp /= 10;
                }

                let digest = md5::compute(&local_secret[..secret_len]);

                digest.0[0] == 0 && digest.0[1] == 0 && (digest.0[2] & mask) == 0
            });

            if let Some(number) = found {
                return number;
            }

            if next_dozen == 1_000_000_000 {
                // Determine overflow behavior or loop termination if strict u32
                // But AoC inputs fit in u32 typically for this problem
            }

            start = next_dozen;
            next_dozen *= 10;
            secret_len += 1;
        }
    };

    let p1 = find_key(0xf0);
    let p2 = find_key(0xff);
    (p1, p2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_solve() {
        let input = "abcdef";
        let result = solve(input);
        assert_eq!(result.0, 609043);
        // Part 2 is known from previous run: 6742839
        assert_eq!(result.1, 6742839);
    }
}
