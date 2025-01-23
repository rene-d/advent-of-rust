//! [Day 4: The Ideal Stocking Stuffer](https://adventofcode.com/2015/day/4)

/// main function
pub fn main() {
    let args = aoc::parse_args();

    args.run(solve);
}

/// # Panics
/// over malformed input
#[must_use]
pub fn solve(data: &str) -> (u32, u32) {
    let mut secret = [0u8; 32];
    let mut base_len = 0;

    // fill a byte buffer with the seed
    for b in data.trim().bytes() {
        secret[base_len] = b;
        base_len += 1;
    }

    let mut number = 0u32;

    let mut find_key = |mask| {
        loop {
            // compute the number of digits of key
            let mut secret_len = base_len;
            let mut number_tmp = number;
            loop {
                secret_len += 1;
                number_tmp /= 10;
                if number_tmp == 0 {
                    break;
                }
            }

            // add the digits to the seed
            let mut number_tmp = number;
            for i in (base_len..secret_len).rev() {
                secret[i] = b'0' + (number_tmp % 10) as u8;
                number_tmp /= 10;
            }

            let digest = md5::compute(&secret[..secret_len]);

            // test the 5 or 6 first nibbles
            if digest.0[0] == 0 && digest.0[1] == 0 && (digest.0[2] & mask) == 0 {
                // println!("{}\t{:?}", key, digest);
                break number;
            }

            number += 1;
        }
    };

    (find_key(0xf0), find_key(0xff))
}
