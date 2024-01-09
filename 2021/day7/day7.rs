//! [Day 7: The Treachery of Whales](https://adventofcode.com/2021/day/7)

/// main function
fn main() {
    let data = aoc::load_input_data_vec(7);

    let positions = data[0]
        .split(',')
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let mut min_sum1 = std::i32::MAX;
    let mut min_sum2 = std::i32::MAX;

    let mm = positions.iter().max().unwrap();
    for pos in 0..*mm {
        let mut sum1 = 0;
        let mut sum2 = 0;
        for q in &positions {
            let n = (q - pos).abs();

            sum1 += n;
            sum2 += n * (n + 1) / 2;
        }
        if sum1 < min_sum1 {
            min_sum1 = sum1;
        }

        if sum2 < min_sum2 {
            min_sum2 = sum2;
        }
    }

    println!("{min_sum1}");
    println!("{min_sum2}");
}
