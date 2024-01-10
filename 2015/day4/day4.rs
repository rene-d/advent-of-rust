//! [Day 4: The Ideal Stocking Stuffer](https://adventofcode.com/2015/day/4)

/// main function
fn main() {
    let data = aoc::load_input_data(4).trim().to_string();

    let mut key = 0;

    loop {
        let key_str = key.to_string();
        let mut x = data.clone();

        x.push_str(&key_str);
        let digest = md5::compute(x);

        if format!("{digest:x}").starts_with("00000") {
            // println!("{}\t{:?}", key, digest);
            println!("{key}");
            break;
        }

        key += 1;
    }

    loop {
        let key_str = key.to_string();
        let mut x = data.clone();

        x.push_str(&key_str);
        let digest = md5::compute(x);

        if format!("{digest:x}").starts_with("000000") {
            // println!("{}\t{:?}", key, digest);
            println!("{key}");
            break;
        }

        key += 1;
    }
}
