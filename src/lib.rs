use std::iter::empty;

/// Get the array of all available solutions.
#[must_use]
pub fn solutions() -> Vec<Solution> {
    empty()
        .chain(year2015())
        .chain(year2016())
        .chain(year2017())
        .chain(year2018())
        .chain(year2019())
        .chain(year2020())
        .chain(year2021())
        .chain(year2022())
        .chain(year2024())
        .collect()
}

/// A solution for given year and day.
/// Offer two callbacks:
///  - `solve` that takes the puzzle input and returns part one and two
///  - `main` that acts like a standalone program for the given day
#[derive(Clone)]
pub struct Solution {
    pub year: u16,
    pub day: u8,
    pub solve: fn(&str) -> (String, String),
    pub main: fn() -> (),
}

macro_rules! make_year {
    ($year:tt $($day:tt),*) => {
        mod $year {
            $(pub mod $day { pub mod $day; })*
        }

        #[must_use]
        pub fn $year() -> Vec<Solution> {
            vec![$({
                let year = stringify!($year)[4..].parse().unwrap();
                let day = stringify!($day)[3..].parse().unwrap();

                let solve = |data: &str| {
                    use crate::$year::$day::$day::solve;
                    let (part1, part2) = solve(data);
                    (part1.to_string(), part2.to_string())
                };

                let main = || {
                    use crate::$year::$day::$day::main;
                    main();
                };

                Solution { year, day, solve, main }
            },)*]
        }
    }
}

make_year!(year2015
    day1,day2,day3,day4,day5,day6,day7,day8,day9,day10,day11,day12,day13,
    day14,day15,day16,day17,day18,day19,day20,day21,day22,day23,day24,day25
);

make_year!(year2016
    day1,day2,day3,day4,day5,day6,day7,day8,day9,day10,day11,day12,day13,
    day14,day15,day16,day17,day18,day19,day20,day21,day22,day23,day24,day25
);

make_year!(year2017
    day1,day2,day3,day4,day5,day6,day7,day8,day9,day10,day11,day12,day13,
    day14,day15,day16,day17,day18,day19,day20,day21,day22,day23,day24,day25
);

make_year!(year2018
    day1,day2,day3,day4,day5,day6,day7,day8,day9,day10,day11,day12,day13,
    day14,day15,day16,day17,day18,day19,day20,day21,day22,day23,day24,day25
);

make_year!(year2019
    day1,day2,day3,day4,day5,day6,day7,day8,day9,day10,day11,day12,day13,
    day14,day15,day16,day17,day18,day19,day20,day21,day22,day23,day24,day25
);

make_year!(year2020
    day1,day2,day3,day4,day5,day6,day7,day8,day9,day10,day11,day12,day13,
    day14,day15,day16,day17,day18,day19,day20,day21,day22,day23,day24,day25
);

make_year!(year2021
    day1,day2,day3,day4,day5,day6,day7,day8,day9,day10,day11,day12,day13,
    day14,day15,day16,day17,day18,day19,day20,day21,day22,day23,day24,day25
);

make_year!(year2022
    day1,day2,day3,day4,day5,day6,day7,day8,day9,day10,day11,day12,day13,
    day14,day15,day16,day17,day18,day19,day20,day21,day22,day23,day24,day25
);

make_year!(year2023
    day1,day2,day3,day4,day5,day6,day7,day8,day9,day10,day11,day12,day13,
    day14,day15,day16,day17,day18,day19,day20,day21,day22,day23,day24,day25
);

make_year!(year2024
    day1,day2,day3,day4,day5,day6,day7,day8,day9,day10,day11,day12,day13,
    day14,day15,day16,day17,day18,day19,day20,day21,day22,day23,day24,day25
);
