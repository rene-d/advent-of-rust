//! [Day 2: I Was Told There Would Be No Math](https://adventofcode.com/2015/day/2)

/// # Panics
#[must_use]
pub fn solve(data: &str) -> (u32, u32) {
    let mut total_paper = 0;
    let mut total_ribbon = 0;

    for line in data.lines() {
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

    (total_paper, total_ribbon)
}

pub fn main() {
    let args = aoc::parse_args();
    args.run(solve);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test01() {
        assert_eq!(solve("2x3x4").0, 58);
        assert_eq!((solve("1x1x10").0), 43);
    }

    #[test]
    fn test02() {
        assert_eq!(solve("2x3x4").1, 34);
        assert_eq!((solve("1x1x10").1), 14);
    }
}
