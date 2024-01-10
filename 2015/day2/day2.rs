//! [Day 2: I Was Told There Would Be No Math](https://adventofcode.com/2015/day/2)

/// main function
fn main() {
    let data = aoc::load_input_data_vec(2);

    let mut total_paper = 0;
    let mut total_ribbon = 0;

    for line in data {
        let mut dimensions = line
            .split('x')
            .map(|s| s.parse::<u32>().unwrap())
            .collect::<Vec<_>>();

        dimensions.sort_unstable(); // trick for rubbon computation (no matter order of l,w,h)

        if dimensions.len() != 3 {
            continue;
        }

        // dimensions (length l, width w, and height h) of each present
        let l = dimensions[0];
        let w = dimensions[1];
        let h = dimensions[2];

        // required wrapping paper for each gift a little easier: find the surface area of the box, which is 2*l*w + 2*w*h + 2*h*l
        let paper = 2 * l * w + 2 * w * h + 2 * h * l;

        // little extra paper for each present: the area of the smallest side.
        let slack = std::cmp::min(l * w, std::cmp::min(w * h, h * l));

        // total square feet of wrapping paper
        total_paper += paper + slack;

        // The ribbon required to wrap a present is the shortest distance around its sides, or the smallest perimeter of any one face
        let ribbon = 2 * l + 2 * w;

        // the perfect bow is equal to the cubic feet of volume of the present
        let bow = l * w * h;

        total_ribbon += ribbon + bow;
    }

    println!("{total_paper}");
    println!("{total_ribbon}");
}
